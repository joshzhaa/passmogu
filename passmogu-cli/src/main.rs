mod error;

use std::io::{self, Read, Write};
use passmogu::secret::Secret;
use crate::error::Error;

const WELCOME_MSG: &str = r#"PassMogu CLI (v0.1.0)

Enter your master password to unlock vault: "#;

// A value larger than any realistic input.
// Would probably take a full 2 mins of typing full speed to exceed.
const MAX_INPUT_LINE_LEN: usize = 1024;

fn main() -> Result<(), Error> {
    unlock_vault()?;
    let mut input_buffer = Secret::zero(MAX_INPUT_LINE_LEN);
    loop {
        // io::stdin().read_line(input_buffer.expose_mut())?;
        read_line(input_buffer.expose_mut())?;
        // let tokens: Vec<&str> = input_buffer.split_whitespace().collect();
        let tokens: Vec<&[u8]> = input_buffer.expose().split(|byte| *byte == b' ').collect();
        if interpret(&tokens)? {
            println!("Quitting, locking vault");
            break;
        }
        input_buffer.zeroize();
    };
    Ok(())
}

fn read_line(buffer: &mut [u8]) -> Result<(), io::Error> {
    let mut i = 0;
    for byte in io::stdin().bytes() {
        buffer[i] = byte?;
        if buffer[i] == b'\n' {
            break;
        }
        i += 1;
    }
    Ok(())
}

/// returns whether main loop should quit
fn interpret(tokens: &Vec<&[u8]>) -> Result<bool, Error> {
    match tokens[0] {
        b"quit" | b"q" => return Ok(true),
        _ => println!("Unrecognized command, enter q[uit] to quit"),
    }
    Ok(false)
}

fn unlock_vault() -> Result<(), Error>{
    print!("{WELCOME_MSG}");
    io::stdout().flush()?;
    Ok(())
}

