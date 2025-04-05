FROM debian:buster-slim

SHELL ["/bin/bash", "-c"]

RUN apt update

# Prepare
RUN mkdir "/data"
WORKDIR "/data"
VOLUME ["/data"]

RUN apt install -y curl git build-essential pkg-config libtool libffi-dev libczmq-dev libzmq3-dev

# Jupyter
RUN curl -o /root/miniconda.sh https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
RUN bash /root/miniconda.sh -b -p /root/miniconda
ENV PATH="${PATH}:/root/miniconda/bin"
RUN conda install -c conda-forge jupyterlab pip nb_conda_kernels ipywidgets

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${PATH}:/root/.cargo/bin"
RUN rustup install nightly
RUN cargo install evcxr_jupyter --no-default-features
RUN evcxr_jupyter --install

# Nim
RUN curl https://nim-lang.org/choosenim/init.sh -sSf | CHOOSENIM_CHOOSE_VERSION=stable sh -s -- -y
ENV PATH="${PATH}:/root/.nimble/bin"
RUN conda install matplotlib
RUN nimble install https://github.com/stisa/jupyternim -y
RUN jupyternim

# C
RUN /root/miniconda/bin/pip install jupyter-c-kernel
RUN install_c_kernel

# C++
RUN conda install xeus-cling -c conda-forge

# ASM
RUN /root/miniconda/bin/pip install emu86
RUN /root/miniconda/bin/python3 -m kernels.intel.install

# Ruby
RUN apt install -y ruby ruby-dev
RUN gem install ffi-rzmq
RUN gem install iruby --pre
RUN iruby register --force

# Go
RUN curl -o /root/go.linux-amd64.tar.gz https://dl.google.com/go/go1.14.4.linux-amd64.tar.gz
RUN tar -C /usr/local -xzf /root/go.linux-amd64.tar.gz
ENV PATH="${PATH}:/usr/local/go/bin"
RUN env GO111MODULE=on go get github.com/gopherdata/gophernotes &&\
  mkdir -p ~/.local/share/jupyter/kernels/gophernotes &&\
  cd ~/.local/share/jupyter/kernels/gophernotes &&\
  cp "$(go env GOPATH)"/pkg/mod/github.com/gopherdata/gophernotes@v0.7.0/kernel/* "." &&\
  chmod +w ./kernel.json &&\
  sed "s|gophernotes|$(go env GOPATH)/bin/gophernotes|" < kernel.json.in > kernel.json

# Nodejs
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt install -y nodejs
RUN npm install -g tslab
RUN tslab install --python /root/miniconda/bin/python3

# Extensions
#RUN /root/miniconda/bin/pip install jupyterlab_latex
#RUN jupyter labextension install @jupyterlab/latex --no-build

#RUN /root/miniconda/bin/pip install plotly
#RUN jupyter labextension install jupyterlab-plotly --no-build

RUN jupyter labextension install @lckr/jupyterlab_variableinspector --no-build
RUN jupyter labextension install @jupyter-widgets/jupyterlab-manager --no-build

RUN jupyter labextension install @oriolmirosa/jupyterlab_materialdarker --no-build

RUN jupyter labextension install @aquirdturtle/collapsible_headings --no-build

RUN jupyter labextension install @krassowski/jupyterlab_go_to_definition --no-build

# Cleanup
RUN apt clean

EXPOSE 8888

VOLUME ["/data","/root/.jupyter"]

RUN jupyter lab build
# ENTRYPOINT ["jupyter","lab","--port=8888","--ip=0.0.0.0","--allow"]
ENTRYPOINT ["jupyter","lab"]
