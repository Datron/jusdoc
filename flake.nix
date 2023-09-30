{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-unstable }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        pkgsu = (import nixpkgs-unstable) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          nativeBuildInputs = with pkgs; [ postgresql_12 ];
          src = ./.;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          # bring your local shell properties into nix env
          # shellHook = ''
          #   echo "you are now in the nix shell"
          #   eval $($SHELL)
          #   '';
          nativeBuildInputs =
            let
              univPkgs = with pkgs; [
                  # Build requirements
                  pkgsu.rustc
                  cargo
                  libiconv
                  # Extras
                  rust-analyzer
                  rustfmt
                  bacon
                  cargo-watch
                  clippy
                ];
              darwinPkgs = with pkgs; [
                  darwin.apple_sdk.frameworks.Security
                ];
            in
              univPkgs ++  (if pkgs.stdenv.isDarwin then darwinPkgs else []);
        };
      }
    );
}
