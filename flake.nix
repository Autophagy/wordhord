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
          bin = naersk-lib.buildPackage {
            root = ./wordhord;
            buildInputs = with pkgs; [ pkg-config openssl ];
            doCheck = true;
          };

          wordhord = pkgs.stdenv.mkDerivation {
            pname = "wordhord";
            inherit (bin) version;
            src = ./.;
            buildInputs = [ bin ];

            buildPhase = ''
              export BUILDDIR=$(pwd)
              export DRV=${placeholder "out"}
              cd $src
              ${bin}/bin/wordhord $src/config.dhall
            '';

            installPhase = ''
              mkdir -p $out
              cp -r $BUILDDIR/* $out
              cp -r static $out
            '';
          };

          default = self.packages.${system}.wordhord;
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy dhall openssl pkg-config ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
