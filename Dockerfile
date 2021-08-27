FROM ubuntu:16.04

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \ 
    libc6-dev-i386 \
    gcc-4.9-arm-linux-gnueabihf \
    wget \


# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Install llvm and clang
RUN wget -q -O clang+llvm-7.0.0-armv7a-linux-gnueabihf.tar.xz https://releases.llvm.org/7.0.0/clang%2bllvm-7.0.0-aarch64-linux-gnu.tar.xz | \
tar -xvf gcc-arm-9.2-2019.12-x86_64-arm-none-linux-gnueabihf.tar.xz

COPY . .