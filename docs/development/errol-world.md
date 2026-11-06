# Errol: complete world candidate

2026-11-06. **Authored and validated; displayed world artwork accepted.**
This approved [finish-plan](../plans/finish.md) candidate adds all 107 omitted
world strips. The existing 36 portrait regions, nine colors, five color groups,
preset IDs, labels and mappings remain exactly unchanged. No additional color
role or source shade is needed. Shared registry, collection selection, planning
and integrated checks belong to the parent integrator.

## Sources and material decisions

The preserved `local/game/assets.zip` has SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
A fresh export contains only the required 143 Errol PNG/metadata pairs, retaining
all native properties and the NPC outfit/cycle definitions. Each profile region
pins the original PNG hash and dimensions. The archive and accepted portrait
fixtures were not replaced or copied into another archive.

Autumn, Summer and Winter each contain 26 strips and 94 native frames; Spring
contains 29 strips and 97 frames. In addition to idle, walk, blink, seated,
drink, eat and action directions, all four outfits include brush, magnify
start/loop/end, pickaxe and trowel. Spring adds all three shocked poses.
The frozen inventory matches `assets/fiddle/npcs/errol.toml` and source metadata:
379 native frame occurrences, 227 exact decoded unique frames and 152 repeated
occurrences. Native East strips add 188 West mirrors, producing 567 review cases.
All frames are 80×80; the longest strip is the ten-frame trowel animation.

Original Spring, Summer and Winter portraits and every world source frame were
inspected to distinguish exposed scalp/nape, ears, eyebrow-adjacent skin, outer
lip, wrists and fingers from the beard, clothing and tool colors. All three
world skin shades already exist in the accepted portrait palette:

| Source | Existing target role | Source material |
| --- | --- | --- |
| `#F4C49C` | Light, index 0 | Lit scalp, face, ears, arms and hands |
| `#C9927B` | Shadow, index 6 | Face/nape, outer-lip and finger shading |
| `#85584B` | Deep, index 5 | Scalp/ear/eye-edge and hand contours |

Every occurrence of these three shades in this world corpus is exposed skin.
The original mixed first color group is preserved: adjacent `#F4C49C` and
`#C9927B` skin can share a component. `#85584B` retains its separate group.
World shades introduce no new mapping and do not modify prior portrait treatment.

The gray/white beard, side hair and eyebrows (`#897C6C`, `#BCB2A5`,
`#EFEAE3` and other native grays), eye highlights and black pupils remain original.
The small `#C9927B` patch between moustache and beard is exposed skin and recolors.
Real dark/red eating and shocked-mouth interiors (`#410808`, `#9E2626`) remain
original, as does all black linework. Hat browns, shirts, suspenders, waistcoats,
Winter coat/scarf, trousers, boots, watch/button details and archaeological tools
retain source pixels. In particular, pickaxe head/shaft/swing sparks, brush
handle/bristles, magnifying lens and trowel metal remain unchanged. Boots are
covered feet, not omitted exposed skin.

The 5,185 new component seeds select 18,794 world pixels per target: Autumn
3,591; Spring 6,672; Summer 5,672; Winter 2,859. Every native frame contains skin;
there are no empty masks or silently omitted strips. Back-facing heads and hands
remain included even in seated/eating North. The entire 143-strip build changes
157,286 pixels per target, including 138,492 retained portrait pixels.

## Review and focused checks

- [Five-choice SAMPLE](../../generated/errol-world-finish-review/summary.png),
  correctly titled ERROL, with Vanilla, Debug Blue, Hayden, Ryis and Seridia in
  all four outfits.
- [Complete offline review](../../generated/errol-world-finish-review/index.html):
  567 cases over 207 small case pages plus four outfit indexes and the root.
  Vanilla is left and Debug Blue right. Both sides show the full 80×80 frame
  at 3× and a scalp/face/hand detail at 6×, including all West mirrors.
- [Component gallery](../../generated/errol-world-mask-review/index.html):
  107 strips / 379 frames / 227 unique frames, reconstructed with no conflicting
  or unresolved groups. These are candidate masks, not newly accepted artwork.
- Bundle: `generated/errol-world-finish-build/`.
- Source corpus: `extracted/errol-world-finish-study/`, with stable test link
  `extracted/test-corpus/errol` pointing there. Historical portrait corpus remains.
- Reproducible helpers, source census, hashes, timing, validations and negative
  controls: `local/authoring/errol-world/`. The 29 complete full/detail inspection
  contact sheets remain under `tmp/errol-world-detail-sheets/`.

The author opened all 20 original source sheets, original portrait studies, all
29 contact sheets containing every one of the 567 native/mirrored full Source/Blue
pairs and enlarged details, and the five-choice sample. This is agent inspection,
not user acceptance. The canonical gallery checker reconciles frame counts with
native metadata, binds 1,134 full-frame cells and 1,134 details plus 20 sample
cells to the current source/bundle, and checks 122,041,152 rendered pixels exactly.
The full views include every native frame pixel, including outlying pickaxe
sparks at local x=77 and their West mirrors.

Focused checks passed:

```sh
nix-shell --pure --run 'cargo test --release --locked --test errol_world --test errol_portraits -- --include-ignored'
nix-shell --pure --run 'cargo clippy --release --locked --test errol_world -- -D warnings'
nix-shell --pure --run 'rustfmt --edition 2024 --check tests/errol_world.rs'
node local/authoring/errol-world/validate.mjs
node local/authoring/errol-world/negative-controls.mjs
node local/authoring/errol-world/browser.mjs
```

Two regular tests preserve the old profile/set and package the literal 107-strip
inventory with all 379 native frames, scalar/array timing, offsets and atlas.
The ignored material test checks every world RGBA pixel for all four targets
against independent literal source-material expectations and a fixed 107-strip
per-frame census, with 288 literal source/target landmark pairs covering 30 skin
and 42 protected source points across the four choices. It also checks exact
unique-frame count and unchanged metadata. Every one of the 288 prior portrait
PNG/metadata outputs matches a fresh build of the original recipe byte-for-byte.
The unchanged retained portrait material test also passes. Expectations do not
read authored seeds or build an expected mask using the authoring helper.

The first source test exposed an incorrectly transcribed lens landmark:
`(44,39)` in magnify-start frame 2 is `#E7F7FF`, not `#639CDA`. Source inspection
corrected only that literal assertion; no candidate artwork changed.
Two separate negative candidates then demonstrate useful visible failures:
removing the Autumn idle-South palm component fails at `(32,44)`; selecting its
white beard component fails at `(39,35)`. Both fail the full-pixel material test
with exit 101 and remain excluded from the final profile and bundle.

All four complete 143-strip choices pass exact CLI recipe validation without
source-hash overrides. Combined validation took 0.588 seconds. Full collection,
standard combined checks and isolated install/remove are integrator work and
are not claimed here. Final CPU Chromium loaded every one of the 212 desktop
HTML pages at 1200 pixels plus all five mobile indexes at 390 pixels: 217 page
loads in 139.194 seconds, with GPU disabled. Every image/link exists and
loads, and no horizontal overflow remains.

## Measurements, retention and limits

Hashing/export took 1.162 seconds and finished at `2026-11-06T08:02:23.060Z`.
The first passing source tests and focused Clippy completed at
`2026-11-06T08:11:57.780Z`. The complete native/mirror full/detail inspection
checkpoint was `2026-11-06T08:14:36.849Z`, about 12m14s after export. Reading,
source analysis, writing independent tests and gallery verification are included
in that elapsed agent interval; helper preparation preceded export. The 0.100s
component generation runtime alone is not an authoring or human-review estimate.

The initial CPU browser check reproduced mobile overflow caused by long native
special-animation names in outfit links. Gallery CSS now permits wrapping those
names. The failed log and subsequent full positive run are retained. This is a
local generated-gallery correction; no application runtime changed.

All image processing used CPU-native Rust tools and Node orchestration. No
Python/Pillow editing, local inference, GPU or live-game test was used. Sources,
bundle, canonical review, component gallery and scoped authoring evidence total
about 51 MiB, plus about 6 MiB of complete inspection sheets and temporary pixel
grids. These game-derived outputs stay ignored/local. Retain them through batch
review and the stable corpus while its source-backed tests depend on it.

Final profile SHA-256:
`a79c7dd41e7356c862841b7a512048adae45666eae0100e455979be7ff7b3257`.
Final preset-set SHA-256:
`a39e3c770feec4c8d557f35308aeab98695c706422b725cf093079825158ad1e`.
The user accepted the displayed Errol/Hemlock world artwork after consolidated
review on 2026-11-06. Inspection depth is unspecified; other browser engines and
live gameplay remain unverified. The [batch record](fifth-finish-batch-2026-11-06.md)
owns combined checks and the actual assessment. The accepted regression snapshot
is `local/fixtures/finish-errol/`; earlier approval limits remain unchanged.
