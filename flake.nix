{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    nix-filter.url = "github:numtide/nix-filter";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    nix-filter,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      libraries = with pkgs; [
        luajit
        wayland
        pkg-config
        libGL
        dbus
        expat
        fontconfig
        freetype
        libxkbcommon
        lua-language-server
        # openssl
      ];
      rustToolchain = pkgs.rust-bin.beta.latest.default; # beta required due to anyhow requiring cargo above 1.83
      library_path = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" libraries;
      rust_platform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
      astrum_package = rust_platform.buildRustPackage {
        pname = "astrum";
        version = "0.1";
        # src = ./.;
        src = nix-filter.lib.filter {
          # root = ~/Documents/Programming/astrum_unstable;
          root = self;
          include = [
            "src"
            "src/astrum_binds"
            "src/astrum_core"
            "src/lua_library"

            ./src
            ./src/astrum_binds
            ./src/astrum_core
            ./src/lua_library
            ./Cargo.lock
            ./Cargo.toml
          ];
        };
        buildInputs = libraries;

        nativeBuildInputs = with pkgs; [
          pkg-config
          makeBinaryWrapper
          lua-language-server
        ];

        RUSTFLAGS = map (a: "-C link-arg=${a}") [
          "-Wl,--push-state,--no-as-needed"
          "-lEGL"
          "-lwayland-client"
          "-Wl,--pop-state"
          "--release"
        ];

        postInstall =
          #bash
          ''
            wrapProgram "$out/bin/astrum"\
              --prefix CARGO_MANIFEST_DIR : "${self}"\
              --prefix LD_LIBRARY_PATH : ${
              pkgs.lib.makeLibraryPath (with pkgs; [
                libxkbcommon
                vulkan-loader
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
              ])
            }
          '';

        verbose = true;
        doCheck = false;
        cargoLock = {
          lockFile = ./Cargo.lock;
          allowBuiltinFetchGit = true;
        };
      };
    in {
      packages.astrum = astrum_package;

      defaultPackage = self.packages.${system}.astrum;

      devShells.default = pkgs.mkShell {
        buildInputs = libraries;

        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [
          wayland
          libGL
          libxkbcommon
        ])}";
      };
    });
}
