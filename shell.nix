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

    dhall
    dhall-json

    jsonnet

    (pkgs.writers.writeBashBin "reformat" ''
      for file in `find ${toString ./.} -type f | egrep "\.rs$"`
      do
        ${pkgs.rustfmt}/bin/rustfmt "$file"
      done

      for file in `find ${toString ./.} -type f | egrep "\.dhall$"`
      do
        tmp_file=`mktemp`
        cat "$file" | ${pkgs.dhall}/bin/dhall format > $tmp_file
        mv $tmp_file "$file"
      done

      for file in `find ${toString ./.} -type f | egrep "\.jsonnet$"`
      do
        ${pkgs.jsonnet}/bin/jsonnetfmt --in-place "$file"
      done

      for file in `find ${toString ./.} -type f | egrep "\.libsonnet$"`
      do
        ${pkgs.jsonnet}/bin/jsonnetfmt --in-place "$file"
      done
    '')
  ];

}
