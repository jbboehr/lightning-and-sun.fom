# Palette mod investigation

Investigated 2026-09-06. The supplied build supports a small asset replacement
through MOMI's current PNG replacement path. Both frames of the test portrait were
installed into an isolated archive copy and recovered pixel-for-pixel from its
atlas. The subsequent [portrait toggle](development/portrait-toggle.md) adds a
separate variant and switches it with F6 in-game; vanilla remains the session default.
The [CLI installer](development/cli-installer.md) now generates and installs that
study from local assets, with checked snapshot restoration for removal.
The later [four-season portrait trial](development/all-season-portraits.md) covers
Adeline's 100 seasonal strips with five palette choices.

Follow-up [runtime integration research](development/runtime-capabilities.md)
examines calling a Rust helper from GML, loading generated images during play,
and the limitations of the game's existing palette shader.

## The supplied build changes the plan

`tmp/fields-of-mistria` contains a native Linux executable, `Maybe.toml`, and a
625,641,237-byte `assets.zip`. It has no `data.win`. The archive contains 122,403
entries, including 38,941 PNGs and 77,801 TOML files. Its SHA-256 is:

```text
b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5
```

The archive fingerprint identifies the tested build. The later isolated game run
displayed `v1.0.4 (modified)` on the title screen. This version was observed in the
running game, not inferred from file timestamps.

The handoff's GameMaker and pre-1.0 assumptions are historical. NPC Studio describes
the migration to its internal engine in its [engine beta announcement](https://www.fieldsofmistria.com/post/new-engine-beta-branch).
Its [official SDK](https://github.com/NPC-Studio/mistria-sdk) now documents assets and
ships Linux/Windows helper binaries. The SDK README targets v0.16.4 and disclaims
support for mod creation, so documentation can still lag the installed build.

## Tool choice

Use MOMI for packaging and atlas updates. The tested release is
[MOMI v0.15.10](https://github.com/Garethp/Mods-of-Mistria-Installer/releases/tag/v0.15.10),
published 2026-08-30. Source inspected at
`d437e08e9ae8a594ccbc8dad020e357274a20b3b`. Its
[replacement installer](https://github.com/Garethp/Mods-of-Mistria-Installer/blob/d437e08e9ae8a594ccbc8dad020e357274a20b3b/ModsOfMistriaInstallerLib/Installer/ImageInstaller.cs)
accepts `images/replace/spr_….png`, finds existing animation metadata by basename,
and updates the appropriate atlas. Keep the original filename and dimensions.
The [manifest format](https://github.com/Garethp/Mods-of-Mistria-Installer/blob/d437e08e9ae8a594ccbc8dad020e357274a20b3b/README.md)
uses `manifest.toml` with name, author, version, minimum installer version, and
manifest version. The scaffold uses that format instead of the handoff's JSON.

MOMI is an installer that rebuilds `assets.zip`, not a runtime palette loader. Its
[archive store](https://github.com/Garethp/Mods-of-Mistria-Installer/blob/d437e08e9ae8a594ccbc8dad020e357274a20b3b/ModsOfMistriaInstallerLib/Store/AssetsStore.cs)
keeps `assets.bak.zip` and handles install/uninstall. Our Rust tools never write
the source archive. They create a package for MOMI to apply to a separate copy.

[UndertaleModTool/UndertaleModCli](https://github.com/UnderminersTeam/UndertaleModTool)
remains relevant to older GameMaker data files. The CLI supports Linux. It was not
run against this ZIP-based build because there is no compatible GameMaker data
file to open. Do not describe that as an UndertaleModTool parsing failure.

The SDK's [`maybe` tool](https://github.com/NPC-Studio/mistria-sdk/blob/c7a92ffdbc6f8088c6c273ba6af2250931c3d208/maybe.md)
is an alternative for atlas operations. It was not needed for this proof.

## Asset layout and preservation

The selected portrait is:

```text
assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral.png
```

Local metadata gives ID `5e2770bc75d292ac`, frame size 296×180, two frames, duration
0.2 seconds, and atlas `PortraitsSpring`. The PNG is a 592×180 horizontal strip.
The runtime atlas is `assets/atlases/PortraitsSpringAtlas.png`, with placements in
the adjacent `.meta.toml`. The corresponding shape metadata is under
`assets/shapes/NPCs/Adeline/Portraits/Spring/` and is left untouched.

An overworld example, inspected without exporting or changing it, is
`assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_spring_idle_south.png`.
It is 80×80, uses the `Default` atlas, and has horizontal offset `Middle` and vertical
offset 54.0. The missing frame count means one frame.

The SDK documents [frame dimensions, timing, offsets, and filtering](https://github.com/NPC-Studio/mistria-sdk/blob/c7a92ffdbc6f8088c6c273ba6af2250931c3d208/asset-properties/animations.md).
Preserve all of them. Atlas entries can share identical frames and record trimmed
rectangles with original dimensions and offsets. The
[atlas specification](https://github.com/NPC-Studio/mistria-sdk/blob/c7a92ffdbc6f8088c6c273ba6af2250931c3d208/asset-properties/texture-atlases.md)
explains these placements. Replacing only a loose PNG does not prove the game will
render it. The test checks the installed atlas frames too.

## Palette model and art limits

`mod/config/character-palettes.json` sketches per-character asset lists, the selected
preset, and paths to palette JSON files relative to the repository root.
`presets.json` separates available presets from planned ones. Both default and
Adeline selection are `vanilla`. These files describe a future selector. The
current CLI requires an explicit palette file and does not consume selector config.

Each palette has an `rgba_map`. Six-digit colors mean fully opaque RGBA, and
eight-digit colors match alpha too. Mapping is simultaneous, so A→B and B→C do not
turn original A pixels into C. Alpha-changing maps are rejected. Fully transparent
pixels remain unchanged. An empty map copies original PNG bytes exactly.

The initial blue study changed 4,000 pixels across the two-frame portrait, including
unwanted robe and jewelry details. The subsequent [region study](development/portrait-regions.md)
uses reviewed seed points to select connected skin-color regions and adds one
previously missed shade. It changes 3,259 pixels while protecting 822 matching-color
pixels outside the mask. Dark linework, eyes, clothing, and jewelry retain their
original colors in the reviewed portrait. Exact matching alone does not identify
body regions; each additional asset still needs region review. Contact sheets use
nearest-neighbor 4× or 8× zoom and a checkerboard, with an optional changed-pixel
column. Recipe-aware validation checks every pixel against the mask and palette,
but cannot certify artistic correctness.

For AI-assisted exploration: export one representative portrait, manually mask the
intended region, ask an editor/model for a palette suggestion, sample a small ramp,
record exact mappings, apply them deterministically, and inspect every related
frame. AI output is reference material. No AI-generated game assets were used here.

## Risks and remaining work

- Game updates can change IDs, file names, shared colors, metadata, and atlases.
  Re-export and validate against each new archive fingerprint. The historical
  pre-1.0 compatibility risk also applies after 1.0.
- Steam updates may replace the modified archive. MOMI's documented workflow is
  to reinstall compatible mods afterward. Actual Steam update behavior was not
  exercised in this session. Never restore an older backup over a newer build.
- MOMI rewrites a whole archive and has non-atomic copy/flush windows. Keep a
  separate pristine copy beyond its own backup. Cosmetic mods targeting the same
  sprite need conflict review.
- MOMI normalizes multiple atlas metadata files even for one replacement. The
  installation experiment checks semantic metadata and recovered frames, not ZIP
  byte equality after install.
- Distribute original source code and palette definitions. Exported images, modified
  portraits, atlases, game binaries, and local MOMI packages are excluded from Git.
  This project does not grant rights to redistribute NPC Studio's artwork.
- The expanded Adeline trial has local art, installed-atlas, and in-game checks
  for the four seasonal outfits. Naturally encountered story dialogue, special
  outfits, multi-character selection, overworld recoloring, and final art remain
  outside this proof of concept.
