FROM gitpod/workspace-rust
RUN sudo apt update
RUN sudo apt install xorriso qemu-system-x86
RUN scripts/bootstrap-limine