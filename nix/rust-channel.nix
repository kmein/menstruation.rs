let
  mozilla = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozilla ]; };
  channel = pkgs.rustChannelOf { date = "2021-10-14"; channel = "nightly"; };
in channel.rust
