{ rustPlatform, fetchgit, stdenv, cairo, ... }:

rustPlatform.buildRustPackage rec {
  name = "image-geneartor-${version}";
  version = "2.0.0";
  # src = ./.;
  src = fetchgit {
    url = "https://git.ingolf-wagner.de/palo/image-generator2";
    rev = "78cdc84a90cd818561a70bf1868ed3875d1e2204";
    sha256 = "1r0splgsfc2sf014s71r91272icdqgfkxsp62k2jav0spwhi7sqy";
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

