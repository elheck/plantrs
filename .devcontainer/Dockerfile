FROM espressif/idf-rust

# [Optional] Uncomment this section to install additional OS packages.
RUN apt-get update && apt-get upgrade -y && export DEBIAN_FRONTEND=noninteractive 

RUN rustup update

RUN cargo install ldproxy cargo-espmonitor cargo-generate

     


