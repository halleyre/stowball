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
in
mkShell {
  packages = [
    go
    rust
    ui-server

    wasm-pack
  ];

  shellHook =  ''
    export TMPSCRIPTS=$(mktemp -d)
    export PATH="$PWD/scripts:$TMPSCRIPTS:$PATH"
    trap "rm -rf $TMPSCRIPTS" EXIT
  '';
}    
