{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages = rec {
          wordhord = naersk-lib.buildPackage {
            root = ./wordhord;
            buildInputs = with pkgs; [ pkg-config openssl ];
          };

          site = pkgs.stdenv.mkDerivation {
            pname = "site";
            inherit (wordhord) version;
            src = ./.;
            buildInputs = [ wordhord ];

            buildPhase = ''
              export BUILDDIR=$(pwd)
              export DRV=${placeholder "out"}
              cd $src
              ${wordhord}/bin/wordhord
            '';

            installPhase = ''
              mkdir -p $out
              cp -r $BUILDDIR/* $out
              cp -r static $out
            '';
          };

          default = self.packages.${system}.site;
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy dhall openssl pkg-config ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
