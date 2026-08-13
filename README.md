# fbsd-kdev

> **Note:** This project is currently under active development. Expect changes and potential issues.

fbsd-kdev is a framework for writing FreeBSD kernel devices in Rust. It is designed to leverage the memory safe features of Rust while writing kernel devices for ease of development. It provides safe abstractions for many features in the FreeBSD kernel. Compile into a static library (.a) on a development machine and link it to a FreeBSD kernel object (.ko) using the FreeBSD build environment.


## Tested Environments
| Hardware | Operating System | Architecture |
| :--- | :--- | :--- |
| **Raspberry Pi 3 Model B** | FreeBSD 15.0-RELEASE | `aarch64` (64-bit ARM) |

## Prerequisites

### Development Machine
* **Rust & Cargo**
* **Target Toolchain** for ARM64:
  ```bash
  rustup target add aarch64-unknown-none
  ```
* **Make**

### Target Machine (FreeBSD)
* **FreeBSD Kernel and Source/Headers**
* **Build Tools** (make & clang)


## Usage

The build pipeline spans two stages, one on the development machine and the other on the target FreeBSD machine.

### Step 1: Gather Necessary Build Files

Clone the repository and build the Rust static Library:
```bash
git clone https://github.com/NickBozarth/fbsd-kdev.git
cd fbsd-kdev/examples
make
```

This compiles the necessary static library (.a) and copies the other necessary files for generating the final FreeBSD kernel object (.ko).

### Step 2: Transfer Files to Target Machine

Transfer the target/ folder to the target FreeBSD machine.

#### Example Using an ext2-formatted USB

On linux host:
```bash
sudo mount /dev/sdX1 /mnt/usb
sudo cp -r target/ /mnt/usb/
sudo unmount /mnt/usb
```

On target FreeBSD machine:
```bash
sudo mount -t ext2fs /dev/da0s1 /mnt/usb
```

### Step 3: Compile Kernel Module

On target FreeBSD machine:

```bash
cd target
make
```

This will assemble the project into a FreeBSD kernel object (.ko) that can now be loaded.


### Step 4: Interact With Kernel Object

Load the Kernel Module
```bash
sudo kldload ./kdev.ko
```

Verify Module Status:
```bash
kldstat | grep kdev
```

Unload Module:
```bash
sudo kldunload kdev
```
