{ pkgs ? import <nixpkgs> { } }:
let

  #taskwarrior-hooks = import ./default.nix {
  #  inherit (pkgs) fetchFromGitHub stdenv rustPlatform;
  #};

in pkgs.mkShell {

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    # taskwarrior-hooks

    cairo

    (pkgs.writers.writeBashBin "reformat" ''
      for file in `find ${toString ./.} -type f | egrep "\.rs"`
      do
        ${pkgs.rustfmt}/bin/rustfmt "$file"
      done
    '')
  ];

}
