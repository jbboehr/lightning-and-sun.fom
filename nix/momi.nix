{ pkgs }:

let
  download = pkgs.fetchurl {
    url = "https://github.com/Garethp/Mods-of-Mistria-Installer/releases/download/v0.16.4/ModsOfMistriaInstaller-cli-linux";
    sha256 = "0c36b78dd1ee964c604f3b54d9669a962bbbed1078ae07e0be8d7abc9f93895c";
  };
  binary = pkgs.runCommand "momi-cli-0.16.4" { } ''
    install -m755 ${download} "$out"
  '';
  libraries = pkgs.lib.makeLibraryPath [
    pkgs.stdenv.cc.cc.lib pkgs.zlib pkgs.openssl pkgs.icu
  ];
  runner = pkgs.writeShellScript "mistria-momi-runner" ''
    set -eu
    # Only this build directory is visible as a game installation. MOMI has no
    # install-path flag and otherwise searches Steam before its current directory.
    exec ${pkgs.bubblewrap}/bin/bwrap --unshare-all --die-with-parent \
      --ro-bind /nix /nix --ro-bind /etc /etc \
      --ro-bind ${pkgs.stdenv.cc.bintools.dynamicLinker} /lib64/ld-linux-x86-64.so.2 \
      --ro-bind "$1" /tools/momi \
      --bind "$2" /game \
      --proc /proc --dev /dev --tmpfs /tmp --dir /home/sandbox --chdir /game \
      --clearenv \
      --setenv LD_LIBRARY_PATH '${libraries}' \
      --setenv DOTNET_BUNDLE_EXTRACT_BASE_DIR /tmp/dotnet \
      --setenv EXIT_ON_COMPLETE true \
      --setenv MOMI_GAME_CONFIG_DIR /game/config \
      /tools/momi --strict-lints --fail-on-skip --compile-check require
  '';
in
{
  inherit binary runner;
}
