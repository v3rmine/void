{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-26.05/nixexprs.tar.zst";
    colmena.url = "github:nix-community/colmena";
    impermanence.url = "github:nix-community/impermanence";
  };

  outputs = { nixpkgs, colmena, impermanence, ... }: 
  let
    lib = nixpkgs.lib;
    get_import_dir =
    dir:
    lib.flatten (
      lib.pipe dir [
        builtins.readDir
        (lib.filterAttrs (name: type: type == "directory" || lib.hasSuffix ".nix" name))
        (lib.mapAttrsToList (
          name: type: if type == "directory" then get_import_dir (dir + ("/" + name)) else dir + ("/" + name)
        ))
      ]
    );
  in {
    nixosConfigurations = {
      phloeomys = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [ ./vps3-phloeomys/configuration.nix ];
      };
      graphiurus = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [ ./nas-graphiurus/vms/uncloud-node/configuration.nix ];
      };
    };

    colmenaHive = colmena.lib.makeHive {
      meta = {
        nixpkgs = import nixpkgs {
          system = "x86_64-linux";
          overlays = [];
        };
      };

      defaults = { pkgs, ... }: {
        imports = [
          impermanence.nixosModules.impermanence
        ] ++ (get_import_dir ./nixos-modules/.);

        # Max versions count of previous boot
        boot.loader.grub.configurationLimit = 4;
        boot.loader.systemd-boot.configurationLimit = 4;

        # Limit journal file size
        services.journald.extraConfig = ''
	  SystemMaxUse=256M
	  MaxFileSec=7day
	'';
      };

      phloeomys = { name, nodes, pkgs, ... }: {
        deployment = {
          tags = [ "ingress" "vps" ];
          targetHost = "46.225.11.44";
          targetPort = 8080;
          targetUser = "root";
        };
        imports = [ ./vps3-phloeomys/configuration.nix ];
      };
      
      graphiurus = { name, nodes, pkgs, ... }: {
        deployment = {
          tags = [ "nas" ];
          targetHost = "10.0.0.14";
          targetPort = 8080;
          targetUser = "root";
        };
        imports = [ ./nas-graphiurus/vms/uncloud-node/configuration.nix ];
      };
    };
  };
}
