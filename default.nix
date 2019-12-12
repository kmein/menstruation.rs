{ mozilla ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)
, pkgs ?  import <nixpkgs> { overlays = [ mozilla ]; }
}:
let
  nightly = pkgs.rustChannelOf { date = "2019-06-20"; channel = "nightly"; };
  nightly-rustPlatform = pkgs.makeRustPlatform {
    rustc = nightly.rust;
    cargo = nightly.rust;
  };
in with pkgs; nightly-rustPlatform.buildRustPackage rec {
  name = "menstruation-rs";

  src = ./.;

  nativeBuildInputs = [ pkgconfig ] ++ lib.optional (builtins.currentSystem == "x86_64-darwin") darwin.apple_sdk.frameworks.Security;

  preConfigure = "export HOME=$(mktemp -d)";

  cargoSha256 = "0r1rj9pb0ln95g6266afr43nygfnh891mqyh7mzk4n4y45vb5dv3";

  buildInputs = [ openssl ];

  meta = with stdenv.lib; {
    homepage = "https://github.com/kmein/menstruation.rs";
    license = licenses.gpl3;
    platforms = platforms.all;
    maintainers = [ maintainers.kmein ];
  };
}
