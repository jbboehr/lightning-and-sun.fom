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
  momi = import ./nix/momi.nix { inherit pkgs; };
in
pkgs.mkShell {
  packages = [
    toolchain
  ];
  RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
  MISTRIA_MOMI_RUNNER = momi.runner;
  MISTRIA_MOMI_BINARY = if pkgs.stdenv.hostPlatform.system == "x86_64-linux" then momi.binary else "";
}
