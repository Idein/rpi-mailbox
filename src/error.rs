use nix;

error_chain! {
    foreign_links {
        Nix(nix::Error);
    }
    errors {
        RequestFailed(code: u32) {
            description("Request failed")
                display("Request failed: {}", code)
        }
        InvalidInput(buf_size: usize, req_resp_size: usize) {
            description("buf_size < req_resp_size")
                display("buf_size < req_resp_size: {} < {}", buf_size, req_resp_size)
        }
        ReqRespSizeBit(req_resp_size: u32) {
            description("req_resp_size[31] was not set by firmware")
                display("req_resp_size[31] was not set by firmware: {}", req_resp_size)
        }
        BufferSizeMismatchYouThink(req_resp_size: usize, think: usize) {
            description("Required bytes buffer sizes for response from firmware different with you think it")
                display("Firmware requires {} bytes for response buffer while you think it is {} bytes", req_resp_size, think)
        }
        BufferSizeMismatchSupplied(req_resp_size: usize, supplied: usize) {
            description("Required bytes buffer size for response from firmware different with supplied buffer size")
                display("Firmware requires {} bytes for response buffer while the supplied buffer is only {} bytes", req_resp_size, supplied)
        }
    }
}
