use std::ops::Drop;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

use nix::fcntl;
use nix::sys::stat;
use nix::unistd;
use nix::NixPath;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mailbox(RawFd);

impl Mailbox {
    /// open device
    ///
    /// device: path to mailbox device. e.g. /dev/vcio
    pub fn new<P>(device: &P) -> Result<Self, Error>
    where
        P: ?Sized + NixPath,
    {
        let fd = fcntl::open(device, fcntl::OFlag::O_NONBLOCK, stat::Mode::empty())?;
        Ok(Mailbox(fd))
    }
}

impl Drop for Mailbox {
    fn drop(&mut self) {
        unistd::close(self.0).expect("Mailbox drop")
    }
}

impl FromRawFd for Mailbox {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Mailbox(fd)
    }
}

impl AsRawFd for Mailbox {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl IntoRawFd for Mailbox {
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}
