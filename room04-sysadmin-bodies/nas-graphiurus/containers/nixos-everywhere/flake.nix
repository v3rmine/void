{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  inputs.disko.url = "github:nix-community/disko";
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { nixpkgs, disko, ... }:
    let
      specialArgs = {
        secrets = { sshPublicKey = builtins.getEnv "SSH_PUBLIC_KEY"; };
      };
      currentSystem = "x86_64-linux";
      targetSystem = "aarch64-linux";
      pkgs = import nixpkgs {
        system = currentSystem;
        crossSystem = targetSystem;
      };
    in {
      nixosConfigurations.default = pkgs.lib.nixosSystem {
        inherit specialArgs;
        # system = "x86_64-linux";
        system = targetSystem;
        modules = [ disko.nixosModules.disko ./etc/nixos/configuration.nix ];
      };
    };
}
