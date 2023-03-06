extern crate chrono;
extern crate nix;
extern crate rpi_mailbox;

use chrono::prelude::*;
use rpi_mailbox::*;

fn main() {
    let mb = Mailbox::new("/dev/vcio").expect("mailbox");

    let rev = firmware_revision(&mb).expect("firmware_revision");
    let date = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(rev as i64, 0).unwrap(),
        Utc,
    );
    println!("Firmware revision: {}", date.format("%b %e %Y %T"));

    let model = get_board_model(&mb).expect("board_model");
    println!("Board model: 0x{:08x}", model);

    let rev = get_board_revision(&mb).expect("board_revision");
    println!("Board revision: 0x{:08x}", rev);

    let mac = get_board_mac_address(&mb).expect("board_mac_address");
    println!("Board MAC address: {:012x}", mac);

    let serial = get_board_serial(&mb).expect("board_serial");
    println!("Board serial: 0x{:x}", serial);

    let (base, size) = get_arm_memory(&mb).expect("arm_memory");
    println!("ARM memory: 0x{:08x} bytes at 0x{:08x}", size, base);

    let (base, size) = get_vc_memory(&mb).expect("vc_memory");
    println!("VC memory:  0x{:08x} bytes at 0x{:08x}", size, base);

    let throttled = get_throttled(&mb).expect("throttled");
    println!("Throttled: 0x{:x}", throttled);
}
