{
  description = "Fly.io's distributed systems challenges";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        jepsen-maelstrom-overlay = self: super: {
          jepsen-maelstrom = self.stdenv.mkDerivation rec {
            name = "maelstrom";
            version = "0.2.3";
            src = builtins.fetchurl {
              url = "https://github.com/jepsen-io/maelstrom/releases/download/v${version}/maelstrom.tar.bz2";
              sha256 = "sha256:06jnr113nbnyl9yjrgxmxdjq6jifsjdjrwg0ymrx5pxmcsmbc911";
            };
            installPhase = ''
              mkdir -p $out/bin
              cp -r lib $out/bin/lib
              install -m755 -D maelstrom $out/bin/maelstrom
            '';
          };
        };

        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            jepsen-maelstrom-overlay
            (import rust-overlay)
          ];
        };
      in {
        devShell = pkgs.mkShell {
          name = "gossip-glomers";
          packages = with pkgs; [
            go
            openjdk
            graphviz
            gnuplot
            jepsen-maelstrom

            pkg-config
            rust-bin.stable.latest.default
          ];
        };
      }
    );
}
