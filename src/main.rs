use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use termios::{Termios, tcgetattr, tcsetattr, TCSAFLUSH};

fn enable_raw_mode() -> Result<Termios, io::Error> {
    let mut raw = Termios::from_fd(io::stdin().as_raw_fd())?;
    raw.c_lflag &= !(termios::ECHO);
    tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw)?;
    Ok(raw)
}

fn main() -> Result<(), io::Error> {
    let _raw_mode = enable_raw_mode()?;
    let mut input = [0u8];
    loop {
        io::stdin().read_exact(&mut input)?;
        if input[0] == b'q' {
            break;
        }
    }
    Ok(())
}

