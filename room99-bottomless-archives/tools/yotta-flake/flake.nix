{
  description = "yotta in a flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachSystem [ utils.lib.system.x86_64-linux ] (system: 
    let 
      pkgs = nixpkgs.legacyPackages.${system};
      stdenv = pkgs.stdenv;
      lib = pkgs.lib;

      pypkgs-build-requirements = {
        hgapi = [ "setuptools" ];
        jsonschema = [ "vcversioner" ];
        mbed-test-wrapper = [ "setuptools" ];
        project-generator-definitions = [ "setuptools" ];
        pyocd-pemicro = [ "setuptools" ];
        pyusb = (self: super: old: {
          postPatch = '''';
        });
        urllib3 = [ "hatchling" ];
      };
      autoOverrides = (self: super:
        (builtins.mapAttrs (package: build-requirements:
          (builtins.getAttr package super).overridePythonAttrs (old: if builtins.isList build-requirements then {
            buildInputs = (old.buildInputs or []) ++ (builtins.map (pkg: if builtins.isString pkg then builtins.getAttr pkg super else pkg) build-requirements);
          } else (build-requirements self super old))
        ) pypkgs-build-requirements)
      );
      manualOverrides = (self: super: {});

      yotta = pkgs.poetry2nix.mkPoetryApplication {
        projectDir = ./.;
        python = pkgs.python38;
        preferWheels = true;
        overrides =
            [ pkgs.poetry2nix.defaultPoetryOverrides autoOverrides manualOverrides ];
      };
      packageName = "yotta";
    in rec {
      packages = {
        "${packageName}" = yotta;
        default = yotta;
      };
      devShells = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [ poetry yotta ];
        };
      };
    }
  );
}
