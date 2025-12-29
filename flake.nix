{
  description = "Menstruation backend written in rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      lib = nixpkgs.lib;
      eachSupportedSystem = lib.genAttrs lib.systems.flakeExposed;
      pkgsFor =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
    in
    {
      devShell = eachSupportedSystem (
        system:
        let
          pkgs = pkgsFor system;
        in
        pkgs.mkShell {
          buildInputs = [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "rustc"
            ])
            pkgs.pkg-config
            pkgs.openssl
          ];
        }
      );
      defaultPackage = eachSupportedSystem (system: self.packages.${system}.menstruation-backend);
      packages = eachSupportedSystem (
        system:
        let
          pkgs = pkgsFor system;
          rustPlatform = pkgs.makeRustPlatform {
            rustc = pkgs.fenix.complete.withComponents [ "rustc" ];
            cargo = pkgs.fenix.complete.withComponents [ "cargo" ];
          };
        in
        {
          menstruation-backend = rustPlatform.buildRustPackage {
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
    };
}
