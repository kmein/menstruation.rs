with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "menstruation-rs";
  buildInputs = with pkgs; [ openssl pkgconfig ];
}
