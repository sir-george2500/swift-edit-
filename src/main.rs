/*** includes ***/
use libc::c_int;
use libc::iscntrl;
use libc::{ICRNL, IXON, OPOST};
use std::io::{self, ErrorKind, Read};
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios, TCSAFLUSH};

const CTRL_KEY_MASK: u8 = 0x1f;

fn ctrl_key(c: u8) -> u8 {
    c & CTRL_KEY_MASK
}

/*** data ***/

// Define a global variable to hold the original terminal attributes
static mut ORIG_TERMIOS: Option<Termios> = None;

/*** terminal ***/
// Function to print error message and exit
fn die(s: &str) -> ! {
    eprintln!("Error: {}", s);
    std::process::exit(1);
}

// Function to disable raw mode
fn disable_raw_mode() -> Result<(), io::Error> {
    // Get the original terminal attributes from the global variable
    let orig_termios = unsafe {
        ORIG_TERMIOS.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Original terminal attributes not set")
        })
    }?;
    // Apply the original terminal attributes to the terminal
    if tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &orig_termios).is_err() {
        die("tcsetattr");
    }
    Ok(())
}

// Function to enable raw mode
fn enable_raw_mode() -> Result<(), io::Error> {
    // Get the current terminal attributes and store them in orig_termios
    let orig_termios = Termios::from_fd(io::stdin().as_raw_fd())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get terminal attributes"))?;
    // Save the original terminal attributes to the global variable
    unsafe {
        ORIG_TERMIOS = Some(orig_termios);
    }
    // Register a cleanup function to disable raw mode on program exit
    unsafe {
        libc::atexit(disable_raw_mode_c);
    }
    // Clone the original terminal attributes to modify them for raw mode
    let mut raw = orig_termios.clone();
    // Disable ICRNL and IXON flags
    raw.c_iflag &= !(ICRNL | IXON);
    // Disable OPOST flag
    raw.c_oflag &= !(OPOST);
    // Disable echoing of input characters and other miscellaneous flags
    raw.c_lflag &= !(termios::ECHO | termios::IEXTEN | termios::ICANON | termios::ISIG);
    raw.c_iflag &=
        !(termios::BRKINT | termios::ICRNL | termios::INPCK | termios::ISTRIP | termios::IXON);
    raw.c_cflag |= termios::CS8;
    raw.c_cc[termios::VMIN as usize] = 0;
    raw.c_cc[termios::VTIME as usize] = 1;
    // Apply the modified terminal attributes to enable raw mode
    if tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw).is_err() {
        die("tcsetattr");
    }
    Ok(())
}

// C-style cleanup function to disable raw mode on program exit
extern "C" fn disable_raw_mode_c() {
    if let Err(e) = disable_raw_mode() {
        eprintln!("Error disabling raw mode: {}", e);
    }
}

/** Read Key **/
/*** init ***/

fn editor_read_key() -> Result<char, io::Error> {
    let mut buffer = [0u8; 1];
    loop {
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => {
                return Ok(buffer[0] as char);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                return Err(e);
            }
        }
    }
}

/**input **/

fn editor_process_keypress() {
    match editor_read_key() {
        Ok(c) => {
            if c as u8 == ctrl_key(b'q') {
                // CTRL_KEY('q')
                std::process::exit(0);
            }
        }
        Err(e) => {
            eprintln!("Error reading key: {}", e);
        }
    }
}
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    loop {
        editor_process_keypress();
    }
}
