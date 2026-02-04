{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    {
      nix.settings = {
        substituters = ["https://wrangler.cachix.org"];
        trusted-public-keys = ["wrangler.cachix.org-1:N/FIcG2qBQcolSpklb2IMDbsfjZKWg+ctxx0mSMXdSs="];
      };
    }
    //
    # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system:
        import ./. rec {
          inherit system flake-utils;
          pkgs = import nixpkgs {inherit system;};
          crane = inputs.crane.mkLib pkgs;
          fenix = inputs.fenix.packages.${system};
        }
    );
}
