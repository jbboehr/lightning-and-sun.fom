# Holt portrait authoring

This pass covers all 32 main Holt portrait strips in the supplied archive:
eight expressions each for Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Every strip is
592×180 with two 296×180 frames, giving 64 unique frames. This character has no
Beach, bath or Wedding strips under the main portrait prefix. World sprites and
UI art are outside this profile.

The profile records original PNG hashes, dimensions and 20,204 explicit component
seeds. Nine separate connected-color groups let the masks distinguish skin from
hair, facial hair and clothing that share those colors. Temporary component
selection helpers remain ignored; the shipped recipe contains no classifier.

## Palette and boundaries

The NPC catalog supplies four colors. Five further shades occur in the fine
face, hand and neck shading:

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#F5C88E` | Main light skin | `#9DB9D4` |
| `#CD9067` | Medium skin and lip-edge shading | `#7F9FBD` |
| `#B45732` | Skin shadow and lower-lip shadow | `#6687AD` |
| `#762E21` | Deep skin contours | `#445F83` |
| `#E5AF7E` | Fine forehead, hand and neck shading | `#7F9FBD` |
| `#C38358` | Eye creases and fine finger shading | `#6687AD` |
| `#A3593C` | Fine hand and neck shadow | `#6687AD` |
| `#9A4929` | Deep neck and hand shading | `#445F83` |
| `#A05F4C` | Fine eye and Summer neck shading | `#6687AD` |

The set uses the same four shade roles for Debug Blue, Hayden, Ryis and Seridia;
Vanilla remains implicit. Each target changes 194,372 pixels.

Holt's orange scalp hair, ginger eyebrows and moustache retain their colors.
The actual eyebrow arches are hair; the lighter creases and small skin-shading
fringes beneath them recolor. The inner outline of the hanging scalp lock stays
ginger, while adjacent skin highlights and the inner-ear wrinkles remain selected.
Black outlines, gray iris details, white eye highlights, blush and tears stay
original. `#4B211A` was investigated and excluded from the source palette: its
inspected uses are dark eye detail and clothing/occlusion outlines.

Closed and speaking mouths were checked early in Blue and Ryis after the
[Nora lip correction](nora-portraits.md#lip-color-correction). Holt has no separate
pale-pink exterior lip highlight in these portraits. The orange band is his
moustache, which stays ginger in every preset; adjacent lip skin and the
lower-lip shadow follow the selected palette. Actual pink/red tongue and mouth
interior colors stay original, so no separate lip ramp is needed here. The Ugh
expression shifts the moustache down to y68; the exclusions follow that shifted
moustache shape.

Exposed neck gaps, fingers, hands and seasonal forearms recolor. The isolated
Spring neck patch at x156–159, y87–93 is included. Winter knit ribs share
`#B45732` and a cuff-side `#762E21` pixel with the skin palette; those fabric
components stay original. Adjacent `#CD9067` wrist contours remain selected on
the skin side of the black hand/cuff outline. Shirts, cuffs, scarves, suspenders,
belts and bags retain their colors.

## Local outputs

Export the 32 profile assets to `extracted/holt-portraits-study`, then use fresh
output directories:

```sh
target/release/mistria-palette build-presets \
  --original extracted/holt-portraits-study \
  --presets palettes/sets/holt-portraits-trial.json \
  --output generated/holt-portraits-reviewed
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/holt.json \
  --output generated/holt-review-reviewed
nix-shell --pure --run 'cargo test --locked --test holt_portraits -- --ignored'
```

The compact six-pose comparisons are `generated/holt-summary-reviewed.html` and
`generated/holt-summary-reviewed-ryis.html`, with matching PNGs. The full-frame
sheets and enlarged actual source/Blue/Ryis faces are ignored `tmp/holt-*` files.
The user reviews completed recolors rather than selecting individual regions.

## Verification evidence

The focused corpus test checks all 32 strips, metadata, dimensions, alpha,
unmapped colors and exact target colors, plus 37 original source-art landmarks
and 128 lower-mouth edge landmarks added by the correction below.
Those landmarks independently cover all nine shades, eye creases, lips, scalp
and eyebrow hair, moustache, iris details, tongue/interior, collar gaps, cuffs
and adjacent exposed hand shading. `FOM_HOLT_RECIPE` accepts a temporary candidate
recipe for mutation checks.

Five observed failures establish useful negative controls:

- The catalog-only mapping misses `#E5AF7E` at `[147,39]`.
- An unrestricted nine-color mapping recolors scalp hair at `[131,30]`.
- Removing the isolated Spring collar component misses skin at `[158,88]`.
- The first moustache boundary recolors the shifted Ugh moustache at `[144,66]`.
- The first cuff boundary recolors a Winter knit rib at `[141,96]`.

The final focused test and formatting check pass. All four targets pass exact
recipe validation across all 128 generated strips. The authored gallery reuses
all 64 frames, and reconstructing its selected seeds reproduces every applied
pixel. All 64 unique full frames and enlarged actual source/Blue/Ryis faces were
inspected, with additional mouth and cuff crops for the corrections above.

Local evidence includes `tmp/holt-catalog-red.log`, `tmp/holt-unmasked-red.log`,
`tmp/holt-gap-red.log`, `tmp/holt-moustache-red.log`, `tmp/holt-cuff-red.log`,
`tmp/holt-green.log`, `tmp/holt-frame-verification.json` and
`tmp/holt-{preset}-validation.json`.

Before the lower-mouth correction below, independent exact verification passed
all 128 target strips and 56 literal
landmarks per preset, checking source hashes, the exact corpus, every pixel,
alpha and transparent RGB, metadata and identical selected masks. Each target
preserves 62,020 additional pixels that share mapped source colors. The common
selected-mask digest is
`2d0588726abc21963da51783b2ee15bf06df5e835b801cf4dede88ca8d481f30`.
Evidence is `tmp/holt-josephine-art-review-holt-verification.json`.

## Lower-mouth edge correction

The user identified four warm pixels beneath the neutral moustache. The broad
moustache exclusion also protected skin shading just below the black mouth
outline. Those isolated components now recolor: closed neutral `[148,67]`,
`[149,67]`, `[156,67]`, and `[157,67]`, plus the corresponding speaking-mouth
corners. The matching corner below sad and speaking Ugh mouths is included too;
the closed Ugh corner was already mapped and provides a consistency check.
Actual moustache fill, ginger hair strokes, black/dark mouth contours, and the
tongue/interior remain unchanged.

Ten distinct source mouth crops were inspected across all 64 frames. The
expanded test first failed at Autumn embarrassed `[148,67]`, then passed after
124 isolated one-pixel components were added across 24 strips. Every previous
seed, color group, palette value, and source hash is retained. Each of the four
targets differs from the previous bundle at exactly those 124 pixels, with zero
differences elsewhere in Holt's 32 strips. The other fourteen characters'
complete output trees are byte-identical, including Josephine's.

The updated two-character PNG is `generated/holt-mouth-fix-preview/summary.png`;
`mouth-comparison.png` beside it shows the previous and corrected mouth edges.
Local evidence includes `tmp/holt-mouth-fix-red.log`,
`tmp/holt-mouth-fix-checks.log`, `tmp/holt-mouth-fix-diff.json`, and
`tmp/holt-mouth-fix-other-characters.log`. Formatting, Clippy, all 87 active tests,
both local character tests, and the release build passed after the correction.

All 5,712 combined variants passed exact recipe validation. A fresh MOMI
installation in `tmp/holt-mouth-fix-playtest` verified the atlas pixels, metadata,
and scripts. The real game checked all 32 Holt strips through all five choices,
the H key, season wraps, independent selections and portrait phase. One Josephine
portrait added an independence check: 33 distinct sources and 165 source/preset
pairs in total, with no script errors and a graceful exit. The usual launcher
now points to that trial. The original game archive remained unchanged.
This correction did not rerun uninstall or the earlier full character/world
matrices; desktop visual acceptance remains with the user.

Installation and live evidence are `tmp/holt-mouth-fix-install-report.json`,
`tmp/holt-mouth-fix-live.log`, and `tmp/holt-mouth-fix-live-audit.json`.

This authoring pass does not install or launch the game. Combined package,
installation, full repository verification and live checks belong to the
integration pass. Only declarative palette/profile data, this development note
and the focused test are intended for Git; game images and generated files stay
ignored.
