{ rustPlatform, fetchgit, stdenv, cairo, ... }:

rustPlatform.buildRustPackage rec {
  name = "image-geneartor-${version}";
  version = "3.0.0";
  # src = ./.;
  src = fetchgit{
    url = "https://github.com/mrVanDalo/image-generator";
    rev = "f8c1bf958aeea1808df6baea62eeb2949aa5fe65";
    sha256 = "1swa2i57sdag3zapzx3m9mdarci0xfjczr611320zampw505ai09";
  };

  cargoSha256 = "07pwds279qc54g5fza805ah2m8jhrdzng7vb1bi24a9ra9ci8s29";
  verifyCargoDeps = true;

  buildInputs = [ cairo ];

  meta = with stdenv.lib; {
    description =
      "An image generator unsing entropy and a JSON as configuration.";
    homepage = "https://github.com/mrVanDalo/image-generator";
    license = licenses.gplv3;
    maintainers = [ maintainers.mrVanDalo ];
    platforms = platforms.all;
  };
}

