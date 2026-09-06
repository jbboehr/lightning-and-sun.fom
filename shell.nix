let
  lock = builtins.fromJSON (builtins.readFile ./flake.lock);
  fetchLockedSource = name:
    let
      locked = lock.nodes.${name}.locked;
    in
    builtins.fetchTarball {
      url = "https://github.com/${locked.owner}/${locked.repo}/archive/${locked.rev}.tar.gz";
      sha256 = locked.narHash;
    };
in
{ pkgs ? import (fetchLockedSource "nixpkgs") { }
, fenixPkgs ? import (fetchLockedSource "fenix") { inherit pkgs; }
}:

let
  toolchain = fenixPkgs.stable.withComponents [
    "cargo"
    "clippy"
    "rust-analyzer"
    "rust-src"
    "rustc"
    "rustfmt"
  ];
in
pkgs.mkShell {
  packages = [
    toolchain
  ];
  RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
}
