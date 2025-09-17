# SPDX-FileCopyrightText: 2024 Gergely Nagy
# SPDX-FileContributor: Gergely Nagy
#
# SPDX-License-Identifier: EUPL-1.2
# SOURCE: https://git.madhouse-project.org/algernon/avalanche/src/branch/main/packs/emacs/packages/makeQuartz.nix

{ pkgs, ... }:
{ pname, markdown, project, }:
pkgs.buildNpmPackage rec {
  inherit pname;
  version = "v4.5.1";

  src = pkgs.fetchFromGitHub {
    owner = "jackyzha0";
    repo = "quartz";
    rev = "7fa9253abc1e4056d425847e2eaa5a8e107fc297";
    hash = "";
  };

  npmDepsHash = "";
  dontNpmBuild = true;

  outputs = [ "out" "public" ];

  installPhase = ''
    runHook preInstall
    npmInstallHook
    cd $out/lib/node_modules/@jackyzha0/quartz
    $out/bin/quartz build --verbose --directory ${markdown}
    mv ./public $public
    runHook postInstall
  '';
}
