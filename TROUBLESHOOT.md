# Troubleshoot 🔫

## Cannot `usbip attach` in Multipass

```
$ sudo usbip attach -r 192.168.1.29 -b 3-1
libusbip: error: udev_device_new_from_subsystem_sysname failed
usbip: error: open vhci_driver
```

Time has passed and you need to reapply the [`usbip-drivers.sh`](https://github.com/akauppi/mp/blob/main/rust/linux/usbip-drivers.sh) script from mp repo.

>Most likely this is about the kernel version proceeding, and `usbip` doesn't find the `vhci_driver` for the new version. 

```
$ apt install -y linux-tools-generic linux-modules-extra-$(uname -r)
$ sudo modprobe vhci-hcd
```

Retry.

If it still doesn't work, please raise an [Issue](https://github.com/akauppi/EZSP-RS/issues). 

