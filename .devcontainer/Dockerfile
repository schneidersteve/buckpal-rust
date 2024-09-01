FROM mcr.microsoft.com/devcontainers/base:bookworm

ARG RUST_VERSION=1.80.1

ENV RUSTUP_HOME=/home/vscode/.rustup
ENV CARGO_HOME=/home/vscode/.cargo

RUN apt update &&\
apt -y full-upgrade &&\
apt install build-essential &&\
apt -y install pkg-config &&\
apt install libssl-dev &&\
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain $RUST_VERSION -y &&\
chown -R vscode $RUSTUP_HOME &&\
chgrp -R vscode $RUSTUP_HOME &&\
chown -R vscode $CARGO_HOME &&\
chgrp -R vscode $CARGO_HOME

ENV PATH=$CARGO_HOME/bin:$PATH
