//! Wrapper of mailbox kernel interface
//!

use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr::{self, NonNull};

use log::*;
use nix::libc::c_int;

use crate::error::{Error, Result};
use crate::mailbox::Mailbox;
use crate::raspberrypi_firmware::rpi_firmware_property_status::*;
use crate::raspberrypi_firmware::rpi_firmware_property_tag::*;
use crate::raspberrypi_firmware::{rpi_firmware_property_tag, rpi_firmware_property_tag_header};

mod ioctl {
    use nix::*;

    /// Derived from
    /// https://github.com/raspberrypi/linux/blob/rpi-4.14.y/drivers/char/broadcom/vcio.c
    const VCIO_IOC_MAGIC: u8 = 100;
    const VCIO_IOC_TYPE_MODE: u8 = 0;

    ioctl_readwrite! {
        /// mailbox_property via ioctl with VCIO_IOC_MAGIC
        mailbox_property, VCIO_IOC_MAGIC, VCIO_IOC_TYPE_MODE, u32
    }
}

fn rpi_firmware_property_list(mb: &Mailbox, data: *mut u8, tag_size: usize) -> Result<c_int> {
    let size: usize = size_of::<u32>() * 2 + tag_size + size_of::<u32>();
    debug!("{}:{}", size, tag_size);

    let mut buf: Vec<u32> = vec![0u32; size / 4];
    // make request
    buf[0] = size as u32;
    buf[1] = RPI_FIRMWARE_STATUS_REQUEST as u32;
    unsafe {
        ptr::copy(data, buf.as_mut_ptr().offset(2) as *mut u8, tag_size);
    }
    buf[size / 4 - 1] = RPI_FIRMWARE_PROPERTY_END as u32;

    // issue request to mailbox
    debug!("buf: {:?}", buf);
    let res = unsafe { ioctl::mailbox_property(mb.as_raw_fd(), buf.as_mut_ptr()) }?;
    debug!("buf: {:?}", buf);

    if buf[1] != RPI_FIRMWARE_STATUS_SUCCESS as u32 {
        return Err(Error::RequestFailed { code: buf[1] });
    }

    // write back to data
    unsafe {
        ptr::copy(buf.as_ptr().offset(2) as *const u8, data, tag_size);
    }
    Ok(res)
}

pub fn rpi_firmware_property(
    mb: &Mailbox,
    tag: rpi_firmware_property_tag,
    tag_data: *mut u8,
    buf_size: usize,
    req_resp_size: usize,
) -> Result<()> {
    if buf_size < req_resp_size {
        return Err(Error::InvalidInput {
            buf_size,
            req_resp_size,
        });
    }

    let data_size = size_of::<rpi_firmware_property_tag_header>() + buf_size;
    debug!("{},{},{}", buf_size, req_resp_size, data_size);

    let mut data = vec![0u8; data_size];
    union U {
        header: NonNull<rpi_firmware_property_tag_header>,
        data: NonNull<u8>,
    }

    let mut u = U {
        data: unsafe { NonNull::new_unchecked(data.as_mut_ptr()) },
    };

    unsafe {
        // initialize request buffer via header
        u.header.as_mut().tag = tag;
        u.header.as_mut().buf_size = buf_size as u32;
        u.header.as_mut().req_resp_size = req_resp_size as u32;

        // follow headers, copy tag_data to u.data buffer
        ptr::copy(
            tag_data,
            u.data
                .as_ptr()
                .add(size_of::<rpi_firmware_property_tag_header>()),
            buf_size,
        );

        // issue request
        debug!("data[..] {} {} {} {}", data[0], data[1], data[2], data[3]);
        rpi_firmware_property_list(mb, u.data.as_ptr(), data_size)?;
        debug!("data[..] {} {} {} {}", data[0], data[1], data[2], data[3]);

        // check response header bit
        let header = u.header.as_mut();
        if (header.req_resp_size & (1u32 << 31)) == 0 {
            return Err(Error::ReqRespSizeBit {
                req_resp_size: header.req_resp_size,
            });
        }
        header.req_resp_size &= !(1u32 << 31); // clear flag
    }

    let header = unsafe { u.header.as_ref() };

    // check response consistency...
    //

    debug!(
        "req_resp_size: {:x},{:x}",
        header.req_resp_size, req_resp_size
    );
    if header.req_resp_size != req_resp_size as u32 {
        info!(
            "Note: req_resp_size seems not to be used in the firmware \
             for now, but we require users to set this to proper value"
        );
        return Err(Error::BufferSizeMismatch {
            req_resp_size: header.req_resp_size as usize,
            think: req_resp_size,
        });
    }

    debug!("buf_size: {}", buf_size);
    if header.req_resp_size > buf_size as u32 {
        return Err(Error::BufferSizeMismatchSupplied {
            req_resp_size: header.req_resp_size as usize,
            supplied: buf_size,
        });
    }

    // write back to tag_data from u.data
    unsafe {
        ptr::copy(
            u.data
                .as_ptr()
                .add(size_of::<rpi_firmware_property_tag_header>()),
            tag_data,
            req_resp_size,
        )
    }

    Ok(())
}
