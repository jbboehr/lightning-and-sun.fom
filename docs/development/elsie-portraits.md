# Elsie portrait authoring

The profile covers all 36 main Elsie portrait strips in the supplied archive:
nine expressions in each of Spring, Summer, Autumn and Winter. The expressions
are closed_eyes, embarrassed, happy, mad, neutral, sad, think, ugh and wink.
Each strip has two 296×180 frames and duration 0.2, giving 72 unique frames.
All nine metadata files for each season use its corresponding seasonal atlas.
There are no Beach, bath or Wedding portraits under Elsie's main portrait prefix.
World sprites and UI assets remain outside this profile.

Every region binds its original PNG SHA-256 and 592×180 dimensions. The profile
stores 19,066 explicit component seeds in eight separate color groups. Temporary
Rust helpers author those selections; the installed recipes contain declarative
palette data.

## Palette and boundaries

The four catalog samples miss deeper face, ear, neck and hand shading, plus two
small eye-fringe shades found in other expressions:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#F6D2AF` | Main skin light | `#9DB9D4` |
| `#E3B598` | Medium skin shading | `#7F9FBD` |
| `#CB9472` | Skin shadows and fine creases | `#6687AD` |
| `#B46D5F` | Deep skin shading | `#445F83` |
| `#794035` | Deep wrinkles, ears, mole, neck and hands | `#445F83` |
| `#4C0F1E` | Selected under-jaw wrinkle | `#445F83` |
| `#CB9879` | Expression-specific eye/brow skin fringe | `#6687AD` |
| `#784B42` | Deep outer-eye skin fringe | `#445F83` |

The added eye shades occur only at x144–169, y55–64 in this corpus. Warm skin
fringes, lip-edge skin, exposed collar gaps and tiny arm creases follow the
palette. Actual burgundy brow/lash strokes, pink eye and lip cosmetics, cheek
color, nail fill and mouth interiors remain original. Optional cosmetic contrast
changes are deferred.

The deep `#4C0F1E` under-jaw wrinkle is selected separately from the same color
in eye strokes, rings, the Autumn bracelet and Winter brooch. `#794035` also
appears in the Summer gold bow and Winter hood/scarf; those accessory components
stay original. Winter's exposed chest and small arm gap above the glove recolor,
while its ivory gloves, cuffs and stole remain unchanged.

Two boundaries required close source comparison. The Summer bow's right dark
folds at `[151..154,92]`, `[155,93]` and `[153..154,94] are protected. The adjacent
continuous finger underside at `[158..169,95]` follows the skin while the actual
pink nail remains. Winter's connected brown eye fringe at `[144,61]` recolors;
separate gold eye cosmetics remain original, including dark components that
correspond to true cosmetic strokes in Spring.

Debug Blue, Hayden, Ryis and Seridia use the same selected regions and shade
roles. Vanilla is implicit in the runtime. Each target changes 135,136 pixels
and preserves 4,782 pixels that share mapped source colors. None of these four
targets has a selected identity-color match.

## Reproduce the local outputs

After exporting the profile's 36 assets into `extracted/elsie-portraits-study`,
use fresh output destinations:

```sh
target/release/mistria-palette build-presets \
  --original extracted/elsie-portraits-study \
  --presets palettes/sets/elsie-portraits-trial.json \
  --output generated/elsie-portraits-final
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/elsie.json \
  --output generated/elsie-review-accepted
nix-shell --pure --run 'cargo test --locked --test elsie_portraits -- --ignored'
```

The combined integration pass supplies the user preview and game trial. Full
Source/Blue/Ryis sheets and enlarged actual face/hand crops remain in ignored
`tmp/elsie-*` files.

## Verification evidence

The focused local test checks all 36 strips, dimensions, exact metadata, alpha,
unmapped colors and exact target colors, plus 55 literal source-art landmarks.
The coordinates are independent of profile seeds and cover all eight shades,
isolated eye fringes, ears, jaw wrinkles, chest/arm shading, finger contours,
cosmetics, mouth interiors, bow folds, jewelry and Winter hood boundaries.
`FOM_ELSIE_RECIPE` accepts a temporary candidate recipe for negative controls.

Observed failures preceded the corresponding corrections:

- The catalog-only recipe misses deep skin at `[154,59]`.
- An unrestricted expanded recipe recolors the true brow stroke at `[149,56]`.
- The first selected candidate misses the expression-specific skin fringe
  `#CB9879` at `[153,61]` in closed_eyes.

The final focused test passes. All four presets validate across 144 output
strips. The accepted gallery reuses all 72 frames; reconstructing its selected
seeds reproduces every applied Blue pixel. All 72 full-frame comparisons and
72 enlarged face/ear/mouth comparisons were inspected, along with seasonal
hand, neck and lower-arm crops. Concrete corrections were collected before
the final stable-data handoff.

The independent final review passes all 144 target strips and 54 literal
landmarks per target, checking source hashes, exact corpus, every pixel,
transparent RGB, metadata and equal selections across presets. The shared
selected-mask digest is
`7b3b5e30b15f66e54ceb26ddca61be47835f89177240c4d62070ca24102945a5`.
Evidence includes `tmp/elsie-errol-art-review-elsie-verification.json`,
`tmp/elsie-{preset}-validation.json`,
`tmp/elsie-roundtrip-verification-final.json`, `tmp/elsie-catalog-red.log`,
`tmp/elsie-spill-red.log`, `tmp/elsie-expression-fringe-red.log` and
`tmp/elsie-green.log`.

Only the profile, palette configurations, focused test and development note are
intended for Git. Source game files, generated images and temporary helpers
remain ignored. Installation, full repository checks and live-game verification
belong to the combined integration pass.
