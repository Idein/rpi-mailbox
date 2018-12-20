//! Message type
//!
//! For accessing mailbox_property requires a pointer to flat u8 buffer.
//! But properties using different data structures for each.
//! This module provides message structures for each property tags.
//!

/// RPI_FIRMWARE_GET_FIRMWARE_REVISION
pub mod firmware_revision {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub firmware_revision: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_BOARD_MODEL
pub mod board_model {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub board_model: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_BOARD_REVISION
pub mod board_revision {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub board_revision: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_BOARD_MAC_ADDRESS
pub mod board_mac_address {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In(pub u64);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub v0: u8,
        pub v1: u8,
        pub v2: u8,
        pub v3: u8,
        pub v4: u8,
        pub v5: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_BOARD_SERIAL
pub mod board_serial {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub board_serial: u64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_ARM_MEMORY
pub mod arm_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub base: u32,
        pub size: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_VC_MEMORY
pub mod vc_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub base: u32,
        pub size: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_ALLOCATE_MEMORY
pub mod allocate_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In {
        pub size: u32,
        pub align: u32,
        pub flags: u32,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub handle: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_LOCK_MEMORY
pub mod lock_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In {
        pub handle: u32,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub busaddr: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_UNLOCK_MEMORY
pub mod unlock_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In {
        pub busaddr: u32,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub status: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_RELEASE_MEMORY
pub mod release_memory {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In {
        pub handle: u32,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub status: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}

/// RPI_FIRMWARE_GET_THROTTLED
pub mod throttled {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct In {
        pub mask: u16,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Out {
        pub throttled: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Message {
        pub in_: In,
        pub out: Out,
    }
}
