{
  description = "simple rust based concrete json to RDF converter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.05";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, nci, ... }:
    nci.lib.makeOutputs {
      # Documentation and examples:
      # https://github.com/yusdacra/rust-nix-templater/blob/master/template/flake.nix
      root = ./.;
      overrides = {
        shell = common: prev: {
          packages = with common.pkgs; prev.packages ++ [
            rust-analyzer
            cargo-watch
            cargo-edit
          ];
        };
      };
    };
}
