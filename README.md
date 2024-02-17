# EZSP Rust Sample


Interfacing with the EZSP (EmberNet Zigbee Serial Protocol), from Rust.

The aims of the author are:

- learn Rust in a realistic (embedded) project
- provide a means to completely interact with a very limited set of Zigbee devices
- eventually, be able to run this in a restricted environment (less than RPi, think ESP32)

>Developed on:
>
>- macOS 14.3
>- Multipass for USB/IP client (running the app)
>- Windows 10 for USB/IP host


## Requirements

- Sonoff [Zigbee 3.0 USB Plus V2](https://sonoff.tech/product/gateway-and-sensors/sonoff-zigbee-3-0-usb-dongle-plus-e/) dongle (model "E")

- Schneider / Wiser [Zigbee smoke alarm](https://www.se.com/fi/fi/product/CCT599501/wiser-palovaroitin-savun-ja-lämmönnousun-tunnistuksella-valkoinen-230v-paristovarmennettu/) (FI page)

- Rust environment

   Install with `rustup` (multiple ways): 

   ```
   $ rustc --version
   rustc 1.75.0 (82e1608df 2023-12-21)
   ```
      
   ```
   $ cargo --version
   cargo 1.75.0 (1d8b05cdd 2023-11-20)
   ```

- [Rust Rover EAP](https://www.jetbrains.com/rust/) IDE

### Ubuntu (including Multipass)

```
$ sudo apt install pkg-config libudev-dev
```

These are [`serialport` crate dependencies](https://github.com/serialport/serialport-rs?tab=readme-ov-file#dependencies).

<!--
Developed on:

- macOS 14.3
- + Multipass (Ubuntu 22.04 LTS)
  - ..connecting to Sonoff dongle over USB/IP (served on a Windows PC)
-->


## Preparation

In my case, I don't want to plug the Sonoff directly to my Mac (it's said to need adapters; don't want to know). So using:

- USB/IP to
  - ..insert the stick in a Windows 10 computer
  - ..that has the [suitable drivers installed](https://learn.adafruit.com/how-to-install-drivers-for-wch-usb-to-serial-chips-ch9102f-ch9102/windows-driver-installation) (`CH343SER.ZIP`)
  - ..sharing the stick via USB/IP to Mac

This allows me to develop on the "big screen" (for me), while keeping the development machine uncluttered.

<!-- whisper
- [`dorssel/usbipd-win`](https://github.com/dorssel/usbipd-win) on Windows 10 computer to share the stick
- [`jiegec/usbip`](https://github.com/jiegec/usbip) to tie the stick to the Mac
-->

Ways to get USB/IP on Mac:

- [VirtualHere USB Client](https://www.virtualhere.com/usb_client_software) (free, but *not open source*)
- Run things under [Multipass](https://multipass.run) sandbox. Linux has `usbip`.

   ```
   $ mp/prep.sh 
   Creating rust /
	```

   ```
   $ multipass shell rust
   ~$
   ```


## Steps

```
$ cargo run --example version
```


## References

<!-- minor; hidden?
- ["About Zigbee EZSP UART"](https://www.owon-smart.com/news/about-zigbee-ezsp-uart/) (Owon blog-like; Feb'22)
-->

- ["How to build a EZSP-UART host application"](https://siliconlabs.my.site.com/community/s/article/how-to-build-an-ezsp-uart-host-application?language=en_US) (Silicon labs; Nov'22)

