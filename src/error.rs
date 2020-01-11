use std::fmt;
use nix;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug, Clone)]
pub enum ErrorKind {
    #[fail(display = "nix error")]
    Nix,
    #[fail(display = "request failed: {}", code)]
    RequestFailed { code: u32 },
    #[fail(display = "buf_size < req_resp_size: {} < {}", buf_size, req_resp_size)]
    InvalidInput {
        buf_size: usize,
        req_resp_size: usize,
    },
    #[fail(display = "req_resp_size[31] was not set by firmware: {}", req_resp_size)]
    ReqRespSizeBit { req_resp_size: u32 },
    #[fail(display = "buffer size mismatch {} != {}", req_resp_size, think)]
    BufferSizeMismatch { req_resp_size: usize, think: usize },
    #[fail(display = "buffer size mismatch (supplied) {} != {}", req_resp_size, supplied)]
    BufferSizeMismatchSupplied {
        req_resp_size: usize,
        supplied: usize,
    },
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<nix::Error> for Error {
    fn from(error: nix::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::Nix),
        }
    }
}
