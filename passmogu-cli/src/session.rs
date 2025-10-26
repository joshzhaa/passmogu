use crate::error::Error;
use passmogu::{encrypt::derive_key, secret::Secret};
use std::io::{self, Read, Write};

const WELCOME_MSG: &str = "Enter your master password to unlock vault: ";

// A value larger than any realistic input.
// Would probably take a full 2 mins of typing full speed to exceed.
const MAX_INPUT_LINE_LEN: usize = 1024;

pub(crate) fn session_repl() -> Result<(), Error> {
    unlock_vault()?;
    let mut input_buffer = Secret::zero(MAX_INPUT_LINE_LEN);
    loop {
        // io::stdin().read_line(input_buffer.expose_mut())?;
        read_line(input_buffer.expose_mut())?;
        // let tokens: Vec<&str> = input_buffer.split_whitespace().collect();
        let tokens: Vec<&[u8]> = tokenize(input_buffer.expose());
        if interpret(&tokens)? {
            println!("Quitting, locking vault");
            break;
        }
        input_buffer.zeroize();
    }
    Ok(())
}

fn read_line(buffer: &mut [u8]) -> Result<(), Error> {
    // unbuffered bytes here to avoid littering a buffer with a secret in it
    // TODO: revise if performance becomes an issue
    #[expect(clippy::unbuffered_bytes)]
    for (i, byte) in io::stdin().bytes().enumerate() {
        buffer[i] = byte?;
        if buffer[i] == b'\n' {
            buffer[i] = b' '; // allows .split to truncate nulls from end
            break;
        }
    }
    Ok(())
}

fn tokenize(buffer: &[u8]) -> Vec<&[u8]> {
    buffer.split(|byte| *byte == b' ').collect()
}

/// Returns whether main loop should quit
fn interpret(tokens: &[&[u8]]) -> Result<bool, Error> {
    match tokens[0] {
        b"quit" | b"q" => return Ok(true),
        _ => println!("Unrecognized command, enter q[uit] to quit"),
    }
    Ok(false)
}

/// Returns master key.
fn unlock_vault() -> Result<Secret, Error> {
    print!("{WELCOME_MSG}");
    io::stdout().flush()?;
    let mut master_password = Secret::zero(MAX_INPUT_LINE_LEN);
    read_line(master_password.expose_mut())?;
    Ok(derive_key(
        master_password.expose(),
        b"TODO: dummy salt string",
    ))
}
