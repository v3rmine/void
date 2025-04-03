FROM kcov/kcov:38 as base
RUN apt update
RUN apt install -y curl gcc cmake
CMD ["/bin/bash"]

FROM base as stable
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable
RUN $HOME/.cargo/bin/cargo install cargo-kcov

FROM base as nightly
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=nightly
RUN $HOME/.cargo/bin/cargo install cargo-kcov
