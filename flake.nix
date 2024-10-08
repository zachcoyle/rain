{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix.url = "github:nix-community/fenix";
  };
  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [];
      perSystem = {pkgs, ...}: let
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          openssl
          stdenv.cc.cc.lib
          fontconfig
          freetype
          wayland
          xorg.libX11
          libxkbcommon
          libGL
        ]);

        rain' = {
          pkgs,
          lib,
          gn,
          makeRustPlatform,
          clangStdenv,
          ninja,
          fetchFromGitHub,
          linkFarm,
          fetchgit,
          runCommand,
          freetype,
          fontconfig,
          libGL,
          ...
        }: let
          inherit (inputs.fenix.packages.${pkgs.system}.minimal) toolchain;
          rustPlatform = makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };

          src = ./.;
          srcMigrations = src + /migrations;

          sqlx-db =
            runCommand "sqlx-db-prepare"
            {nativeBuildInputs = [pkgs.sqlx-cli];}
            ''
              mkdir $out
              export DATABASE_URL=sqlite:$out/db.sqlite3
              sqlx database create
              sqlx migrate run --source ${srcMigrations}
            '';
        in
          rustPlatform.buildRustPackage.override {stdenv = clangStdenv;} {
            pname = "rain";
            version = "unstable";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "morphorm-0.6.4" = "sha256-JZ49mB44q/EQbNMdflcnJVNjbnY0dg6+gAjVX4mDhJg=";
                "selectors-0.23.0" = "sha256-9nD2YY9Z9YDrQqy99T02FCC5Q7oGjJamPP/ciTmCkUc=";
              };
            };

            SKIA_SOURCE_DIR = let
              repo = fetchFromGitHub {
                owner = "rust-skia";
                repo = "skia";
                # see rust-skia:skia-bindings/Cargo.toml#package.metadata skia
                rev = "m126-0.74.2";
                hash = "sha256-4l6ekAJy+pG27hBGT6A6LLRwbsyKinJf6PP6mMHwaAs=";
              };
              # The externals for skia are taken from skia/DEPS
              externals = linkFarm "skia-externals" (
                lib.mapAttrsToList (name: value: {
                  inherit name;
                  path = fetchgit value;
                }) (lib.importJSON ./skia-externals.json)
              );
            in
              runCommand "source" {} ''
                cp -R ${repo} $out
                chmod -R +w $out
                ln -s ${externals} $out/third_party/externals
              '';
            SKIA_GN_COMMAND = "${gn}/bin/gn";
            SKIA_NINJA_COMMAND = "${ninja}/bin/ninja";

            buildInputs = with pkgs; [
              openssl
              rustPlatform.bindgenHook
              freetype
              fontconfig
              wayland
              libGL
            ];

            nativeBuildInputs = with pkgs; [
              python3
              pkg-config
            ];

            # disallowedReferences = [SKIA_SOURCE_DIR];

            # INFO: https://ipetkov.dev/blog/building-with-sqlx-on-nix/
            # # FIXME: this needs modified for rustPlatform.buildRustPackage
            linkDb = ''
              export DATABASE_URL=sqlite:${sqlx-db}/db.sqlite3
            '';

            preBuildPhases = ["linkDb"];

            inherit LD_LIBRARY_PATH;
          };

        rain = pkgs.callPackage rain' {};
      in {
        packages.default = rain;

        devShells.default = pkgs.mkShell.override {stdenv = pkgs.clangStdenv;} {
          buildInputs = with pkgs; [sqlx-cli];
          inputsFrom = [rain];
          inherit LD_LIBRARY_PATH;
          SKIA_GN_COMMAND = pkgs.lib.getExe pkgs.gn;
          SKIA_NINJA_COMMAND = pkgs.lib.getExe pkgs.ninja;
        };
      };
    };
}
