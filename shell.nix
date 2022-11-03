{ pkgs ? import <nixpkgs> { } }:

let
  # Rolling updates, not deterministic.
  pkgs =
    import (fetchTarball ("channel:nixpkgs-unstable")) { inherit overlays; };

  overlays = [
    (self: super: {
      nodePackages.pnpm =
        super.nodePackages.pnpm.override { nodejs = pkgs.nodejs-18_x; };
    })
  ];
in pkgs.mkShell {
  name = "nix-shell";

  packages = with pkgs; [ bashInteractive nodejs nodePackages.pnpm ];
}
