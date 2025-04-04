{
  description = "Build for microbit in nix";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    yotta.url = "github:joxcat/yotta";
  };

  outputs = { self, nixpkgs, utils, yotta }: 
  utils.lib.eachSystem [ utils.lib.system.x86_64-linux ] (system: 
    let 
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            yotta.packages.${system}.default
            cmake
            ninja
            gcc-arm-embedded
            srecord
          ];
        };
      };
    }
  );
}
