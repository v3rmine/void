{
  description = "Mise dev MicroVM";

  nixConfig = {
    extra-substituters = [ "https://microvm.cachix.org" ];
    extra-trusted-public-keys =
      [ "microvm.cachix.org-1:oXnBc6hRE3eX5rSYdRyMYXnfzcCxC7yKPTbZXALsqys=" ];
  };

  inputs.impermanence.url = "github:nix-community/impermanence";
  inputs.microvm = {
    url = "github:astro/microvm.nix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, impermanence, nixpkgs, microvm }:
    let
      # BEGIN shared persistence for all projects using Mise flake
      sharedPersistencePath = "/persistent-shared";
      sharedPersistenceHostPath = "${
          builtins.getEnv "HOME"
        }/.local/share/microvm/vms/${vmname}/persistent";
      # END shared persistence for all projects using Mise flake
      # BEGIN unique persistence for the PWD
      uniquePersistencePath = "/persistent-unique";
      pwdFullHash = builtins.hashString "sha256" (builtins.getEnv "PWD");
      pwdHash = builtins.substring ((builtins.stringLength pwdFullHash) - 10) 10
        pwdFullHash;
      uniquePersistenceHostPath = "${
          builtins.getEnv "HOME"
        }/.local/share/microvm/vms/${vmname}-${pwdHash}/unique";
      # END unique persistence for the PWD
      system = "x86_64-linux";
      user = "v3rmine";
      vmname = "mise";
      wrapperPath = "/run/wrappers";
      nixosConfiguration = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          microvm.nixosModules.microvm
          impermanence.nixosModules.impermanence
          ({ pkgs, ... }:
            let
              customPkgs = {
                quit = (pkgs.writeScriptBin "quit" ''
                  #!${pkgs.bash}/bin/bash
                  echo -e "\033[0;31mShutting down the VM...\033[0m"
                  sudo shutdown 0
                '');
              };
            in {
              environment.persistence.${sharedPersistencePath} = {
                hideMounts = true;
                directories = [ "/var/lib/nixos" ];
                users.${user} = { directories = [ ".local/share/mise" ]; };
              };
              environment.persistence.${uniquePersistencePath} = {
                hideMounts = true;
                directories = [ ];
                users.${user} = { directories = [ ]; };
              };

              environment.sessionVariables = { TERM = "screen-256color"; };

              environment.systemPackages = builtins.concatLists [
                (with pkgs; [ git python3 gcc gnumake gnupg mise ])
                (builtins.attrValues customPkgs)
              ];

              networking.hostName = vmname;

              nix.enable = true;
              nix.nixPath = [ "nixpkgs=${builtins.storePath <nixpkgs>}" ];
              nix.settings = {
                extra-experimental-features = [ "nix-command" "flakes" ];
                trusted-users = [ user ];
              };

              security.sudo-rs = {
                enable = true;
                wheelNeedsPassword = false;
              };

              services.getty.autologinUser = user;
              services.openssh = { enable = true; };
              system.stateVersion = "24.11";
              systemd.services.loadnixdb = {
                description = "import hosts nix database";
                path = [ pkgs.nix ];
                wantedBy = [ "multi-user.target" ];
                requires = [ "nix-daemon.service" ];
                script =
                  "cat ${sharedPersistencePath}/nix-store-db-dump|nix-store --load-db";
              };
              time.timeZone = nixpkgs.lib.mkDefault "Europe/Paris";
              # HACK: Fix incorrect home directory permissions
              # SOURCE: https://github.com/NixOS/nixpkgs/issues/10888
              systemd.tmpfiles.rules = [ "d /home/${user} 0700 ${user} user" ];
              users.users.${user} = {
                extraGroups = [ "wheel" "video" "docker" ];
                group = "user";
                home = "/home/${user}";
                isNormalUser = true;
                password = "";
              };
              users.users.root.password = "";
              users.groups.user = { };

              systemd.user.services = {
                fix-app-mount-permissions = {
                  description = "Fix app permissions";
                  path = [ pkgs.coreutils wrapperPath ];
                  after = [ "home-${user}-app.mount" ];
                  before = [ "default.target" ];
                  wantedBy = [ "default.target" ];
                  serviceConfig = {
                    Type = "oneshot";
                    ExecStart =
                      "${pkgs.bash}/bin/bash -c 'sudo chown -R ${user}:user /home/${user}/app'";
                  };
                };
                fix-mise-permissions = {
                  description = "Fix mise permissions";
                  path = [ pkgs.coreutils wrapperPath ];
                  after = [ "home-v3rmine-.local-share-mise.mount" ];
                  before = [ "default.target" ];
                  wantedBy = [ "default.target" ];
                  serviceConfig = {
                    Type = "oneshot";
                    ExecStart =
                      "${pkgs.bash}/bin/bash -c 'sudo chown -R ${user}:user /home/${user}/.local/share/mise'";
                  };
                };
              };

              # Need persistences path for boot
              fileSystems.${sharedPersistencePath}.neededForBoot =
                nixpkgs.lib.mkForce true;
              fileSystems.${uniquePersistencePath}.neededForBoot =
                nixpkgs.lib.mkForce true;

              microvm = {
                forwardPorts = [ ];
                interfaces = [{
                  type = "user";
                  id = "usernet";
                  mac = "00:00:00:00:00:02";
                }];
                shares = [
                  {
                    # use proto = "virtiofs" for MicroVMs that are started by systemd
                    proto = "9p";
                    tag = "ro-store";
                    # a host's /nix/store will be picked up so that no
                    # squashfs/erofs will be built for it.
                    source = "/nix/store";
                    mountPoint = "/nix/.ro-store";
                  }
                  {
                    proto = "9p";
                    tag = "persistent-shared";
                    source = sharedPersistenceHostPath;
                    mountPoint = sharedPersistencePath;
                    securityModel = "mapped";
                  }
                  {
                    proto = "9p";
                    tag = "persistent-unique";
                    source = uniquePersistenceHostPath;
                    mountPoint = uniquePersistencePath;
                    securityModel = "mapped";
                  }
                  {
                    proto = "9p";
                    tag = "current";
                    source = "${builtins.getEnv "PWD"}";
                    mountPoint = "/home/${user}/app";
                    securityModel = "mapped";
                  }
                ];

                # "qemu" has 9p built-in!
                hypervisor = "qemu";
                socket = "/run/user/1000/microvm-control.socket";
                vcpu = 14;
                mem = 16384;
                volumes = [ ];
                writableStoreOverlay = "/nix/.rwstore";
              };
            })
        ];
      };
      vmRunner = pkgs:
        let declaredRunner = nixosConfiguration.config.microvm.declaredRunner;
        in pkgs.writeShellScriptBin "run-${vmname}-microvm" ''
          # Create persistences directories if they doesn't exist
          mkdir -p "${sharedPersistenceHostPath}"
          mkdir -p "${uniquePersistenceHostPath}"
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
