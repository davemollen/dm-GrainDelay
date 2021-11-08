FROM ubuntu:16.04 as mod-duo-build

# Set root user
USER root

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \ 
    gcc-4.9-arm-linux-gnueabihf \
    libc6-dev-i386
    
# Get LLVM
RUN curl -SL https://releases.llvm.org/7.0.0/clang+llvm-7.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz \
 | tar -xJC . && \
 mv clang+llvm-7.0.0-x86_64-linux-gnu-ubuntu-16.04 llvm
ENV PATH="/llvm/bin:$PATH"
ENV LLVM_PATH="/llvm:$LLVM_PATH"
ENV LD_LIBRARY_PATH="/llvm/lib:$LD_LIBRARY_PATH"
ENV DYLD_LIBRARY_PATH="/llvm/lib:$DYLD_LIBRARY_PATH"

# Get Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add armv7-unknown-linux-gnueabihf

# Copy 
COPY ./ ./

# Build 
RUN cargo build --release --target armv7-unknown-linux-gnueabihf

# Export
FROM scratch AS mod-duo-export
COPY --from=mod-duo-build /target/armv7-unknown-linux-gnueabihf/release/liboctaver.so ./mod-duo

####################################

FROM ubuntu:18.04 as mod-dwarf-build

# Set root user
USER root

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
RUN rustup target add aarch64-unknown-linux-gnu

# Copy 
COPY ./ ./

# Build 
RUN cargo build --release --target aarch64-unknown-linux-gnu

# Export
FROM scratch AS mod-dwarf-export
COPY --from=mod-dwarf-build /target/aarch64-unknown-linux-gnu/release/liboctaver.so ./mod-dwarf