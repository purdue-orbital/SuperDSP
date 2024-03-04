# RustDSP (WIP)

This library is a very work in progress, and the interface currently is subject to changes.

## Mission

This library is not trying to replace existing radio-prototyping tools or amateur radio software like
[GNURadio-Companion](https://www.gnuradio.org/) or [SDRAngel](https://rgetz.github.io/sdrangel/). Rather, this project
aims to provide Rust tools and simplified versions of complex operations that enables these systems to be built. It
also aims to be used in production code and act as a drop-in replacement of a prototype built in
GNURadio-Companion. Although not the intended use, this library can be used in other applications like Ground,
Navigation, and Control (GNC) applications or sound design.

## Goals:


- [ ] Cross-Hardware Math Acceleration
    - [x] CPUs (Native rust)
    - [x] Vulkan
    - [ ] FPGAs (Verilog?)
    - [ ] ???
- [ ] Frequency and Phase Locked Loops
- [x] Rational Resampler
- [x] Filters
  - [x] Low-pass filters
  - [x] High-pass filters
  - [x] Pass-band filters
- [ ] Gain Control
  - [x] Manual Gain Control (MGC)
  - [ ] Automatic Gain Control (AGC)
- [ ] UI (for debugging)
  - [x] Waterfall Chart
  - [x] Time Chart
  - [x] Constellation Chart
  - [ ] Eye Diagram
- [ ] Modulation and Demodulation
  - [x] FSK
  - [ ] BPSK
  - [ ] QPSK
  - [ ] QAM
- [ ] ???
