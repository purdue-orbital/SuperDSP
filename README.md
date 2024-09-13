[![MIT](https://img.shields.io/github/license/purdue-orbital/SuperDSP)](../master/LICENSE)
[![Tests](https://img.shields.io/github/actions/workflow/status/purdue-orbital/SuperDSP/test.yml?label=Tests)](https://github.com/purdue-orbital/SuperDSP/actions)

# SuperDSP (WIP)https://github.com/purdue-orbital/SuperDSP/actions

This library is a very work in progress, and the interface currently is subject to changes.

## Table of Contents

- [Mission](#mission)
- [Feature Flags](#feature-flags)
- [Pre-requisites](#pre-requisites)
    - [No-std](#No-Std)
    - [Std](#std)
    - [BladeRF](#bladerf)
        - [Linux](#linux)
            - [Ubuntu](#ubuntu)
            - [Nix-Shell (NixOS)](#nix-shell-nixos)
            - [Building from source](#building-from-source)
        - [Windows](#windows)
        - [MacOS](#macos)
    - [GUI](#gui)
        - [Linux](#linux-1)
            - [Ubuntu](#ubuntu-1)
            - [Fedora](#fedora)
            - [Nix-Shell (NixOS) (Wayland)](#nix-shell-nixos-wayland)
            - [Nix-Shell (NixOS) (X11)](#nix-shell-nixos-x11)
        - [Windows](#windows-1)
        - [MacOS](#macos-1)
- [Goals](#goals)

## Mission

This library is not trying to replace existing radio-prototyping tools or amateur radio software like
[GNURadio-Companion](https://www.gnuradio.org/) or [SDRAngel](https://rgetz.github.io/sdrangel/). Rather, this project
aims to provide Rust tools and simplified versions of complex operations that enables these systems to be built. It
also aims to be used in production code and act as a drop-in replacement of a prototype built in
GNURadio-Companion. Although not the intended use, this library can be used in other applications like Ground,
Navigation, and Control (GNC) applications or sound design.

## Feature Flags

To use this library without the standard library, disable default features. This library **REQUIRES** the ``alloc``
crate.
Even in no_std mode, the ``alloc`` crate is used to allocate memory for buffers and other objects.

- ``std``: Enabled by default. Disables the ``no_std`` feature and enables the standard library.
- ``gui``: Enables GUI features and objects. This flag automatically enables the ``std`` feature.
- ``bladerf``: Enables the BladeRF hardware support. This flag automatically enables the ``std`` feature. Please make
  sure
  you have the BladeRF library (libbladeRF) installed on your system. Check
  the [BladeRF](https://github.com/Nuand/bladeRF/wiki/#getting-started)
  GitHub wiki for more information on how to install the BladeRF library.

## Pre-requisites

### No-Std

Other than cargo and alloc support, with default-features disabled, you don't need anything else.

### Std

Nothing needs to be installed

### BladeRF

Please make sure you have the BladeRF library (libbladeRF) installed on your system. Check
the [BladeRF](https://github.com/Nuand/bladeRF/wiki/#getting-started)
GitHub wiki for more information on how to install the BladeRF library.

Here are some instructions for some operating systems:

#### Linux

##### Ubuntu

```bash
sudo add-apt-repository ppa:nuandllc/bladerf
sudo apt update
sudo apt install bladerf libbladerf-dev
```

##### Nix-Shell (NixOS)

```nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
    buildInputs = with pkgs; [
        libbladeRF
        
        git
        cmake
    ];
    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.libbladeRF
    ]}";
}
```

##### Building from source

Install the dependencies:

Debian/Ubuntu:

```bash
sudo apt install libusb-1.0-0-dev libusb-1.0-0 build-essential cmake libncurses5-dev libtecla1 libtecla-dev pkg-config git wget
```

RedHat/Fedora:

```bash
sudo yum groupinstall "Development Tools" "Development Libraries"
sudo yum install libusbx libusbx-devel cmake wget gcc-c++ libedit libedit-devel
```

Build the library:

```bash
git clone https://github.com/Nuand/bladeRF.git ./bladeRF
cd ./bladeRF
cd host/
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local -DINSTALL_UDEV_RULES=ON ../
make && sudo make install && sudo ldconfig
```

Troubleshooting:
It is possible that you may encounter an error saying that you have insufficient permissions to access bladeRF. To fix
this,
try running the program as root.

#### Windows

Use this installer to install the BladeRF
library: [BladeRF Windows Installer](https://nuand.com/windows_installers/bladeRF-win-installer-latest.exe)

#### MacOS

MacPorts:

```bash
sudo port install bladeRF +tecla
```

Compile from source (Be sure to have [Homebrew](https://brew.sh/) installed):

```bash
brew install libusb pkgconfig cmake libtecla
git clone https://github.com/Nuand/bladeRF.git
cd ./bladeRF
cd host/
mkdir build; cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/opt/local ..
make
sudo make install
```

### GUI

#### Linux

Most of these dependencies are already installed on your system. But if you don't have them, you can install them using
your package manager.
You need to have:

- GTK3
- Wayland or X11
- OpenGL
- libxkbcommon
- fontconfig
- pkg-config
- gcc
- rustc
- cargo

##### Ubuntu

```bash
sudo apt install libgtk-3-dev libwayland-dev libxkbcommon-dev libgl-dev fontconfig pkg-config gcc rustc cargo
```

##### Fedora

```bash
sudo dnf install gtk3-devel wayland-devel libxkbcommon-devel mesa-libGL-devel fontconfig pkg-config gcc rust cargo
```

##### Nix-Shell (NixOS) (Wayland)

```nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
		rustc
		cargo
		pkg-config
		fontconfig

        libxkbcommon
        libGL
        wayland
	];
	LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.rustc
        pkgs.cargo
        pkgs.libxkbcommon
        pkgs.libGL
        pkgs.wayland
    ]}";
}
```

##### Nix-Shell (NixOS) (X11)

```nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
		rustc
		cargo
		pkg-config
		fontconfig

        libxkbcommon
        libGL
        
        
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11
	];
	LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.rustc
        pkgs.cargo
        pkgs.libxkbcommon
        pkgs.libGL
        pkgs.xorg.libXcursor
        pkgs.xorg.libXrandr
        pkgs.xorg.libXi
        pkgs.xorg.libX11
    ]}";
}
```

#### Windows

All dependencies should already be included on your system. If you don't have them, you can install them using
the [MSYS2](https://www.msys2.org/) package manager.

#### MacOS

All dependencies should already be included on your system. If you don't have them, you can install them using
the [Homebrew](https://brew.sh/) package manager.

## Goals:

- [x] Cross-Hardware Math Acceleration
    - [x] CPUs (Native rust)
- [ ] Frequency and Phase Locked Loops
- [ ] Filters
    - [ ] Low-pass filters
    - [ ] High-pass filters
    - [ ] Pass-band filters
- [ ] Gain Control
    - [ ] Manual Gain Control (MGC)
    - [ ] Automatic Gain Control (AGC)
- [ ] UI (for debugging)
    - [x] Waterfall Chart
    - [x] Time Chart
    - [ ] Constellation Chart
    - [ ] Eye Diagram
- [ ] Modulation and Demodulation
    - [ ] FSK
    - [ ] BPSK
    - [ ] QPSK
    - [ ] QAM
- [ ] ???
