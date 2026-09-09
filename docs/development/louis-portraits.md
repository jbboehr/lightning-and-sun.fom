# Louis portrait authoring

The profile covers all 32 main Louis portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Every strip has
two 296×180 frames; all 64 frames are unique. There are no main beach, bathing
or wedding portraits in this corpus. Overworld and UI assets are outside it.

The seasonal paths are under `assets/animations/NPCs/Louis/Portraits/`, but all
32 metadata files use **PortraitsMisc**, including their seasonal variants.
The stored profile has 18,487 component seeds across the 32 regions, each bound
to its original PNG's hash and size.
Source images, generated images and temporary Rust authoring tools stay ignored.

## Skin shades and protected details

The four catalog samples omit twelve exact shades. Several differ from a main
shade by only one RGB value, but remain visible as warm specks if left unchanged.
Each source color has its own connected-component group so a thin exposed neck
region can be separated from the shirt and cravat sharing its colors.

| Source colors | Role | Debug Blue |
| --- | --- | --- |
| `#F6D2AF`, `#F4D0AE`, `#F1CEAD` | Lit skin and fine raised-hand/neck blends | `#9DB9D4` |
| `#CB9879`, `#C99679`, `#CA9779`, `#CA9879` | Middle skin and fine hand/forehead blends | `#7F9FBD` |
| `#E3B590`, `#E3BD97` | Forehead bridge and jaw-edge shading | `#7F9FBD` |
| `#AD6F5C`, `#AB6F5C`, `#AC6F5C` | Skin shadows | `#6687AD` |
| `#784B42` | Deep skin shading | `#445F83` |
| `#634040`, `#40252A` | Local nose/cheek and outer-mouth contours | `#445F83` |
| `#491F1B` | Brown fringes around black eye/brow cores | `#445F83` |

The raised hand, face, ears, exposed neck and lower hand follow the target.
Actual brown head hair retains its `#634040` and `#40252A` components; the same
colors below the glasses form anatomical nose/cheek contours and recolor.
The black eyebrow cores remain black while the narrow brown endpoints and eye
fringes follow the skin palette. Glass frames, chains, lens colors and eye
details remain original.

Closed-mouth shading and the speaking lower outer arc follow skin. The upper
speaking interior border and pink tongue remain original. For neutral's speaking
frame, `#634040` at `[151,79]` and `[152..154,80]` is outer shading, while
`#634040` at `[150,78]` and `#491F1B` at `[151,77]` are interior boundaries.
Blue and Ryis mouths were inspected across every expression; no cosmetic
palette adjustment was needed.

The shirt and cravat share many skin colors. In Spring, the exposed neck includes
`#AD6F5C` at `[148,89]`, `[149..150,90]`, `[150..151,91]` and the dark wedge to
its right. The black boundary closes on row 92; the adjacent `#CB9879` shirt
pixels at `[147,90]`, `[148,91]` and `[154,92]` stay original. Each season's
neck is selected independently. Summer's cream sleeves and cuffs stay original,
including the ten `#40252A` pixels on the cuff side of the lower hand's black
separator. The hand's skin and finger creases on the other side recolor. Eleven
isolated `#CB9879` pixels along the raised cream cuff also stay original, from
`[178,62]` through `[185,75]`, below the raised hand's black boundary.

## Presets and checks

`palettes/sets/louis-portraits-trial.json` provides Debug Blue, Hayden, Ryis and
Seridia, with Vanilla implicit in the installed controls. The natural presets
use the catalog's four shade roles. Louis is assigned to V by integration.
Ryis's deepest shade is itself `#491F1B`: selected eye/brow-fringe pixels with
that source color correctly remain identical. Selection counts therefore differ
from changed-pixel counts for that target.

```sh
nix-shell --pure --run 'cargo test --locked --test louis_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/louis-portraits-study \
  --presets palettes/sets/louis-portraits-trial.json \
  --output generated/louis-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/louis.json \
  --output generated/louis-review-gallery
```

Use fresh output directories. `FOM_LOUIS_RECIPE` lets the opt-in source-corpus
test run against temporary omission or spill controls. The catalog-only control
failed at the omitted `#AB6F5C` hand/forehead blend `[147,51]` in Spring neutral.
Unrestricted mapping failed on true `#634040` head hair at `[157,38]`. The first
selected candidate exposed a further Autumn neck omission at `[145,87]`; a
literal-landmark assertion reproduced it before correction. Tests inspect source
coordinates independently of profile seeds. A temporary neck-only correction
then failed on the cream-cuff spill at Summer `[178,62]`. The final test passes
72 skin and 49 protected literal source landmarks across both frames.

The test also checks every output pixel for exact source-or-target color,
unchanged alpha and untouched unrelated colors. All 32 PNG dimensions and
metadata are checked, including the unusual PortraitsMisc atlas and frame count.

All four targets passed validation of 32 PNGs and their metadata. Each target
selects 70,164 pixels and protects 53,176 other occurrences of the source colors.
Debug Blue, Hayden and Seridia change every selected pixel. Ryis changes 69,148:
its other 1,016 selected pixels already match the darkest target shade. The
profile and selected regions are the same for every target.

The author inspected all 64 actual full-frame trios, all 64 enlarged
Source/Blue/Ryis face and mouth trios, and five distinct seasonal lower-hand
trios. The complete sweep found the Autumn/Summer neck omissions and Summer
cuff spill before the final correction. Exact comparisons verify that the
correction adds 544 neck pixels and protects 176 cuff pixels per target across
16 strips, with every other pixel unchanged. The affected final images were
also inspected.

Independent review inspected the complete frame, face and seasonal hand corpus,
then passed all 128 final target strips with 56 separately chosen source
landmarks per target. It confirmed the same selection for all four palettes,
including Ryis's identity mapping, and the exact neck/cuff correction with no
unrelated pixel or metadata changes. Its selected-mask SHA-256 is
`9dd32d2fb0507e9d10c88ae26e3e78f9cbb29e31c94fadf1fdbd85d38b52d9a3`.
Evidence is `tmp/hemlock-louis-art-review-louis-verification.json` and
`tmp/hemlock-louis-art-review-louis-delta.json`.

The final output is `generated/louis-portraits-reviewed/variants/`. The authored
gallery `generated/louis-review-final/` reuses all 64 frames without pending or
conflicting selections. Reconstructing a profile from those gallery selections
reproduces all 64 applied Blue frames exactly.

Local evidence includes `tmp/louis-catalog-red.log`,
`tmp/louis-unrestricted-red.log`, `tmp/louis-neck-omission-red.log`,
`tmp/louis-cuff-spill-red.log`, `tmp/louis-final-test.log`,
`tmp/louis-*-validation.json`, `tmp/louis-final-delta.json`,
`tmp/louis-final-roundtrip-verification.json`, `tmp/louis-final-faces-*.png`,
`tmp/louis-final-hands-*.png` and `tmp/louis-final-inputs.sha256`.

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The final profile SHA-256 is
`ff958051ffed36a444c05568d45d786696d6b718630bb71cc5cb7d0f67676d92`.
Integration owns the combined preview, installation, full repository checks
and live-game testing. This authoring pass neither modifies the game source nor
stages generated images.
