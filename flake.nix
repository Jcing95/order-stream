{
  description = "OrderStream — Leptos + Axum + Postgres dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        rustToolchain = with fenix.packages.${system}; combine [
          complete.cargo
          complete.rustc
          complete.rust-src
          complete.rustfmt
          complete.clippy
          complete.rust-analyzer
          targets.wasm32-unknown-unknown.latest.rust-std
        ];

        dieselCli = pkgs.diesel-cli.override {
          sqliteSupport = false;
          mysqlSupport = false;
          postgresqlSupport = true;
        };

        wasmBindgenVersion = "0.2.120";
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-leptos
            pkgs.cargo-binstall
            pkgs.tailwindcss_4
            dieselCli
            pkgs.docker-compose
            pkgs.pkg-config
            pkgs.openssl
            pkgs.postgresql.lib
          ];

          env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.postgresql.lib
            pkgs.openssl
          ];

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
            need_install=1
            if command -v wasm-bindgen >/dev/null 2>&1; then
              current="$(wasm-bindgen --version 2>/dev/null | awk '{print $2}')"
              [ "$current" = "${wasmBindgenVersion}" ] && need_install=0
            fi
            if [ "$need_install" = 1 ]; then
              echo "Installing wasm-bindgen-cli ${wasmBindgenVersion} via cargo-binstall..."
              cargo binstall -y --force --version ${wasmBindgenVersion} wasm-bindgen-cli
            fi
          '';
        };
      });
}
