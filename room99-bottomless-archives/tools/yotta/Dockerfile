FROM nixos/nix
RUN nix-channel --update
COPY ./nix /build
WORKDIR /build
RUN nix --extra-experimental-features nix-command --extra-experimental-features flakes develop --command sh -c 'exit' 

WORKDIR /app
VOLUME /app
CMD nix --extra-experimental-features nix-command --extra-experimental-features flakes develop /build
