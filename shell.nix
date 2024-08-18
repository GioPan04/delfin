{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs.buildPackages; [ libadwaita glib mpv epoxy openssl libglvnd ];
    nativeBuildInputs =  with pkgs; [ pkg-config mold meson ninja cargo rustc git appstream desktop-file-utils rustfmt clippy ];
}
