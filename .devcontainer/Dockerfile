FROM ubuntu:24.04

RUN apt-get update -qq && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get install -y --no-install-recommends \
    git \
    gdb-multiarch \
    automake \
    autoconf \
    build-essential \
    texinfo \
    libtool \
    libftdi-dev \
    libusb-1.0-0-dev  \
    apt-transport-https \
    ca-certificates \
    pkg-config \
    minicom \
    curl \
    libudev-dev \
    binutils-arm-none-eabi \
    htop

RUN update-ca-certificates

RUN git clone https://github.com/raspberrypi/openocd.git --branch picoprobe --depth=1

RUN cd openocd && \
    ./bootstrap && \
    ./configure --disable-werror --enable-ftdi --enable-sysfsgpio --enable-bcm2835gpio --enable-picoprobe && \
    make -j4 && \
    make install

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update
RUN rustup target install thumbv6m-none-eabi
RUN cargo install flip-link probe-run

CMD [ "/bin/bash" ]
