{ pkgs ?  import <nixpkgs> }:
let
  channel = import nix/rust-channel.nix;
in pkgs.mkShell {
  buildInputs = [ channel pkgs.pkg-config pkgs.openssl ];
}
