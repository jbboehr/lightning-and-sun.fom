# Darcy portrait authoring

The profile covers all 32 main Darcy portrait strips in the supplied archive:
eight expressions each for Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Every strip has
two 296×180 frames, giving 64 unique frames. All 32 metadata files use
`PortraitsMisc`, including the seasonal outfits. There are no Beach, bath or
Wedding strips under Darcy's main portrait prefix in this archive.

Each region binds its strip to the original SHA-256 and 592×180 dimensions. The
profile stores 12,264 explicit component seeds in ten separate color groups.
Temporary Rust helpers select those components; the shipped files contain
declarative palette data. World sprites and UI assets are outside this profile.

## Palette and boundaries

The catalog's four colors omit most exposed Summer arm pixels and several fine
face, ear, neck and hand shades. The authored palette uses these roles:

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#FCD8A9` | Main skin light | `#9DB9D4` |
| `#EAB979` | Medium shading | `#7F9FBD` |
| `#D99D60` | Skin shadow | `#6687AD` |
| `#B7672F` | Deep shading and lip edges | `#445F83` |
| `#FFD8AC` | Alternate Summer arm light | `#9DB9D4` |
| `#DDA266` | Fine eye and face shading | `#6687AD` |
| `#904522` | Nose, ear, neck and lower-lip shading | `#445F83` |
| `#6C3407` | Deep jaw, neck and hand contours | `#445F83` |
| `#551D06` | Selected deep neck contours | `#445F83` |
| `#7B2806` | Two deep Summer arm contours | `#445F83` |

Warm skin fringes around the eyes, skin below the lips, and exposed collar gaps
follow the palette. Actual dark `#551D06` eye and speaking-mouth contours stay
original. The separate taupe `#846159` brow and lower-eye details also remain,
along with gray hair, iris colors, black outlines and tongue/interior colors.

The gold ornament shares `#B7672F` with Spring skin and `#FCD8A9`/`#EAB979` with
Summer skin. Its components are excluded. Clothing, cuffs, buttons, ribbons and
the Winter scarf remain original. Darcy's cosmetic `#FCB4B0` and `#DD716B` lip
colors are intentionally preserved. Their contrast against darker presets can
be reviewed later; skin-edge completeness is checked separately.

After the combined trial, the user noted that the mouth feels off. Close-ups
of the original and all four targets show the pale pink lip highlight becoming
a bright streak against Blue and Ryis. The recommended cosmetic follow-up is
to darken that highlight for deeper palettes while retaining its pink hue and
the original mouth interiors. This assessment does not change Darcy's recipe;
the comparisons remain local in `tmp/darcy-mouth-assessment-*.png`.

Debug Blue, Hayden, Ryis and Seridia use the same masks and shade roles. Vanilla
is supplied by the runtime. Each target changes 64,432 pixels and preserves 392
pixels that share mapped source colors. There are no selected identity-color
matches for these four Darcy targets.

## Reproduce the local outputs

After exporting the 32 profile assets into `extracted/darcy-portraits-study`,
use fresh output destinations:

```sh
target/release/mistria-palette build-presets \
  --original extracted/darcy-portraits-study \
  --presets palettes/sets/darcy-portraits-trial.json \
  --output generated/darcy-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/darcy.json \
  --output generated/darcy-review-authored
nix-shell --pure --run 'cargo test --locked --test darcy_portraits -- --ignored'
```

The six-example preview is `generated/darcy-summary.html` and its sibling PNG.
It compares actual Original, Debug Blue and Ryis outputs. Full-frame sheets and
enlarged faces remain in ignored `tmp/darcy-*` files.

## Verification evidence

The focused corpus test checks all 32 strips, dimensions, byte-identical metadata,
alpha, unmapped pixels and exact target colors, plus 41 literal source-art
landmarks. Those landmarks cover light arms, deep arm contours, eye fringes,
ears, neck/collar openings, lip skin, gold ornaments, actual eye details,
cosmetics and speaking-mouth interiors. `FOM_DARCY_RECIPE` accepts a temporary
candidate recipe for negative controls.

Two observed failures preceded the final masks:

- The catalog-only recipe leaves `#DDA266` skin at `[153,77]` unchanged.
- An unrestricted ten-color recipe recolors the Spring gold ornament's
  `#B7672F` pixel at `[130,62]`.

The final focused test passes. All four presets validate across 128 output
strips. The authored gallery reuses all 64 unique frames; reconstructing its
selected seeds reproduces every applied pixel. All 64 actual full-frame
Source/Blue/Ryis comparisons and all 64 enlarged face/neck comparisons were
inspected, including closed and speaking mouths.

The independent review also passes all 128 target strips and 44 literal
landmarks per target, checking source hashes, exact corpus, every pixel,
transparent RGB and metadata. All targets have the selected-mask digest
`6dde881847e6ce171073a0642f66da46e2c9c44e59f6ef3386a3b6d884440f53`.
Evidence is in `tmp/darcy-dell-art-review-darcy-verification.json`,
`tmp/darcy-{preset}-validation.json`, `tmp/darcy-roundtrip-verification.json`,
`tmp/darcy-catalog-red.log`, `tmp/darcy-spill-red.log` and `tmp/darcy-green.log`.

This authoring pass does not install or launch the game. Combined package and
live-game checks belong to the integration pass. Only the profile, palettes,
focused test and development note are intended for Git; source game files and
generated images remain ignored.
