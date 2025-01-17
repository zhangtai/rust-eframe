## USB IT8951 epaper driver Rust

## New instructions

### System

```shell
sudo sh -c "echo 'SUBSYSTEM==\"usb\", ATTRS{idVendor}==\"048d\", MODE=\"0666\"' > /etc/udev/rules.d/60-it8951.rules"
sudo udevadm control --reload-rules && sudo udevadm trigger
```

### Build

```shell
cargo build --release
sudo cp target/release/eframe /usr/local/bin
```

### Usage

```shell
eframe kitten.jpg fill
```

### Take screenshot from web

```shell
npm config set prefix '~/.local/'
npm install --global pageres-cli
pageres https://worldcup.cole.ws/ --delay=15 624x800 --scale=3 --selector='#groups'
```

### Troubleshooting

#### Cannot open it8951

I have this issue on Fedora 38 Server, I ran `dmesg` as per [this guide](https://docs.fedoraproject.org/en-US/fedora/f28/install-guide/install/Troubleshooting/), remove USB, and plug back. It works, don't know the root cause.


## Originals

Waveshare sells a [range of epaper
displays](https://www.waveshare.com/product/displays/e-paper/epaper-1.htm),
some of which ship with a IT8951 display HAT. This IT8951 can be controlled via
SPI (typically through a Raspberry Pi) but also through USB, which I was
interested in so I could control it with my Linux desktop.

Thankfully, a large part of the problem had already been solved by [Martijn
Braam in C](https://blog.brixit.nl/epaper/), and his solution inspired mine.
One drawback of Martijn Braam's solution is that it requires root access as it
uses a specific Linux ioctl command to send data to the epaper display.

Martijn Braam links to a [IT8951 USB Programming
Guide](https://www.waveshare.com/w/upload/c/c9/IT8951_USB_ProgrammingGuide_v.0.4_20161114.pdf)
which gave me the clue that it should be possible to do this directly over USB
without root access. After quite some digging I've implemented a working
solution in Rust.

## Preparation

In order to make this work you need to create a udev rule that gives users
permission to talk to this device. To this end add a file `60-it8951.rules`
to the `/etc/udev/rules.d/etc/udev/rules.d` directory with the following
contents:

```
SUBSYSTEM=="usb", ATTRS{idVendor}=="048d", MODE="0666"
```

This gives applications access to talk to devices by vendor "048d", which is
the IT8951. You can then restart your system, or by write this to trigger
without reboot:

```
udevadm control --reload-rules && udevadm trigger
```

## Custom SCSI commands over SCSI over USB

The IT8951 implements custom SCSI commands to control the epaper display.
Normally SCSI is a disk protocol, but with these custom commands it can be
extended to arbitrary new commands. It's possible to send SCSI commands over
USB: you need to wrap them in a Command Block Wrapper and Command Status
Wrapper. Unlike the ioctl which can be used to talk to SCSI directly, USB may
be controlled by users, which is how this code works without requiring root
access.
