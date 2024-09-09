{
    description = "A very basic flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
        inherit (nixpkgs) lib;
        forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in {
        packages = forAllSystems (system:
            let pkgs = nixpkgs.legacyPackages.${system}; in rec {
                default = shake;
                shake = pkgs.callPackage ./default.nix { };
        });
    };
}
