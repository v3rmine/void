FROM rust:latest as stable
RUN apt update
RUN apt install -y gcc-mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
CMD ["/bin/bash"]

FROM instrumentisto/rust:beta as beta
RUN apt update
RUN apt install -y gcc-mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
CMD ["/bin/bash"]

FROM instrumentisto/rust:nightly as nightly
RUN apt update
RUN apt install -y gcc-mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
CMD ["/bin/bash"]

