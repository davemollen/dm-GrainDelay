FROM ubuntu:18.04 as moddwarf-build

# Set root user
USER root

# Set binary name
ARG BINARY_NAME
ENV BINARY_NAME=$BINARY_NAME

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \ 
    gcc-7-aarch64-linux-gnu \
    libc6-dev-i386

# Get LLVM
RUN curl -SL https://releases.llvm.org/8.0.0/clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz \
    | tar -xJC . && \
    mv clang+llvm-8.0.0-x86_64-linux-gnu-ubuntu-18.04 llvm
ENV PATH="/llvm/bin:$PATH"
ENV LLVM_PATH="/llvm:$LLVM_PATH"
ENV LD_LIBRARY_PATH="/llvm/lib:$LD_LIBRARY_PATH"
ENV DYLD_LIBRARY_PATH="/llvm/lib:$DYLD_LIBRARY_PATH"

# Get Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.58.1
RUN rustup target add aarch64-unknown-linux-gnu --toolchain 1.58.1

# Copy files
COPY ./ ./

# Build 
RUN cd lv2 && cargo +1.58.1 build --release --target aarch64-unknown-linux-gnu

# Export
FROM scratch AS moddwarf-export
COPY --from=moddwarf-build "lv2/target/aarch64-unknown-linux-gnu/release/$BINARY_NAME" .