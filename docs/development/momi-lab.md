# Isolated MOMI experiment on NixOS

This procedure exposes a copied archive to MOMI through bubblewrap. Its mount
namespace has no Steam installation or real save directory. Only `tmp/momi-lab`
is writable outside temporary sandbox storage. Run from the repository root after
building `generated/momi-adeline-stylized`. Use a new lab directory if it already
exists. Reserve at least 2 GB of disk space for the tool and archive copies.

The commands below use the Nixpkgs revision used for the installer experiment.
The Rust dev shell has its own lock file. No .NET SDK or Wine is needed.

```sh
set -eu
MOMI_ROOT="$PWD"
MOMI_PKGS=github:NixOS/nixpkgs/a3116115851d68b8952a2a4221cc25a84e56b532

mkdir -p tmp/tools
curl -fL \
  https://github.com/Garethp/Mods-of-Mistria-Installer/releases/download/v0.15.10/ModsOfMistriaInstaller-cli-linux \
  -o tmp/tools/ModsOfMistriaInstaller-cli-linux
printf '%s\n' 'a0a068c4f0b8f4f85d801fc2aa2b6611f0e172c0d7a629cbec5713d6ab235a12  tmp/tools/ModsOfMistriaInstaller-cli-linux' | sha256sum -c -
chmod u+x tmp/tools/ModsOfMistriaInstaller-cli-linux

mkdir tmp/momi-lab
cp tmp/fields-of-mistria/assets.zip tmp/fields-of-mistria/Maybe.toml tmp/momi-lab/
mkdir tmp/momi-lab/mods
cp -R generated/momi-adeline-stylized tmp/momi-lab/mods/adeline-palette-study

nix build --no-link "$MOMI_PKGS#bubblewrap" "$MOMI_PKGS#glibc" \
  "$MOMI_PKGS#stdenv.cc.cc.lib" "$MOMI_PKGS#zlib" "$MOMI_PKGS#openssl" "$MOMI_PKGS#icu"
MOMI_BWRAP="$(nix eval --raw "$MOMI_PKGS#bubblewrap.outPath")/bin/bwrap"
MOMI_LOADER=$(nix eval --raw "$MOMI_PKGS#stdenv.cc.bintools.dynamicLinker")
MOMI_LIBS=$(nix eval --raw --impure --expr "let
  pkgs = (builtins.getFlake \"$MOMI_PKGS\").legacyPackages.x86_64-linux;
  in pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib pkgs.zlib pkgs.openssl pkgs.icu ]")
test -x "$MOMI_BWRAP"
test -x "$MOMI_LOADER"

run_momi_lab() {
  "$MOMI_BWRAP" --unshare-all --die-with-parent \
    --ro-bind /nix /nix --ro-bind /etc /etc \
    --ro-bind "$MOMI_LOADER" /lib64/ld-linux-x86-64.so.2 \
    --ro-bind "$MOMI_ROOT/tmp/tools" /tools \
    --bind "$MOMI_ROOT/tmp/momi-lab" /lab \
    --proc /proc --dev /dev --tmpfs /tmp --dir /home/sandbox --chdir /lab \
    --setenv LD_LIBRARY_PATH "$MOMI_LIBS" \
    --setenv DOTNET_BUNDLE_EXTRACT_BASE_DIR /tmp/dotnet \
    --setenv EXIT_ON_COMPLETE true \
    --setenv MOMI_GAME_CONFIG_DIR /lab/config \
    /tools/ModsOfMistriaInstaller-cli-linux "$@"
}

run_momi_lab --version
run_momi_lab > tmp/momi-install.log 2>&1
```

Confirm the log says `Guessed Location: /lab`, names the selected portrait, and
reports one installed mod. A successful process exit alone is insufficient,
because MOMI can skip a replacement. The SHA-256 above matches the release asset's
GitHub API digest. Directly invoking Nix's dynamic loader on this bundled .NET app
produced a misleading bundle-corruption error. Giving the executable its expected
loader path inside the namespace worked.

## Check the atlas and preserved metadata

Run this before uninstalling. It reconstructs both frames from their trimmed atlas
rectangles, checks exact RGBA equality with the generated strip, and checks that
other spring atlas placements are unchanged. It also verifies that no unexpected
archive member was introduced. All paths here are local ignored files.

```sh
nix-shell --pure --run 'cargo test --locked --test momi_install -- --ignored --nocapture'
```

## Uninstall and verify restoration

Use the same sandbox function. This removes all MOMI mods from the lab copy:

```sh
run_momi_lab --uninstall > tmp/momi-uninstall.log 2>&1
sha256sum tmp/fields-of-mistria/assets.zip tmp/momi-lab/assets.zip tmp/momi-lab/assets.bak.zip
cmp tmp/fields-of-mistria/assets.zip tmp/momi-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/momi-lab/assets.bak.zip
```

All three hashes must equal the recorded original fingerprint. Keep the generated
mod and comparison sheet for review. The game itself has not been launched by
this procedure. A final manual smoke test should use a copied install and disposable
save, inspect this spring portrait in dialogue, then check vanilla after removal.
