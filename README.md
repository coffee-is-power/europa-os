# europa-os
A OS made in rust for learning and fun

# Running a VM
## Requirements:
  - Xorriso
  - qemu (x86)
  - [rustup](https://rustup.rs) **(Recommended)** | **nightly** rust compiler
  - And have a **linux machine** (If you have windows you'll need WSL)
## Steps
1. Clone the project:
```bash
git clone https://github.com/coffee-is-power/europa-os --recurse-submodules
```
2. Compile limine-install
```bash
scripts/bootstrap-limine
```
3. Build kernel and run qemu
```bash
cargo run
```
You're done!
