use std::io::{self, Read};
use libc::ICRNL;
use libc::IXON;
use libc::OPOST;
use termios::{Termios, tcsetattr, TCSAFLUSH};
use std::os::unix::io::AsRawFd;
use libc::c_int;
use libc::iscntrl;
// Define a global variable to hold the original terminal attributes
static mut ORIG_TERMIOS: Option<Termios> = None;

// Function to disable raw mode
fn disable_raw_mode() -> Result<(), io::Error> {
    // Get the original terminal attributes from the global variable
    let orig_termios = unsafe {
        ORIG_TERMIOS.take().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Original terminal attributes not set"))
    }?;
    // Apply the original terminal attributes to the terminal
    tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &orig_termios)?;
    Ok(())
}

// Function to enable raw mode
fn enable_raw_mode() -> Result<(), io::Error> {
    // Get the current terminal attributes and store them in orig_termios
    let orig_termios = Termios::from_fd(io::stdin().as_raw_fd())?;
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
    raw.c_iflag &= !(ICRNL|IXON);

    // Disable OPOST flag
    raw.c_oflag &= !(OPOST);
    // Disable echoing of input characters
    raw.c_lflag &= !(termios::ECHO|termios::IEXTEN | termios::ICANON | termios::ISIG);
    // Apply the modified terminal attributes to enable raw mode
    tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw)?;
    Ok(())
}

// C-style cleanup function to disable raw mode on program exit
extern "C" fn disable_raw_mode_c() {
    if let Err(e) = disable_raw_mode() {
        eprintln!("Error disabling raw mode: {}", e);
    }
}
fn main() -> Result<(), io::Error> {
    let _raw_mode = enable_raw_mode()?;

    let mut input = [0u8];
    loop {
        io::stdin().read_exact(&mut input)?;
        let c = input[0];

        if c == b'q' {
            break;
        }
        unsafe{

            if iscntrl(c as c_int) != 0 {
                println!("{} (CTRL)\r\n", c);
            } else {
                println!("{} ('{}')\r\n", c, c as char);
            }
        }
    }

    Ok(())
}

