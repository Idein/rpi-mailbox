//! Wrapper of mailbox kernel interface
//!

use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;

use nix;
use nix::libc::c_int;

use mailbox::Mailbox;
use raspberrypi_firmware::rpi_firmware_property_status::*;
use raspberrypi_firmware::rpi_firmware_property_tag::*;
use raspberrypi_firmware::{rpi_firmware_property_tag, rpi_firmware_property_tag_header};

mod ioctl {
    /// Derived from
    /// https://github.com/raspberrypi/linux/blob/rpi-4.14.y/drivers/char/broadcom/vcio.c
    const VCIO_IOC_MAGIC: u8 = 100;
    const VCIO_IOC_TYPE_MODE: u8 = 0;

    ioctl! {
        /// mailbox_property via ioctl with VCIO_IOC_MAGIC
        readwrite mailbox_property with VCIO_IOC_MAGIC, VCIO_IOC_TYPE_MODE; * mut u8
    }
}

fn rpi_firmware_property_list(mb: Mailbox, data: *mut u8, tag_size: usize) -> nix::Result<c_int> {
    use self::ioctl;

    let size: usize = size_of::<u32>() * 2 + tag_size + size_of::<u32>();
    debug!("{}:{}", size, tag_size);
    let mut buf: Vec<u32> = vec![0u32; size / 4];
    buf[0] = size as u32;
    buf[1] = RPI_FIRMWARE_STATUS_REQUEST as u32;
    unsafe {
        debug!(
            "ptr: {:?}, offset(2): {:?}",
            buf.as_mut_ptr() as *mut u8,
            buf.as_mut_ptr().offset(2) as *mut u8
        );
        ptr::copy::<u8>(data, buf.as_mut_ptr().offset(2) as *mut u8, tag_size);
    }
    buf[size / 4 - 1] = RPI_FIRMWARE_PROPERTY_END as u32;
    debug!("buf[{}]: ", buf.len());
    buf.iter().for_each(|x| debug!("{},", x));
    debug!("");

    let res = unsafe { ioctl::mailbox_property(mb.as_raw_fd(), buf.as_mut_ptr() as *mut *mut u8) };
    debug!("err: {:?}", res);
    if res.is_err() {
        return res;
    }
    debug!("buf[1]: {}", buf[1]);
    if buf[1] != RPI_FIRMWARE_STATUS_SUCCESS as u32 {
        error!("Request failed: {:08x}", buf[1]);
        return res;
    }

    unsafe {
        ptr::copy::<u8>(buf.as_ptr().offset(2) as *const u8, data, tag_size);
    }
    res
}

pub fn rpi_firmware_property(
    mb: Mailbox,
    tag: rpi_firmware_property_tag,
    tag_data: *mut u8,
    buf_size: usize,
    req_resp_size: usize,
) {
    if buf_size < req_resp_size {
        println!("error: buf_size < req_resp_size");
        return;
    }

    let data_size = size_of::<rpi_firmware_property_tag_header>() + buf_size;
    let mut data = vec![0u32; data_size / 4];
    data[0] = tag as u32;
    data[1] = buf_size as u32;
    data[2] = req_resp_size as u32;
    debug!("{},{},{}", data[1], data[2], data_size);
    debug!(
        "size_of::<rpi_firmware_property_tag_header>: {}",
        size_of::<rpi_firmware_property_tag_header>()
    );
    unsafe {
        ptr::copy(
            tag_data,
            (data.as_mut_ptr() as *mut u8)
                .offset(size_of::<rpi_firmware_property_tag_header>() as isize),
            buf_size,
        )
    }
    debug!("data[..] {} {} {} {}", data[0], data[1], data[2], data[3]);

    let res = rpi_firmware_property_list(mb, data.as_mut_ptr() as *mut u8, data_size);
    if res.is_err() {
        error!("err: {:?}", res);
        return;
    }

    debug!(
        "header->req_resp_size: {}, {:x}",
        data[2],
        (data[2] & (1u32 << 31))
    );
    if (data[2] & (1u32 << 31)) == 0 {
        println!("header->req_resp_size[31] was not set by firmware");
        return;
    }
    data[2] &= !(1u32 << 31);
    debug!("header->req_resp_size: {},{}", data[2], req_resp_size);
    if data[2] != req_resp_size as u32 {
        println!(
            "Firmware requires {} bytes for response buffer while you think it is {} bytes",
            data[2], req_resp_size
        );
        return;
    }
    debug!("buf_size: {}", buf_size);
    if data[2] > buf_size as u32 {
        println!("Firmware requires {} bytes for response buffer while the supplied buffer is only {} bytes",
                data[2], buf_size);
        return;
    }

    unsafe {
        ptr::copy(
            (data.as_ptr() as *mut u8)
                .offset(size_of::<rpi_firmware_property_tag_header>() as isize),
            tag_data,
            req_resp_size,
        )
    }
}
