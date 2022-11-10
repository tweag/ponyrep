{
  description = "A flake for building ponyrep.";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
          ponyrep = pkgs.rustPlatform.buildRustPackage {
            pname = "ponyrep";
            version = "0.0.1";
            src = ./.;
            cargoBuildFlags = "-p ponyrep";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        in rec {
          defaultPackage = ponyrep;
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              clang
              rustfmt
            ];
          };
        }
      );
}
