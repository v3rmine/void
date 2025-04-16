{
  description = "Deployment for Tailscale container";

  outputs = { nixpkgs, ... }: {
    nixosConfigurations.tailscale = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [ ./configuration.nix ];
    };
  };
}
