//! Kind of memory flags
//!

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const MEM_FLAG_DISCARDABLE = (1 << 0);
        const MEM_FLAG_NORMAL = (0 << 2);
        const MEM_FLAG_DIRECT = (1 << 2);
        const MEM_FLAG_COHERENT = (1 << 3);
        const MEM_FLAG_L1_NONALLOCATING = Self::MEM_FLAG_DIRECT.bits() | Self::MEM_FLAG_COHERENT.bits();
        const MEM_FLAG_ZERO = (1 << 4);
        const MEM_FLAG_NO_INIT = (1 << 5);
        const MEM_FLAG_HINT_PERMALOCK = (1 << 6);
    }
}
