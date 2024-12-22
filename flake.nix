{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    nix-filter,
    ...
  }: let
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
    ];
    rustToolchain = pkgs.rust-bin.stable.latest.default;
    library_path = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" libraries;
    rust_platform = pkgs.makeRustPlatform {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
  in {
    packages.x86_64-linux = {
      default = rust_platform.buildRustPackage {
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

        buildTools = with pkgs; [
          rustToolchain
          pkg-config
          wayland
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          wayland
        ];
        #
        preConfigure =
          #bash
          ''
            export LD_LIBRARY_PATH="${library_path}"
            # export PKG_CONFIG_PATH="${pkgs.luajit}/pkgconfig/"
            export LOLMAN="${self}"
            export OKLOL="${./.}"
            export RUST_BACKTRACE=1
            echo ${self}
          '';
        buildPhase =
          #bash
          ''
            export LD_LIBRARY_PATH="${library_path}"
            export RUST_BACKTRACE=1
          '';
        #
        # unpackPhase =
        #   #bash
        #   ''
        #     cp -r ${./src} ${self}/src
        #   '';
        # configurePhase =
        #   #bash
        #   ''
        #   '';

        verbose = true;
        doCheck = true;
        cargoLock = {
          lockFile = ./Cargo.lock;
          allowBuiltinFetchGit = true;
        };
      };
    };

    # devShells.${system}.default =
  };
}
