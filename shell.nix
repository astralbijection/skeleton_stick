{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
  buildInputs = with pkgs; [
    python3
    python39Packages.virtualenv
    python39Packages.pip
    texlive.combined.scheme-full
  ];
}
