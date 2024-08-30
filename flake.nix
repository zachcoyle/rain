{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildInputs = with pkgs; [
          openssl
          libGL
          libxkbcommon
          vulkan-loader
          wayland
          wayland-protocols
        ];

        rain = crane.lib.${system}.buildPackage {
          src = ./.;
          inherit buildInputs;

          nativeBuildInputs = with pkgs; [
            gtk-layer-shell
            gtk3
            pkg-config
          ];
        };
      in
      {
        checks = {
          my-crate = rain;
        };

        packages = {
          inherit rain;
          default = rain;
        };

        apps.default = flake-utils.lib.mkApp { drv = rain; };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            openssl
            pkg-config
            sqlx-cli
          ];

          inputsFrom = builtins.attrValues self.checks;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          # Extra inputs can be added here
        };
      }
    );
}
