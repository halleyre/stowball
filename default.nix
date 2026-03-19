{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/d24777934d.tar.gz") {},
  fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/6dfabfd.tar.gz")
                 { inherit pkgs; },
}:
with pkgs; let

  rust = (with fenix; with latest; combine [
      rustc
      cargo
      rust-src
      targets.wasm32-unknown-unknown.latest.rust-std
  ]);

  ui-server = buildGoModule {
    pname = "ui-server";
    src = ./servers/ui;
    version = "0.1.0";
    vendorHash = null;
  };

  wasm-bindgen = wasm-bindgen-cli.overrideAttrs (attrs: rec {
    version = "0.2.114";
    src = fetchCrate { inherit version;
      pname = "wasm-bindgen-cli";
      sha256 = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
    };
    cargoDeps = rustPlatform.importCargoLock {
      lockFile = "${src}/Cargo.lock";
    };
  });

in
mkShell {
  packages = [
    go
    rust
    ui-server

    # dev runtime dependencies
    libxkbcommon
    libGL
    wayland
    vulkan-loader

    wasm-bindgen
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath [
    libxkbcommon
    libGL
    wayland
    vulkan-loader
  ];

  shellHook =  ''
    export TMPSCRIPTS=$(mktemp -d)
    export PATH="$PWD/scripts:$TMPSCRIPTS:$PATH"
    trap "rm -rf $TMPSCRIPTS" EXIT
  '';
}    
