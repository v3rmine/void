# Inspired by Avalanche https://git.madhouse-project.org/algernon/avalanche

{
  description = "Nix packages because nix";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05"; };

  outputs = { self, nixpkgs, systems, ... }@inputs:
    let
      inherit (nixpkgs) lib;

      forEachSystem = f:
        nixpkgs.lib.genAttrs (import systems)
        (system: let pkgs = import nixpkgs { inherit system; }; in f pkgs);

      forEachPackage = pred: f:
        builtins.listToAttrs (map (pkg: {
          name = pkg;
          value = f pkg;
        }) (builtins.filter pred
          (builtins.attrNames (builtins.readDir ./pkgs))));

      makeOverlays = pkg:
        let
          mainPackage = ./pkgs + "/${pkg}/package.nix";
          overlayFile = ./pkgs + "/${pkg}/overlay.nix";
          subPackage = pkg: subPkg: ./pkgs + "/${pkg}/packages/${subPkg}";
        in if builtins.pathExists overlayFile then
          import overlayFile { inherit self inputs; }
        else
          _final: prev:
          let makePackage = path: (prev.callPackage path { inherit inputs; });
          in if builtins.pathExists mainPackage then {
            ${pkg} = makePackage mainPackage;
          } else
            builtins.listToAttrs (map (file: {
              name = lib.strings.removeSuffix ".nix" file;
              value = makePackage (subPackage pkg file);
            }) (builtins.attrNames
              (builtins.readDir (./pkgs + "/${pkg}/packages"))));

      hasFile = file: pkg: builtins.pathExists (./packs + "/${pkg}/${file}");
      hasPackages = pkg: hasFile "package.nix" pkg || hasFile "packages" pkg;

    in {
      overlays = let directOverlays = forEachPackage hasPackages makeOverlays;
      in directOverlays // {
        default = final: prev:
          builtins.foldl' (acc: val: acc // val) { } (builtins.attrValues
            (builtins.mapAttrs (_: o: o final prev) directOverlays));
      };

      packages = forEachSystem (pkgs: self.overlays.default pkgs pkgs);
    };
}
