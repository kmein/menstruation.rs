{
  description = "Menstruation backend written in rust";

  inputs = {
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = pkgs.fenix.complete.withComponents [ "rustc" ];
          cargo = pkgs.fenix.complete.withComponents [ "cargo" ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "rustc"
            ])
            pkgs.pkg-config
            pkgs.openssl
          ];
        };
        defaultPackage = self.packages.${system}.menstruation-backend;
        packages.menstruation-backend = rustPlatform.buildRustPackage {
          pname = "menstruation";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = builtins.path {
              path = self + "/Cargo.lock";
              name = "Cargo.lock";
            };
            allowBuiltinFetchGit = true;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          meta = with pkgs.lib; {
            homepage = "https://github.com/kmein/menstruation.rs";
            license = licenses.gpl3;
            platforms = platforms.all;
            maintainers = [ maintainers.kmein ];
          };
        };
      }
    );
}
