{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      nixpkgs-mozilla,
      flake-utils,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        naersk' = pkgs.callPackage naersk { };

        toolchain =
          (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "sha256-Qxt8XAuaUR2OMdKbN4u8dBJOhSHxS+uS06Wl9+flVEk=";
            #        ^ Update this, in case if the toolchain file has changed. You can get it by
            #          running `nix build`. The hash will appear in the error message.
          }).rust;
      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;

          cargo = toolchain;
          rustc = toolchain;
        };

        apps.default = {
          type = "app";
          program = "${defaultPackage}/bin/tohum";
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];

          nativeBuildInputs = with pkgs; [
            rustc
            cargo
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}
