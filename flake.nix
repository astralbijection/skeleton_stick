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
      in {
        packages = {
          skstctl = naersk-lib.buildPackage {
            src = ./skstctl;
            root = ./.;
          };
          skstd = naersk-lib.buildPackage {
            src = ./skstd;
            root = ./.;
          };
        };

        apps = {
          skstctl = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };
        };

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustPackages.clippy
              rustc
              rustfmt

              python3
              python39Packages.virtualenv
              python39Packages.pip

              nixfmt
              pre-commit
              texlive.combined.scheme-full
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
