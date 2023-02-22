{
  description = "Menstruation backend written in rust";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (self: super: let myRust = self.rust-bin.nightly."2023-02-03"; in {
                rustc = myRust.default;
                cargo = myRust.default;
            })
          ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = pkgs.rustc;
          cargo = pkgs.cargo;
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [ pkgs.rustc pkgs.cargo pkgs.pkg-config pkgs.openssl ];
        };
        defaultPackage = self.packages.${system}.menstruation-backend;
        packages.menstruation-backend = rustPlatform.buildRustPackage {
          pname = "menstruation";
          version = "0.1.0";
          src = ./.;
          cargoSha256 = "sha256-CAxvNta6Jap9zciP6YN6ebcwhqRtd43PhoRJf/PtufQ=";
          nativeBuildInputs = [ pkgs.pkg-config ];
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
