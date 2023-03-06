extern crate nix;
extern crate rpi_mailbox;

use rpi_mailbox::*;

fn print_addr(mb: &Mailbox, flags: memflag::Flags) -> Result<()> {
    let handle = mailbox_mem_alloc(mb, 4096, 4096, flags)?;

    let busaddr = mailbox_mem_lock(mb, handle).map_err(|err| {
        mailbox_mem_free(mb, handle).ok();
        err
    })?;

    println!("0x{:08x}", busaddr);

    mailbox_mem_unlock(mb, busaddr).map_err(|err| {
        mailbox_mem_free(mb, handle).ok();
        err
    })?;

    mailbox_mem_free(mb, handle).map_err(|err| {
        mailbox_mem_free(mb, handle).ok();
        err
    })?;

    Ok(())
}

fn main() {
    use memflag::Flags;

    let mb = Mailbox::new("/dev/vcio").expect("mailbox");

    print!("NORMAL:           ");
    print_addr(&mb, Flags::MEM_FLAG_NORMAL).expect("MEM_FLAG_NORMAL");
    print!("DIRECT:           ");
    print_addr(&mb, Flags::MEM_FLAG_DIRECT).expect("MEM_FLAG_DIRECT");
    print!("COHERENT:         ");
    print_addr(&mb, Flags::MEM_FLAG_COHERENT).expect("MEM_FLAG_COHERENT");
    print!("L1_NONALLOCATING: ");
    print_addr(&mb, Flags::MEM_FLAG_L1_NONALLOCATING).expect("MEM_FLAG_L1_NONALLOCATING");
}
