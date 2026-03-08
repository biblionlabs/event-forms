{
  description = "Event Forms - Cloudflare Workers + Nuxt dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # FHS environment to run dynamically linked binaries like workerd
        fhsEnv = pkgs.buildFHSEnv {
          name = "event-forms-env";
          targetPkgs = pkgs: with pkgs; [
            # Core
            nodejs_22
            corepack_22

            # Libraries required by workerd
            glibc
            gcc-unwrapped.lib
            zlib
            openssl
            libuv
            stdenv.cc.cc.lib

            # Useful tools
            git
            curl
            jq
          ];
          runScript = "bash";
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            fhsEnv
          ];

          shellHook = ''
            echo "=========================================="
            echo " Event Forms - Dev Environment"
            echo "=========================================="
            echo ""
            echo "Entrando al entorno FHS para workerd..."
            echo "Usa 'exit' para salir del entorno FHS"
            echo ""
            exec event-forms-env
          '';
        };

        # Alternative: direct shell without FHS for basic tasks
        devShells.basic = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_22
            corepack_22
            git
          ];
        };
      }
    );
}
