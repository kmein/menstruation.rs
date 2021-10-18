{ pkgs ?  import <nixpkgs> }:
let
  channel = import nix/rust-channel.nix;
  platform = pkgs.makeRustPlatform { rustc = channel; cargo = channel; };
in platform.buildRustPackage {
  name = "menstruation";
  src = ./.;

  buildInputs = [ pkgs.openssl ];
  nativeBuildInputs = [ pkgs.pkg-config ];

  cargoSha256 = "17x4c9sjbxpi81nrd9w1448mq6z8220hhar32vjq3d668n2842gh";
}
