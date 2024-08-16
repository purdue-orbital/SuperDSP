# RustDSP (WIP)

This library is a very work in progress, and the interface currently is subject to changes.

## Mission

This library is not trying to replace existing radio-prototyping tools or amateur radio software like
[GNURadio-Companion](https://www.gnuradio.org/) or [SDRAngel](https://rgetz.github.io/sdrangel/). Rather, this project
aims to provide Rust tools and simplified versions of complex operations that enables these systems to be built. It
also aims to be used in production code and act as a drop-in replacement of a prototype built in
GNURadio-Companion. Although not the intended use, this library can be used in other applications like Ground,
Navigation, and Control (GNC) applications or sound design.

## Feature Flags
To use this library without the standard library, disable default features. This library **REQUIRES** the ``alloc`` crate.
Even in no_std mode, the ``alloc`` crate is used to allocate memory for buffers and other objects.

- ``std``: Enabled by default. Disables the ``no_std`` feature and enables the standard library.
- ``gui``: Enables GUI features and objects. This flag automatically enables the ``std`` feature.
- ``bladerf``: Enables the BladeRF hardware support. This flag automatically enables the ``std`` feature. Please make sure
you have the BladeRF library (libbladeRF) installed on your system. Check the [BladeRF](https://github.com/Nuand/bladeRF/wiki/#getting-started) 
Github wiki for more information on how to install the BladeRF library.

## Goals:

- [ ] Cross-Hardware Math Acceleration
    - [x] CPUs (Native rust)
    - [ ] Vulkan
    - [ ] FPGAs (Verilog?)
    - [ ] ???
- [ ] Frequency and Phase Locked Loops
- [ ] Rational Resampler
- [ ] Filters
    - [ ] Low-pass filters
    - [ ] High-pass filters
    - [ ] Pass-band filters
- [ ] Gain Control
    - [ ] Manual Gain Control (MGC)
    - [ ] Automatic Gain Control (AGC)
- [ ] UI (for debugging)
    - [ ] Waterfall Chart
    - [x] Time Chart
    - [ ] Constellation Chart
    - [ ] Eye Diagram
- [ ] Modulation and Demodulation
    - [ ] FSK
    - [ ] BPSK
    - [ ] QPSK
    - [ ] QAM
- [ ] ???
