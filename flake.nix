{
  description = "A flake for building a Rust workspace using cargo2nix.";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        code = pkgs.callPackage ./. { inherit nixpkgs system cargo2nix; };
      in rec {
        packages = {
          hello-world = code.hello-world;
#          wasm = code.wasm;
          all = pkgs.symlinkJoin {
            name = "all";
            paths = with code; [ hello-world ];
          };
          default = packages.all;
        };

        devShells = {
          default = code.rustPkgs.workspaceShell {
            packages = [
              pkgs.rust-analyzer
              pkgs.cargo
              pkgs.rustc
              pkgs.evcxr
            ];
          };
        };
      }
    );
}
