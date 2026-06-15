# flake.nix
{
  description = "Runtime Secret Fetcher - Rust App";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "penv";
      version = "0.1.0";

      src = ./src;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };
  };
}
