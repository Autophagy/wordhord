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
        src = ./.;
      in
      {
        packages = rec {
          bin = naersk-lib.buildPackage {
            root = src;
            buildInputs = with pkgs; [ pkg-config openssl ];
          };

          wordhord = pkgs.stdenv.mkDerivation {
            pname = "wordhord";
            inherit (bin) version;
            inherit src;
            buildInputs = [ bin ];

            buildPhase = ''
              export BUILDDIR=$(pwd)
              cd $src
              ${bin}/bin/wordhord
            '';

            installPhase = ''
              mkdir -p $out
              cp -r $BUILDDIR/* $out
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
