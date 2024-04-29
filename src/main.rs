use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut c = [0; 1]; // Define a buffer to read a single byte at a time
    loop {
        match io::stdin().read_exact(&mut c) {
            Ok(()) => continue, // Continue looping if exactly 1 byte is read successfully
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break, // Break the loop if EOF is reached
            Err(e) => return Err(e), // Propagate any other errors
        }
    }
    Ok(())
}

