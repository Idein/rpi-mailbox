use nix;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("nix error")]
    Nix(#[from] nix::Error),
    #[error("request failed: {}", code)]
    RequestFailed { code: u32 },
    #[error("buf_size < req_resp_size: {} < {}", buf_size, req_resp_size)]
    InvalidInput {
        buf_size: usize,
        req_resp_size: usize,
    },
    #[error("req_resp_size[31] was not set by firmware: {}", req_resp_size)]
    ReqRespSizeBit { req_resp_size: u32 },
    #[error("buffer size mismatch {} != {}", req_resp_size, think)]
    BufferSizeMismatch { req_resp_size: usize, think: usize },
    #[error("buffer size mismatch (supplied) {} != {}", req_resp_size, supplied)]
    BufferSizeMismatchSupplied {
        req_resp_size: usize,
        supplied: usize,
    },
}
