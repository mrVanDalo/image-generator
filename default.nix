{ rustPlatform, fetchgit, stdenv, cairo, ... }:

rustPlatform.buildRustPackage rec {
  name = "image-geneartor-${version}";
  version = "2.0.0";
  # src = ./.;
  src = fetchgit {
    url = "https://git.ingolf-wagner.de/palo/image-generator2";
    rev = "c9ef48b1bd73a819a5fb86095774472f35d7b576";
    sha256 = "0pdhw2calqxczdrz66wyvydd5d6m2fx7xm64z1l25nazdad930p5";
  };
  cargoSha256 = "07pwds279qc54g5fza805ah2m8jhrdzng7vb1bi24a9ra9ci8s29";
  verifyCargoDeps = true;

  buildInputs = [ cairo ];

  meta = with stdenv.lib; {
    description =
      "An image generator unsing entropy and a JSON as configuration.";
    homepage = "https://git.ingolf-wagner.de/palo/image-generator2";
    license = licenses.gplv3;
    maintainers = [ maintainers.mrVanDalo ];
    platforms = platforms.all;
  };
}

