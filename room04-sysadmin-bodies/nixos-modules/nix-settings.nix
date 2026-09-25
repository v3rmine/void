{ ... }: {
  imports = [];
  options = {};

  config = {
    nix = {
      gc = {
        automatic = true;
        dates = "daily";
        options = "--delete-older-than 2d";      
      };
      extraOptions = ''
        min-free = ${toString (100 * 1024 * 1024)}
        max-free = ${toString (1024 * 1024 * 1024)}
      '';

      settings = {
        auto-optimise-store = true; # <- this option will hardlink identical files
        experimental-features = [ "nix-command" "flakes" ];
        substituters = [
          "https://cache.nixos.org"
          "https://nix-community.cachix.org"
        ];
        trusted-public-keys = [
          "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
        ];
      };
    };
  };
}
