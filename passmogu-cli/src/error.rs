use std::io;

#[derive(Debug)]
pub(crate) enum Error {
    IOError,
    UnlockError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::IOError
    }
}

