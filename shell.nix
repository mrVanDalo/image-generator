{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt

    cairo

    jsonnet
    (pkgs.writers.writeBashBin "run" ''
      input="$1"
      set -e
      set -o pipefail
      ${pkgs.jsonnet}/bin/jsonnet "$input" -o ${toString ./.}/.example.json
      ${pkgs.cargo}/bin/cargo run -- --output ${toString ./.}/.example.png ${toString ./.}/.example.json
      ${pkgs.feh}/bin/feh ${toString ./.}/.example.png
      rm ${toString ./.}/.example.png
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
