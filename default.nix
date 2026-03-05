{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/cf59864ef8aa2e17.tar.gz") {},
  fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/9ba6d89.tar.gz") {},
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
    rust
    go
    ui-server
  ];

  shellHook =  ''
    export TMPSCRIPTS=$(mktemp -d)
    export PATH="$PWD/scripts:$TMPSCRIPTS:$PATH"
    trap "rm -rf $TMPSCRIPTS" EXIT
  '';
}    
