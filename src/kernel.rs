//! Wrapper of mailbox kernel interface
//!

use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;

use nix::libc::c_int;

use mailbox::Mailbox;
use raspberrypi_firmware::rpi_firmware_property_status::*;
use raspberrypi_firmware::rpi_firmware_property_tag::*;
use raspberrypi_firmware::{rpi_firmware_property_tag, rpi_firmware_property_tag_header};

use error::{ErrorKind, Result};

mod ioctl {
    /// Derived from
    /// https://github.com/raspberrypi/linux/blob/rpi-4.14.y/drivers/char/broadcom/vcio.c
    const VCIO_IOC_MAGIC: u8 = 100;
    const VCIO_IOC_TYPE_MODE: u8 = 0;

    ioctl! {
        /// mailbox_property via ioctl with VCIO_IOC_MAGIC
        readwrite mailbox_property with VCIO_IOC_MAGIC, VCIO_IOC_TYPE_MODE; u32
    }
}

fn rpi_firmware_property_list(mb: &Mailbox, data: *mut u8, tag_size: usize) -> Result<c_int> {
    use self::ioctl;

    let size: usize = size_of::<u32>() * 2 + tag_size + size_of::<u32>();
    debug!("{}:{}", size, tag_size);
    let mut buf: Vec<u32> = vec![0u32; size / 4];
    buf[0] = size as u32;
    buf[1] = RPI_FIRMWARE_STATUS_REQUEST as u32;
    unsafe {
        ptr::copy(data, buf.as_mut_ptr().offset(2) as *mut u8, tag_size);
    }
    buf[size / 4 - 1] = RPI_FIRMWARE_PROPERTY_END as u32;
    debug!("buf[{}]: ", buf.len());
    buf.iter().for_each(|x| debug!("{},", x));
    debug!("");

    let res = unsafe { ioctl::mailbox_property(mb.as_raw_fd(), buf.as_mut_ptr()) }?;

    debug!("buf[1]: {}", buf[1]);
    if buf[1] != RPI_FIRMWARE_STATUS_SUCCESS as u32 {
        return Err(ErrorKind::RequestFailed(buf[1]).into());
    }

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
        return Err(ErrorKind::InvalidInput(buf_size, req_resp_size).into());
    }

    let data_size = size_of::<rpi_firmware_property_tag_header>() + buf_size;
    let mut data = vec![0u32; data_size / 4];
    data[0] = tag as u32;
    data[1] = buf_size as u32;
    data[2] = req_resp_size as u32;
    debug!("{},{},{}", data[1], data[2], data_size);

    unsafe {
        ptr::copy(
            tag_data,
            (data.as_mut_ptr() as *mut u8)
                .offset(size_of::<rpi_firmware_property_tag_header>() as isize),
            buf_size,
        )
    }
    debug!("data[..] {} {} {} {}", data[0], data[1], data[2], data[3]);

    rpi_firmware_property_list(mb, data.as_mut_ptr() as *mut u8, data_size)?;

    debug!("req_resp_size: {:x}", data[2]);

    if (data[2] & (1u32 << 31)) == 0 {
        return Err(ErrorKind::ReqRespSizeBit(data[2]).into());
    }
    data[2] &= !(1u32 << 31);

    debug!("req_resp_size: {:x},{:x}", data[2], req_resp_size);
    if data[2] != req_resp_size as u32 {
        info!(
            "Note: req_resp_size seems not to be used in the firmware \
             for now, but we require users to set this to proper value"
        );
        return Err(ErrorKind::BufferSizeMismatchYouThink(data[2] as usize, req_resp_size).into());
    }

    debug!("buf_size: {}", buf_size);
    if data[2] > buf_size as u32 {
        return Err(ErrorKind::BufferSizeMismatchSupplied(data[2] as usize, buf_size).into());
    }

    unsafe {
        ptr::copy(
            (data.as_ptr() as *mut u8)
                .offset(size_of::<rpi_firmware_property_tag_header>() as isize),
            tag_data,
            req_resp_size,
        )
    }

    Ok(())
}
