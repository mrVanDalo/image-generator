{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt

    cairo

    jsonnet
    (pkgs.writers.writeBashBin "run" ''
      set -e
      set -o pipefail
      ${pkgs.jsonnet}/bin/jsonnet ${toString ./.}/sketch/example.jsonnet -o ./sketch/example.json
      ${pkgs.cargo}/bin/cargo run && ${pkgs.feh}/bin/feh file.png && rm file.png
    '')

    (pkgs.writers.writeBashBin "reformat" ''
      for file in `find ${toString ./.} -type f | egrep "\.rs$"`
      do
        ${pkgs.rustfmt}/bin/rustfmt "$file"
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
