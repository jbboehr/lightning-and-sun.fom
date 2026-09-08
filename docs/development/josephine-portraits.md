# Josephine portrait authoring

This profile covers all 32 main Josephine portrait strips in the supplied
archive, with eight expressions in each of four seasons: embarrassed, happy,
mad, neutral, sad, think, ugh and wink. Each strip contains two 296×180 frames.
All 64 frames are unique. This corpus has no beach, bathing or wedding strips.
Overworld and UI assets are outside the profile.

The compact preview is `generated/josephine-summary.html`, with six original
and Debug Blue pairs covering all four seasons and closed/speaking faces.
The optional component gallery is `generated/josephine-review-authored/index.html`.
All source images, generated images and temporary Rust helpers remain ignored.

## Source colors and boundaries

`palettes/profiles/josephine-portraits.json` records 3,327 component seeds,
original PNG hashes and dimensions. The catalog's four samples omit five
shades used on the nose, eye fringe, jaw, neck and hands:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#9B643A` | Main skin | `#9DB9D4` |
| `#814030` | Middle skin shading | `#7F9FBD` |
| `#752F23` | Skin shadow | `#6687AD` |
| `#5B241E` | Eye, neck, arm and lower-lip contours | `#445F83` |
| `#481518` | Deep jaw contour | `#445F83` |
| `#8D5036` | Fine eye and nose shading | `#7F9FBD` |
| `#8E522F` | Fine neck shading | `#7F9FBD` |
| `#46232B` | Deep finger creases | `#445F83` |
| `#3A1D24` | Deep nose contour | `#445F83` |

Four connected-color groups isolate the shared `#5B241E`, `#46232B` and
`#3A1D24` shades from the other six colors. Explicit source-bound seeds select
skin while retaining hair curls, eyebrow hair, clothes and accessories. In
particular, `#46232B` is both a hand-crease color and a common hair shade.
`#3A1D24` shades the nose, but also outlines eyebrows and the rose ornament.
The Winter gold accessory contains `#9B643A`, which stays original there.
`#A44F35` beside the Spring white neckline belongs to the blouse/hair edge
and is not added to the skin map.

The face, ears, neck, collar gaps, exposed arms and both hands recolor.
Skin-colored eye fringes and the lower-lip shadow follow the target palette.
Summer arm contours and the small skin slivers beneath the hair are included.
At the lower hand, the Winter sleeve boundary and Autumn cuff tip have a
continuous dark-to-light skin ramp and recolor. The dark outlines within the
Spring cuff band and the higher Autumn cuff crease remain original.

Josephine's saturated lipstick (`#943D52` and `#C15871`) already fits all four
target ramps in the inspected closed and speaking mouths. These colors also
appear in eye makeup, ornaments and clothes. They remain original, along with
tongues and mouth interiors. The deeper speaking-mouth border is protected
separately from the shifted lower-lip skin shadow. This treatment follows an
actual four-preset mouth comparison, including Ryis, rather than copying
Nora's correction for her much paler lip highlight.

Each target changes 123,754 pixels and preserves 38,368 other pixels that
share the nine source colors. Temporary selection rules are not part of the
runtime, which uses only the recorded source-bound components.

## Presets and atlas

`palettes/sets/josephine-portraits-trial.json` orders Debug Blue, Hayden, Ryis
and Seridia. The three NPC targets use their catalog's four shades through the
roles above. All four targets share one mask. Vanilla is implicit in the
installed controls, and integration assigns Josephine to P.

Josephine's source PNGs and metadata are directly under
`assets/animations/NPCs/Josephine/Portraits/`. The season appears in each filename,
with no seasonal subdirectory. All 32 metadata files use `PortraitsMisc`.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test josephine_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/josephine-portraits-study \
  --palette palettes/stylized/josephine-portraits.json \
  --output generated/josephine-portraits-study
target/release/mistria-palette validate \
  --original extracted/josephine-portraits-study \
  --modified generated/josephine-portraits-study \
  --palette palettes/stylized/josephine-portraits.json
target/release/mistria-palette build-presets \
  --original extracted/josephine-portraits-study \
  --presets palettes/sets/josephine-portraits-trial.json \
  --output generated/josephine-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/josephine.json \
  --output generated/josephine-review-authored
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all 32 strips for dimensions, alpha, unchanged metadata, untouched unrelated
colors and exact mapped colors. Its 69 literal source landmarks include 44
skin points and 25 protected points. They cover all nine skin shades, fine
nose/eye shading, neck gaps, arm contours, finger creases, lower-lip shadows,
hair, eyebrow hair, cosmetics, tongue/interior colors, cuffs and ornaments.
`FOM_JOSEPHINE_RECIPE` accepts a temporary candidate recipe for mutation checks.

The catalog-only recipe failed on the omitted jaw shade at Spring neutral
`[150,89]`. The unrestricted nine-color recipe failed on the Autumn hair curl
at `[175,110]`. The first masked candidate then exposed an omitted Summer arm
contour at `[97,133]`. Each failure was observed before its corresponding
mapping or selection correction. The final focused test passes.

All four targets passed exact recipe validation across all 32 strips.
The authored gallery reuses all 64 frames with zero pending or conflicting
selections. Reconstructing a profile from those selections reproduces every
applied pixel, and all 64 frames match the inspected author previews. All
unique full frames and 64 enlarged actual Source/Blue/Ryis face trios were
inspected, plus 32 distinct five-way mouth crops covering the source and all
four presets. The six-example summary was also inspected.

Local evidence is in `tmp/josephine-catalog-red.log`,
`tmp/josephine-spill-red.log`, `tmp/josephine-summer-red.log`,
`tmp/josephine-candidate-test.log`, `tmp/josephine-*-validation.json`,
`tmp/josephine-frame-verification.json`, `tmp/josephine-authored-report.json`,
`tmp/josephine-final-faces-*.png`, `tmp/josephine-mouth-matrix-*.png` and
`tmp/josephine-final-inputs.sha256`. The source archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This authoring pass changes declarative recipes and a focused corpus test.
Combined generation, installation, full repository verification and live-game
checks belong to the integration pass. No source game files or generated
images are staged by this workflow.

## Upper-mouth follow-up

The user later questioned the upper mouth. A closer Source/Blue/Ryis comparison
covered all 64 frames and seven distinct mouth crops. All 5,410 light and middle
skin pixels in the inspected mouth region map correctly in each target,
including its 224 fine `#814030` pixels. No omitted skin or spill was found.

The dark upper line uses the original `#6F2635` and `#46232B` inside the
lipstick/mouth shape. The isolated `#46232B` at `[160,80]` in the neutral closed
frame and `[159,79]` in its speaking frame joins that line and the black outline
in the source. These pixels remain original. The changed skin makes this
existing contrast more visible, but the source comparison did not support a
specific palette correction. Recipes and tests are unchanged.

The user agreed to defer further lipstick adjustments. Its dark upper edge can
look harsher against some replacement skin tones even when the mask is correct.
Matching lipstick shades to each palette remains a cosmetic follow-up; the
current source colors are accepted for this prototype checkpoint.

The bounded audit and comparison are
`tmp/josephine-upper-mouth-audit.json` and
`tmp/josephine-upper-mouth-comparison.png`. The original corpus test passed
again, and all four recipe/profile hashes still match the initial bundle.
