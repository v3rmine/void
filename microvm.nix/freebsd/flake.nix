{
  description = "FreeBSD MicroVM";

  nixConfig = {
    extra-substituters = [ "https://microvm.cachix.org" ];
    extra-trusted-public-keys =
      [ "microvm.cachix.org-1:oXnBc6hRE3eX5rSYdRyMYXnfzcCxC7yKPTbZXALsqys=" ];
  };

  inputs.microvm = {
    url = "github:astro/microvm.nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, microvm, ... }:
    let
      system = "x86_64-linux";
      vmname = "freebsd";
      nixosConfiguration = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          microvm.nixosModules.microvm
          ({ ... }: {
            networking.hostName = vmname;
            users.users.root.password = "";
            microvm = {
              guest.enable = false;
              interfaces = [{
                type = "user";
                id = "usernet";
                mac = "00:00:00:00:00:02";
              }];

              # "qemu" has 9p built-in!
              hypervisor = "qemu";
              vcpu = 14;
              mem = 16384;
            };
          })
        ];
      };
      vmRunner = pkgs:
        let declaredRunner = nixosConfiguration.config.microvm.declaredRunner;
        in pkgs.writeShellScriptBin "run-${vmname}-microvm" ''
          # Run the original VM runner
          exec ${declaredRunner}/bin/microvm-run
        '';
    in {
      packages.${system} = {
        default = with import nixpkgs { inherit system; }; vmRunner pkgs;
        vm = nixosConfiguration.config.microvm.declaredRunner;
      };
    };
}
