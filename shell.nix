let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
pkgs.mkShell {
  packages = [
    pkgs.git
    pkgs.cacert
    pkgs.python311
    pkgs.pdm
    pkgs.gcc14
    pkgs.rustup
  ];
}
