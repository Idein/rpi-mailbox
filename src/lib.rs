#[macro_use]
extern crate nix;
#[macro_use]
extern crate log;

pub mod kernel;
pub mod mailbox;
pub mod message;
pub mod raspberrypi_firmware;

pub use mailbox::Mailbox;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
