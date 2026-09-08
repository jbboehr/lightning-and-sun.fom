# Olric portrait authoring

This pass covers all **36 main portrait strips / 72 unique frames** in the
supplied archive. Each season has nine strips: neutral, embarrassed, happy,
mad, sad, think, ugh, wink and bunny-ears neutral. Every strip contains two
296×180 frames. There are no beach, bathing or wedding portraits in this
corpus. Overworld and UI assets are outside this profile.

The current combined preview is `generated/olric-eyes-preview/index.html`.
The original compact reviews are `generated/olric-summary.html` for Debug Blue and
`generated/olric-ryis-summary.html` for Ryis: six original/recolored pairs
covering all four seasons, bunny ears, exposed arms and a speaking mouth.
All 72 unique frames were inspected in the eight sheets at
`tmp/olric-{outfit}-{page}.png`, with enlarged face, mouth, neck and arm crops.
The nine poses' closed and speaking faces were also inspected using
actual Ryis output. The optional component editor is
`generated/olric-review-authored/index.html`. Game-derived images and local
Rust authoring helpers remain ignored by Git.

## Source colors and boundaries

`palettes/profiles/olric-portraits.json` records 7,866 component seeds, original
PNG hashes and dimensions. The catalog's five samples need three additional
skin shades for the face, neck, arm and wrist details:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#F2DBA7` | Main light skin | `#9DB9D4` |
| `#DDA966` | Light skin shadow | `#7F9FBD` |
| `#C37B43` | Middle skin shadow and lower-lip shading | `#6687AD` |
| `#B7672F` | Deep skin shading | `#445F83` |
| `#813E16` | Deep nose, ear, jaw and body contours | `#445F83` |
| `#E5C586` | Fine pale face, neck and arm shading | `#9DB9D4` |
| `#B65932` | Fine neck and underarm contours | `#445F83` |
| `#CB884E` | Wrist shading beside the gloves | `#6687AD` |

Three connected-color groups separate `#813E16` and `#B7672F` from the other
six colors. Both deep shades appear in skin and eyebrow details. Brown brow
fringes and eye creases follow the skin palette; black brows and lashes, iris
details, and the deepest speaking-mouth contours stay original. Lip edges and the
separate lower-lip shadow follow the target palette. The speaking frame's shifted
lower-lip shadow is included. Pink tongues, mouth interiors, blush and tears
remain original.

Hair, gloves, shirts, jackets, belts, straps, buckles and bunny ears retain their
colors. Exposed face, ears, neck, chest, arms and the narrow skin beside cuffs
recolor. Only explicit source-bound seeds are retained in the profile; temporary
authoring selection rules are not part of the runtime.

Each target changes **157,432 pixels** and preserves **432 other pixels that
share source colors** across the strips.

## Presets and atlases

`palettes/sets/olric-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. Each NPC target copies its catalog's four colors through the roles
above, using the same profile. Vanilla is implicit in the installed control;
integration assigns Olric to **K**.

The original metadata maps nine strips each to `PortraitsSpring`,
`PortraitsSummer`, `PortraitsAutumn` and `PortraitsWinter`. Bunny ears use the
native seasonal outfit names such as `spring_bunny_ears` with expression
`neutral`; their portrait filenames are still inside the seasonal directories.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test olric_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/olric-portraits-study \
  --palette palettes/stylized/olric-portraits.json \
  --output generated/olric-portraits-study
target/release/mistria-palette validate \
  --original extracted/olric-portraits-study \
  --modified generated/olric-portraits-study \
  --palette palettes/stylized/olric-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/olric.json \
  --output generated/olric-review-authored
target/release/mistria-palette build-presets \
  --original extracted/olric-portraits-study \
  --presets palettes/sets/olric-portraits-trial.json \
  --output generated/olric-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all 36 strips, dimensions, alpha, metadata, unrelated colors and the main
face/neck tone in every expression. Its 71 literal source landmarks cover all
eight skin shades, eye and brow shading, black outlines and iris details,
lighter lip edges, the lower-lip shadow and dark
mouth contours/tongues. It accepts `FOM_OLRIC_RECIPE` for mutation checks. The
catalog-only mapping failed on omitted `#E5C586` at `[152,52]` in spring neutral;
the initial unrestricted eight-color mapping failed on the eyebrow at `[138,49]`.
The eye follow-up below intentionally revises that brown-fringe expectation.

All four targets passed exact recipe validation for all 36 strips. The authored
gallery reused all 72 frames with no pending or conflicting selections.
Reconstructing a profile from the gallery selections reproduced every output
pixel, and all applied frames matched the inspected author previews. Both
six-example summaries were inspected.

Local evidence is in `tmp/olric-catalog-red.log`,
`tmp/olric-unrestricted-red.log`, `tmp/olric-green.log`,
`tmp/olric-*-validation.json`, `tmp/olric-verify-frames-report.json`,
`tmp/olric-authored-report.json` and `tmp/olric-final-inputs.sha256`.
The archive used is SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

Initial independent art verification passed all four targets against 41 additional
source-inspected landmarks, original PNG hashes/dimensions, alpha and transparent
RGB, unchanged metadata, exact target mappings and identical changed masks.
All 72 unique frames and 48 deduplicated actual source/Blue/Ryis face crops were
inspected. The initial changed-mask digest, before the eye follow-up below, is
`4b58833c94c0065ef4eeecd791a65c2d365c6c7a6260fd0c315b4fd4f816db6b`.
Evidence is `tmp/eiland-olric-art-review-olric-verification.json`. No further
correction was requested by that review.

This authoring pass changes declarative recipes and a focused corpus test.
Combined package generation, installation, full repository verification and
live-game checks belong to the integration pass. No source game files or
generated images are staged by this workflow.

## Eye and eyebrow follow-up

After accepting Eiland's correction, the user reported missed pixels around
Olric's eyes and eyebrows. The original selections excluded brown fringe and
crease components beside the black outlines, leaving warm spots surrounded by
recolored skin. The profile now includes those components across all 36 strips,
including the four seasonal bunny-ears portraits: 1,264 additional seeds and
1,416 additional pixels per target. Source colors, groups, hashes, and every
previous seed remain unchanged. Black brows and lashes, iris detail, mouth
interiors, and all pixels outside these additions are preserved.

The expanded regression first failed on a missed brow pixel at `[138,49]`, then
passed with the correction. It checks 28 eye/brow shading landmarks and 12
protected outline/iris landmarks across the two Spring neutral frames, alongside
the existing face, body, and mouth checks. An unrestricted replacement still
fails on the protected mouth interior at `[443,66]`. All 48 distinct
source/Blue/Ryis face crops covering the 72 frames were inspected.

Formatting, Clippy, all 87 active tests, both Eiland/Olric corpus tests, and the
release build passed. All 5,200 variants in the rebuilt package at
`generated/characters-olric-eyes-trial` passed exact recipe validation. Comparing
with the accepted Eiland build confirmed that only the inspected 1,416 pixels
per Olric target changed. The other ten character output trees, including
Eiland, were byte-identical.

MOMI installed the package in the separate `tmp/olric-eyes-playtest` copy and
verified its pixels, metadata, and scripts. A fresh game session passed all five
choices for all 36 Olric portraits and an Eiland expression: 185 distinct
source/palette pairs. K's full cycle, bunny ears, all four outfits, and outfit
wrap passed. Captured Spring Blue/Ryis portraits were inspected. The shared
`./tmp/play-characters` launcher opens this build; F7 opens the preview, F4 moves
from Eiland to Olric, and K cycles Olric's palette. The user accepted the updated
combined summary and approved committing the batch. The previous full
eleven-character matrix and uninstall roundtrip were
not repeated.

Local evidence includes `tmp/olric-eyes-red.log`, `tmp/olric-eyes-unrestricted-red.log`,
`tmp/olric-eyes-final-checks.log`, `tmp/olric-eyes-audit.json`,
`tmp/olric-eyes-validation-count.txt`, `tmp/olric-eyes-install-report.json`, and
`tmp/olric-eyes-live-run.log`. The corrected profile SHA-256 is
`23d6c49a9cd88107907e0bfea36a4bfcd58374ddd1694c34a325d4f766059d79`.
