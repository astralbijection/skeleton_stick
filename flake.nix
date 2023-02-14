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
        packages.default = naersk-lib.buildPackage ./.;

        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              SDL2
              xorg.libX11
              xorg.libXcursor
              xorg.libXext
              xorg.libXrandr
              xorg.libXfixes
              xorg.libXi
              xorg.libXrender
              xorg.libXau
              xorg.libXdmcp
              xorg.libXScrnSaver
              xorg.libxcb
            ];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
            LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
