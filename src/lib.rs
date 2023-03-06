//! A RaspberryPi mailbox interface
//!

pub mod error;
mod kernel;
mod mailbox;
pub mod memflag;
mod message;
pub mod raspberrypi_firmware;

use std::mem::size_of;

use kernel::rpi_firmware_property;
pub use mailbox::Mailbox;
use raspberrypi_firmware::rpi_firmware_property_tag::*;

pub use error::Result;

pub fn firmware_revision(mb: &Mailbox) -> Result<u32> {
    use message::firmware_revision::*;

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

    let mut msg = Message { in_: In(0) };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_BOARD_MAC_ADDRESS,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe {
        Ok((msg.out.v0 as u64) << (8 * 5)
            | (msg.out.v1 as u64) << (8 * 4)
            | (msg.out.v2 as u64) << (8 * 3)
            | (msg.out.v3 as u64) << (8 * 2)
            | (msg.out.v4 as u64) << 8
            | (msg.out.v5 as u64))
    }
}

pub fn get_board_serial(mb: &Mailbox) -> Result<u64> {
    use message::board_serial::*;

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

pub fn mailbox_mem_alloc(
    mb: &Mailbox,
    size: u32,
    align: u32,
    flags: memflag::Flags,
) -> Result<u32> {
    use message::allocate_memory::*;

    let mut msg = Message {
        in_: In {
            size,
            align,
            flags: flags.bits(),
        },
    };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_ALLOCATE_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<In>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.handle) }
}

pub fn mailbox_mem_free(mb: &Mailbox, handle: u32) -> Result<u32> {
    use message::release_memory::*;

    let mut msg = Message { in_: In { handle } };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_RELEASE_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<In>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.status) }
}

pub fn mailbox_mem_lock(mb: &Mailbox, handle: u32) -> Result<u32> {
    use message::lock_memory::*;

    let mut msg = Message { in_: In { handle } };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_LOCK_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<In>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.busaddr) }
}

pub fn mailbox_mem_unlock(mb: &Mailbox, busaddr: u32) -> Result<u32> {
    use message::unlock_memory::*;

    let mut msg = Message {
        in_: In { busaddr },
    };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_UNLOCK_MEMORY,
        &mut msg as *mut Message as *mut u8,
        size_of::<In>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.status) }
}

pub fn get_throttled(mb: &Mailbox) -> Result<u32> {
    use message::throttled::*;

    let mut msg = Message {
        in_: In { mask: 0 },
    };
    rpi_firmware_property(
        mb,
        RPI_FIRMWARE_GET_THROTTLED,
        &mut msg as *mut Message as *mut u8,
        size_of::<Message>(),
        size_of::<Out>(),
    )?;
    unsafe { Ok(msg.out.throttled) }
}
