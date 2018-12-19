#[macro_use]
extern crate nix;
#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;

pub mod error;
pub mod kernel;
pub mod mailbox;
pub mod message;
pub mod raspberrypi_firmware;

use std::mem::size_of;

use kernel::rpi_firmware_property;
pub use mailbox::Mailbox;
use raspberrypi_firmware::rpi_firmware_property_tag;

pub use error::Result;

pub fn firmware_revision(mb: &Mailbox) -> Result<u32> {
    use message::firmware_revision::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_FIRMWARE_REVISION,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.firmware_revision) }
}

pub fn get_board_model(mb: &Mailbox) -> Result<u32> {
    use message::board_model::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_BOARD_MODEL,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.board_model) }
}

pub fn get_board_revision(mb: &Mailbox) -> Result<u32> {
    use message::board_revision::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_BOARD_REVISION,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.board_revision) }
}

pub fn get_board_mac_address(mb: &Mailbox) -> Result<u64> {
    use message::board_mac_address::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In(0) };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_BOARD_MAC_ADDRESS,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe {
        Ok((msg.out.v0 as u64) << 8 * 5
            | (msg.out.v1 as u64) << 8 * 4
            | (msg.out.v2 as u64) << 8 * 3
            | (msg.out.v3 as u64) << 8 * 2
            | (msg.out.v4 as u64) << 8 * 1
            | (msg.out.v5 as u64) << 8 * 0)
    }
}

pub fn get_board_serial(mb: &Mailbox) -> Result<u64> {
    use message::board_serial::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_BOARD_SERIAL,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.board_serial) }
}

pub fn get_arm_memory(mb: &Mailbox) -> Result<(u32, u32)> {
    use message::arm_memory::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_ARM_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok((msg.out.base, msg.out.size)) }
}

pub fn get_vc_memory(mb: &Mailbox) -> Result<(u32, u32)> {
    use message::vc_memory::*;
    use rpi_firmware_property_tag::*;

    let mut msg = Message { in_: In };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_VC_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok((msg.out.base, msg.out.size)) }
}
