# Superdsp-bladerf
Implements bladerf source and sink modules for superdsp

## Install Dependencies
Required
- Bladerf drivers

### Linux (Package Managers)

#### Ubuntu
```shell
sudo add-apt-repository ppa:nuandllc/bladerf
sudo apt update
sudo apt install bladerf
sudo apt install libbladerf-dev
```

#### Arch (Broken)
On 2/27/2025, there was an error in installing this library involving C versions. This problem is fixed on 
Nuand's repo but has not been updated on the AUR package manager, which isn't first party. Use the build from source for now.
```shell
yay -Syu libbladerf-git
```

### Linux (From Source)

#### Ubuntu
```shell
sudo apt install libusb-1.0-0-dev libusb-1.0-0 build-essential cmake libncurses5-dev libtecla1 libtecla-dev pkg-config git wget
git clone https://github.com/Nuand/bladeRF.git ./bladeRF
cd ./bladeRF
cd host/
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local -DINSTALL_UDEV_RULES=ON ../
make && sudo make install && sudo ldconfig
```

#### Fedora
```shell
sudo yum groupinstall "Development Tools" "Development Libraries"
sudo yum install libusbx libusbx-devel cmake wget gcc-c++ libedit libedit-devel
git clone https://github.com/Nuand/bladeRF.git ./bladeRF
cd ./bladeRF
cd host/
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local -DINSTALL_UDEV_RULES=ON ../
make && sudo make install && sudo ldconfig
```

#### Arch
```shell
sudo pacman -Syu libusb cmake wget gcc git
git clone https://github.com/MartinHerren/bladeRF.git ./bladeRF
cd ./bladeRF
cd host/
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr/local -DINSTALL_UDEV_RULES=ON ../
make && sudo make install && sudo ldconfig
```