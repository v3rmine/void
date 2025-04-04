FROM ubuntu:20.04 AS base
MAINTAINER joxcat
ENV ROOTDIR=/notebooks
ENV TZ=Europe/Paris

RUN apt-get update -y
RUN apt-get install -y tzdata \
    && apt-get install -y python3 python3-pip
RUN pip install 'jupyterlab>=3.0.0,<4.0.0a0' jupyterlab-lsp

ENV BUILD_DEPS="curl"
RUN apt-get install -y $BUILD_DEPS

# Install Elm
# https://github.com/abingham/jupyter-elm-kernel
WORKDIR /build/elm
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz \
    && gunzip elm.gz \
    && chmod +x elm \
    && mv elm /usr/local/bin/
WORKDIR /build
RUN pip install elm_kernel \
    && python3 -m elm_kernel.install

# Install Emu86
# https://github.com/gcallah/Emu86/tree/master/kernels
RUN pip install emu86 \
    && python3 -m kernels.intel.install

# Install iruby
# https://github.com/SciRuby/iruby
ENV BUILD_DEPS="${BUILD_DEPS} libtool libffi-dev ruby ruby-dev make libzmq3-dev libczmq-dev"
RUN apt-get install -y $BUILD_DEPS

RUN gem install ffi-rzmq \
    && gem install iruby --pre
RUN iruby register --force

# Install TS / JS
# https://github.com/yunabe/tslab
ENV BUILD_DEPS="${BUILD_DEPS} nodejs npm"
RUN apt-get install -y $BUILD_DEPS

RUN npm install -g tslab \
    && tslab install

# Install Rust
# https://github.com/google/evcxr/tree/main/evcxr_jupyter
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${PATH}:/root/.cargo/bin"

ENV BUILD_DEPS="${BUILD_DEPS} cmake"
RUN apt-get install -y $BUILD_DEPS

RUN cargo install evcxr_jupyter \
    && evcxr_jupyter --install

# Install coq
# https://github.com/EugeneLoy/coq_jupyter
RUN apt-get install -y coq coqide
RUN pip install coq-jupyter \
    && python3 -m coq_jupyter.install

# Install Java
# https://github.com/SpencerPark/IJava
RUN apt-get install -y openjdk-11-jdk
WORKDIR /build/java
RUN curl -L -o ijava.zip https://github.com/SpencerPark/IJava/releases/download/v1.3.0/ijava-1.3.0.zip \
    && unzip ijava.zip
RUN python3 install.py

# Install Clojure
# https://github.com/clojupyter/clojupyter
ENV BUILD_DEPS="${BUILD_DEPS} git"
RUN apt-get install -y $BUILD_DEPS

RUN curl https://raw.githubusercontent.com/technomancy/leiningen/stable/bin/lein > lein \
    && mv lein /usr/local/bin/lein \
    && chmod a+x /usr/local/bin/lein
RUN apt-get install -y rlwrap
RUN curl -O https://download.clojure.org/install/linux-install-1.10.3.839.sh \
    && chmod +x linux-install-1.10.3.839.sh \
    && ./linux-install-1.10.3.839.sh
WORKDIR /build/clojure
RUN git clone https://github.com/clojupyter/clojupyter --recurse-submodules --depth 1 --branch 0.3.2 .

ENV LANG=en_US.UTF-8 LANGUAGE=en_US.UTF-8 LC_ALL=en_US.UTF-8
RUN apt-get install -y locales \
    && sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && locale-gen
RUN make install

# Install gophernotes
# https://github.com/gopherdata/gophernotes
RUN apt-get install -y golang
ENV GOPATH /go
RUN env GO111MODULE=on go get github.com/gopherdata/gophernotes@v0.7.2 \
    && mkdir -p /usr/share/jupyter/kernels/gophernotes \
    && cd /usr/share/jupyter/kernels/gophernotes \
    && cp "$(go env GOPATH)"/pkg/mod/github.com/gopherdata/gophernotes@v0.7.2/kernel/* "." \
    && chmod +w ./kernel.json \
    && sed "s|gophernotes|$(go env GOPATH)/bin/gophernotes|" < kernel.json.in > kernel.json

# Install ielixir
# https://github.com/pprzetacznik/IElixir
ENV DEBIAN_FRONTEND=noninteractive

ENV BUILD_DEPS="${BUILD_DEPS} libncurses5-dev libncursesw5-dev libssl-dev libxslt-dev libwxbase3.0-dev autoconf"
RUN apt-get install -y $BUILD_DEPS

ENV WORK_DIR=/opt/elixir
ENV PATH=/opt/mix:${PATH} \
    MIX_ENV=prod \
    MIX_HOME=/opt/mix \
    HEX_HOME=/opt/hex \
    ERL_VERSION=23.1.2 \
    ELIXIR_VERSION=1.11.2 \
    ERL_TOP=$WORK_DIR/otp \
    ELIXIR_TOP=$WORK_DIR/elixir
WORKDIR $WORK_DIR
RUN curl -L -o otp.zip https://github.com/erlang/otp/archive/OTP-$ERL_VERSION.zip \
  && unzip otp.zip -d $WORK_DIR \
  && mv $WORK_DIR/otp-OTP-$ERL_VERSION $ERL_TOP \
  && rm otp.zip

RUN cd $ERL_TOP \
  && ./otp_build autoconf \
  && ./configure --prefix=$ERL_TOP/target \
  && make \
  && make install \
  && cd $WORK_DIR \
  && curl -L -o elixir.zip https://github.com/elixir-lang/elixir/releases/download/v$ELIXIR_VERSION/Precompiled.zip \
  && unzip elixir.zip -d $ELIXIR_TOP \
  && rm elixir.zip

RUN apt-get install -y \
    libzmq3-dev \
    libsqlite3-dev
ENV PATH="${ERL_TOP}/target/bin:${ELIXIR_TOP}/bin:${PATH}"
WORKDIR /opt/ielixir
RUN mix local.rebar --force \
    && mix local.hex --force \
    && git clone --depth 1 https://github.com/pprzetacznik/IElixir.git .
RUN mix deps.get
RUN mix compile
RUN echo '{"argv":["/opt/ielixir/start_script.sh","{connection_file}"],"display_name":"Elixir","language":"Elixir"}' \
    | python3 -m json.tool > /opt/ielixir/resources/ielixir/kernel.json \
    && cp -r /opt/ielixir/resources/ielixir /usr/share/jupyter/kernels

# Hy lang (print "python X lisp <3")
RUN pip install hy \
    && pip3 install git+https://github.com/ekaschalk/jedhy.git \
    && pip3 install git+https://github.com/Calysto/calysto_hy.git \
    && python3 -m calysto_hy install

# Cleanup
# RUN apt-get remove -y $BUILD_DEPS
# RUN apt-get clean -y

VOLUME $ROOTDIR
WORKDIR $ROOTDIR
EXPOSE 8888
CMD [ "jupyter", "notebook", "--no-browser", "--allow-root", "--ip=0.0.0.0" ]

FROM base
COPY . $ROOTDIR
