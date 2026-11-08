{ pkgs, fenixPkgs }:

let
  toolchain = fenixPkgs.stable.withComponents [ "cargo" "rustc" ];
  rustPlatform = pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
  momi = import ./momi.nix { inherit pkgs; };
  root = ../.;
  source = pkgs.lib.cleanSourceWith {
    src = root;
    filter = path: type:
      let
        relative = pkgs.lib.removePrefix "${toString root}/" (toString path);
        top = builtins.head (pkgs.lib.splitString "/" relative);
        hasExtension = extensions: builtins.any (ext: pkgs.lib.hasSuffix ext relative) extensions;
      in
      if type == "directory" then
        builtins.elem top [ "src" "tests" "palettes" "mod" ]
        || builtins.elem relative [ "docs" "docs/development" ]
      else type == "regular" && (
        builtins.elem relative [ "Cargo.toml" "Cargo.lock" ]
        || (top == "src" && hasExtension [ ".rs" ".html" ".js" ".css" ])
        || (top == "tests" && hasExtension [ ".rs" ".js" ".json" ".gml" ])
        || (top == "palettes" && hasExtension [ ".json" ])
        || builtins.elem relative [
          "docs/development/player27-mappings.json"
          "docs/development/muted-tone-mappings.json"
        ]
        || builtins.elem relative [
          "mod/config/characters.json"
          "mod/manifest.toml"
          "mod/toggle/manifest.toml"
          "mod/toggle/gml/palette_toggle.gml"
          "mod/toggle/gml/palette_controls.gml"
        ]
      );
  };
in
rustPlatform.buildRustPackage {
  pname = "mistria-palette";
  version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;
  src = source;
  cargoLock.lockFile = ../Cargo.lock;
  strictDeps = true;

  MISTRIA_MOMI_RUNNER = momi.runner;
  MISTRIA_MOMI_BINARY = momi.binary;

  postInstall = ''
    install -d "$out/share/mistria-palette"
    cp -r palettes "$out/share/mistria-palette/palettes"
  '';

  meta = {
    description = "Offline Fields of Mistria palette toolkit";
    mainProgram = "mistria-palette";
    platforms = [ "x86_64-linux" ];
  };
}
