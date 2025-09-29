{
    description = "A simple rust tool to set up a postgres database from evironment variables";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    };

    outputs = { self, nixpkgs, ... }:
    let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
    in {
        devShells.${system}.default = pkgs.mkShell {
            nativeBuildInputs = [ pkgs.rustPlatform.rustc pkgs.rustPlatform.cargo ];
        };
        packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "pg-setup-from-env";
            version = "1.0.0";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
        };
    };
}