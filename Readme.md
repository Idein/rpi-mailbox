
# RaspberrypiMailbox

An rust interface of the `mailbox` a low level feature of the RaspberryPi single board computer.
Mailbox interface is a communication channnel between the ARM and the VideoCore firmware.


## Acknowledgment

This project use [Terminus-IMRC/mailbox](https://github.com/Terminus-IMRC/mailbox) as a reference and a implementation.


## Build

Build for the target `arm-unknown-linux-gnueabihf` like below:

```console
$ cargo build --target=arm-unknown-linux-gnueabihf
```


## Link

- [firmware/wiki](https://github.com/raspberrypi/firmware/wiki)
- [Terminus-IMRC/mailbox](https://github.com/Terminus-IMRC/mailbox)

