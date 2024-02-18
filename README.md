# EZSP Rust Sample


Interfacing with the EZSP (EmberNet Zigbee Serial Protocol), from Rust.

The aims of the author are:

- learn Rust in a realistic (embedded) project
- provide a means to completely interact with a very limited set<sup>`[1]`</sup> of Zigbee devices
- eventually, be able to run this in a restricted environment (less than RPi, think ESP32)

*<sup>`[1]`: One, to be precise ;)</sup>*

## Requirements

- Sonoff [Zigbee 3.0 USB Plus V2 "E"](https://sonoff.tech/product/gateway-and-sensors/sonoff-zigbee-3-0-usb-dongle-plus-e/) dongle

<!--
- Schneider / Wiser [Zigbee smoke alarm](https://www.se.com/fi/fi/product/CCT599501/wiser-palovaroitin-savun-ja-lämmönnousun-tunnistuksella-valkoinen-230v-paristovarmennettu/) (the Zigbee device)
-->

- Rust environment

   The author uses [Rust Rover EAP](https://www.jetbrains.com/rust/) IDE on Mac - and [Multipass](https://multipass.run) Linux VMs to run the code. 
   
   Serial port is provided by a neighbouring Windows machine, over USB/IP to the Linux VM. This setup means that the Sonoff driver only needs to be installed on the (less safety critical?) Windows computer.
   
   Your preferences may vary. Let the author know if you need more guidance to set up your environment.

>Developed on:
>
>- macOS 14.3
>- + Multipass 1.13.1 (Ubuntu 22.04 LTS)
>  - ..connecting to Sonoff dongle over USB/IP (served on a Windows PC)
> 
> ```
> rustc 1.76.0 (07dca489a 2024-02-04)
> cargo 1.76.0 (c84b36747 2024-01-18)
> ```


## Preparation

In my case, I don't want to plug the Sonoff directly to my Mac (it's said to need adapters; don't want to know). So using:

- USB/IP to
  - ..insert the stick in a Windows 10 computer
  - ..that has the [suitable drivers installed](https://learn.adafruit.com/how-to-install-drivers-for-wch-usb-to-serial-chips-ch9102f-ch9102/windows-driver-installation) (`CH343SER.ZIP`)
  - ..sharing the stick via USB/IP to Mac

This allows me to develop on the "big screen" (for me), while keeping the development machine uncluttered.

<!-- whisper
- [`dorssel/usbipd-win`](https://github.com/dorssel/usbipd-win) on Windows 10 computer to share the stick
-->

Ways to get USB/IP on Mac:

- [VirtualHere USB Client](https://www.virtualhere.com/usb_client_software) (free, but *not open source*)
- Run things under [Multipass](https://multipass.run) sandbox. See [`akauppi/mp`](https://github.com/akauppi/mp) for setting up a VM with `usbip` client.

   ```
   $ {path-to}/mp/prep.sh 
   Creating rust /
	```

   ```
   $ multipass shell rust
   ~$
   ```
   
   ```
   $ export CARGO_TARGET_DIR=.target-mp   # optional, if you also want to build on the host
   ```


## Run it!

```
$ cargo run --example reset
```

Simply checks that a connection with the dongle can be achieved.

```
$ cargo run --example random
0x456a
[...]
```

Writes out random numbers, originating from the dongle's random number generator.

---

*More examples might be in the `examples` folder.*


<!-- tbd. Use it

- If we publish, but that gets way more complicated.
-->

## References

>Note: We're focusing on EmberZNet 7.x and onwards. Any documents that focus on 6.x are omitted, simply for clarity!

- [UG101 - UART-EZSP Gateway Protocol Reference](https://www.silabs.com/documents/public/user-guides/ug101-uart-gateway-protocol-reference.pdf) (PDF 20 pp, Silicon Labs; year unmarked)

   Describes the ASH (Asynchronous Serial Host) protocol, which takes care of
   host <-> NCP (Network Co-Processor) traffic, when a UART is used. This covers:
   
   - starting the communication
   - retries and checksums
   - randomizing data traffic <sub>the author doesn't really think this is needed in our case, but it's compulsory on non-debug-mode dongles, so we do it..</sub>
   
   The actual EZSP frames travel over such ASH pipe.
   

- [UG100 - EZSP Reference Guide](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf) (PDF 148 pp., Silicon Labs; 2023)

   >"[..] up to date with EmberZNet PRO Release 7.4.0"

   The documentation on EZSP (not ASH) frames.
   
   <!-- tbd. more detailed description, once read it -->

- [QSG180: Zigbee EmberZNet Quick-Start Guide for SDK v7.0 and Higher](https://www.silabs.com/documents/public/quick-start-guides/qsg180-zigbee-emberznet-7x-quick-start-guide.pdf) (PDF 34 pp, Silicon Labs; 2021)

   A software document.

   >Note: The `ERF32MG12` mentioned in the document was no longer available for the author (`MG14` is).
   >
   >The author wasn't able to get any examples from Gecko 4.4 up using "Simplicity Studio" - but nonetheless
   >this document is worth noting (and following; hopefully new versions past "Simplicity Studio" 5.8 emerge,
   >since [there's been some changes](...), there.).

- [Getting started with EZSP-UART](https://community.silabs.com/s/article/getting-started-with-ezsp-uart?language=en_US) (web page; Silicon Labs; Jul'21)

   A brief article, defining some background and terminology.

- [AN706 - EZSP-UART Host Interfacing Guide](https://www.silabs.com/documents/public/application-notes/an706-ezsp-uart-host-interfacing-guide.pdf) (PDF 10 pp, Silicon Labs; 2022)

   A hardware focused document on communicating with the NCP (network co-processor) inside the dongle.

- ["Mapping EmberZNet Versions to Zigbee Specification ...](https://community.silabs.com/s/article/Mapping-EmberZNet-and-GSDK-Versions-to?language=en_US) (Knowledge article, Silicon Labs; Jul'23)

   This information isn't really available anywhere else.

   >Ignore the "Zigbee Cluster Library" - it's no longer a thing with EmberZNet 7 (that we focus on).

- [Gecko (4.4) SDK](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.4) (GitHub)

   See [protocol/zigbee/app/ezsp-host](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.4/protocol/zigbee/app/ezsp-host) for sample C code.
