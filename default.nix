{ rustPlatform, fetchFromGitHub, stdenv, ... }:

rustPlatform.buildRustPackage rec {
  name = "image-geneartor-${version}";
  version = "2.0.0";
  src = ./.;
  #src = fetchFromGitHub {
  #  owner = "mrVanDalo";
  #  repo = "taskwarrior-hooks";
  #  rev = "${version}";
  #  sha256 = "1mj0k6ykac332667315kqrvg37j8r8078g48nafv7ini6lw8djas";
  #};

  cargoSha256 = "1ijnh2ank9slmfglw4yhnycl11x26m94m2hiq3hcasmbs6c39zj5";
  verifyCargoDeps = true;

  meta = with stdenv.lib; {
    description =
      "An image generator unsing entropy and a JSON as configuration.";
    homepage = "https://git.ingolf-wagner.de/palo/image-generator2";
    license = licenses.gplv3;
    maintainers = [ maintainers.mrVanDalo ];
    platforms = platforms.all;
  };
}

