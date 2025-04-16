{
  description = "Deployment for Tailscale container";

  # For accessing `deploy-rs`'s utility Nix functions
  inputs.deploy-rs.url = "github:serokell/deploy-rs";

  outputs = { self, nixpkgs, deploy-rs }: {
    nixosConfigurations.tailscale = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [ ./configuration.nix ];
    };

    deploy.nodes.tailscale = {
        hostname = "tailscale";
        sshUser = "root";

        profiles.system = {
          user = "root";
          path = deploy-rs.lib.x86_64-linux.activate.nixos self.nixosConfigurations.tailscale;
        };
    };

    # This is highly advised, and will prevent many possible mistakes
    checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
  };
}
