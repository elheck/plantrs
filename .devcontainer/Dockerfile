FROM ubuntu:22.04

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
     minicom

RUN update-ca-certificates

RUN git clone https://github.com/raspberrypi/openocd.git --branch picoprobe --depth=1

RUN cd openocd && \
    ./bootstrap && \
    ./configure --disable-werror --enable-ftdi --enable-sysfsgpio --enable-bcm2835gpio --enable-picoprobe && \
    make -j4 && \
    make install && \
    cd .. && \
    rm -r openocd
