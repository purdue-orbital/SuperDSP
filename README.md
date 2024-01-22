# RustDSP (WIP)
This library is a very work in progress and the interface currently is subject to changes.

## Mission
This library is not trying to replace existing radio prototyping tools / ham radio software like 
[GNURadio-Companion](https://www.gnuradio.org/) or [SDRAngel](https://rgetz.github.io/sdrangel/). Rather, this project 
aims to provide Rust the tools and simplified versions of complex operations that enables these systems to be built. It 
also aims to be used in production code and act as a drop in replacement of turning a prototype built in 
GNURadio-Companion into code that can be flashed onto an FPGA of an SDR radio or computer system connected to the radio.
Although not the intended use, this library can be used in other applications like Ground, Navigation, and Control (GNC)
applications or sound design.

## Goals:
- [ ] Cross-Hardware Math Acceleration
  - [ ] CPUs (Native rust)
    - [x] Single threaded
    - [ ] Multithreaded ???
  - [x] Vulkan 
  - [ ] FPGAs (Verilog?)
  - [ ] ???
- [ ] Frequency and Phase Locked Loops
- [ ] Rational Resamplers
- [ ] Filters
  - [ ] Low pass filters
  - [ ] Pass-band filters
- [ ] Modulation and Demodulation
  - [ ] BPSK
  - [ ] QPSK
  - [ ] FM
  - [ ] QAM
- [ ] ???
