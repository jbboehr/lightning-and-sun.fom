# Errol portrait authoring

This profile covers all 36 main Errol portrait strips in the supplied archive.
Each of Spring, Summer, Autumn and Winter has nine expressions: embarrassed,
happy, mad, neutral, neutral_closed, sad, think, ugh and wink. Every strip has
two 296×180 frames, and all 72 frames are unique. This corpus has no beach,
bathing or wedding portraits. Overworld and UI assets are outside the profile.

Source PNGs and metadata are under
`assets/animations/NPCs/Errol/Portraits/{Season}/`. Each season's nine metadata
files use the corresponding `PortraitsSpring`, `PortraitsSummer`,
`PortraitsAutumn` or `PortraitsWinter` atlas. All source and generated images
and temporary Rust helpers remain ignored.

## Colors and boundaries

The profile records 5,024 component seeds, source PNG hashes and dimensions.
The catalog's four samples omit fine shades used on the face, ears, wrists,
neck and chest:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#F4C49C` | Main skin | `#9DB9D4` |
| `#DDAB82` | Middle skin shading | `#7F9FBD` |
| `#C38C70` | Skin shadow | `#6687AD` |
| `#9B6656` | Deep skin shading | `#445F83` |
| `#724A3E` | Deep eye, ear, nose, outer-lip and wrist contours | `#445F83` |
| `#85584B` | Fine deep ear and Summer neck shading | `#445F83` |
| `#C9927B` | Fine Summer neck and chest shadows | `#6687AD` |
| `#A77E6A` | Shadowed skin infill between wrist hairs | `#6687AD` |
| `#D3B296` | Lit skin fringes beside wrist and chest hairs | `#7F9FBD` |

Five connected-color groups isolate the fine shades from the main skin group.
Explicit source-bound components select the exposed skin and
outer lips while protecting the actual speaking-mouth interior contour.
The white and gray eyebrows, moustache, beard and body hair stay original.
Skin beside those features follows the target, including isolated wrinkles
and the partly hidden left wrist in Spring and Summer.

The gray hair strokes use `#897C6C` and `#BCB2A5`. Warmer `#A77E6A` and
`#D3B296` pixels between and beside them follow the skin palette, including
the matching Summer chest fringes. See the [hair-fringe correction](#hair-fringe-correction)
for the source coordinates and bounded verification. The dark `#4D2D24` pixels
inside Happy's black eye contour are eye
detail and stay original. Hats, clothes, neckwear, suspenders, cuffs, buttons,
watch chains and the Summer flower likewise retain their source colors.

The closed-mouth lower shadow recolors, as do the speaking lower arc and its
skin-facing corners. In the neutral speaking frame, the `#724A3E` upper run
at `[151..156,66]` and interior corners `[149,67]` and `[158,67]` stay original.
The lower arc at `[151..156,69]` and corners `[150,68]` and `[157,68]` recolor.
This distinction applies consistently to all expressions, including Ugh's
mouth shifted down one pixel. The existing `#522F14` and `#A1121D` interior
colors and pink tongue colors stay original. Actual Blue and Ryis closed and
speaking mouths were inspected; no cosmetic palette adjustment was needed.

Each target selects and changes 138,492 pixels while preserving 160 other
pixels sharing the nine source colors. Those protected matching pixels are
the speaking-mouth interior components. Temporary authoring rules are not
runtime code; the stored recipe uses only the recorded components.

## Presets and verification

`palettes/sets/errol-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. The natural targets use their catalog's four shades through the roles
above, with the same selection for every target. Vanilla is implicit in the
installed controls. Integration assigns Errol to X.

```sh
nix-shell --pure --run 'cargo test --locked --test errol_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/errol-portraits-study \
  --presets palettes/sets/errol-portraits-trial.json \
  --output generated/errol-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/errol.json \
  --output generated/errol-review-authored
```

Use fresh output directories when regenerating. The corrected authoring build
is `generated/errol-arm-final/variants/`. The authored gallery reuses all 72 frames
with no pending or conflicting selections. Rebuilding the profile from those
gallery selections reproduces every applied pixel.

The opt-in test checks all 36 strips for dimensions, alpha, unchanged metadata,
seasonal atlas, frame count, untouched unrelated colors and exact mapped colors.
Its 617 literal source landmarks include 548 skin/outer-lip points and 69
protected points. All nine shades are represented. Every expression checks
the closed and speaking lower lip independently of the protected upper
interior. `FOM_ERROL_RECIPE` accepts a temporary recipe for omission and spill
controls.

The catalog-only control failed on the omitted outer-lip shade at Autumn
embarrassed `[152,68]`. The unrestricted seven-color control then failed on
the speaking upper interior in the same strip. Both failures were observed
before the mapping and mask corrections. The final test passes.

All four targets passed exact validation of 36 PNGs and 36 metadata files.
The author inspected every unique full frame, all 72 actual enlarged
Source/Blue/Ryis face and mouth trios, and the four distinct seasonal hand/body
trios. An independent review also inspected the entire corpus and passed all
144 target strips with 41 separately chosen source landmarks per target.
No further concrete omission or spill remained before the stable-data handoff.

The initial pass's local evidence includes `tmp/errol-catalog-red.log`, `tmp/errol-spill-red.log`,
`tmp/errol-final-test.log`, `tmp/errol-*-validation.json`,
`tmp/errol-frame-verification.json`, `tmp/errol-final-faces-*.png`,
`tmp/errol-final-hands-0.png` and `tmp/errol-final-inputs.sha256`. Independent
evidence is `tmp/elsie-errol-art-review-errol-verification.json`. The shared
selection digest is
`1e31b04db9af6addd8d40a93d1f1e53fadf8565341c0217c42c9b2f7dde061b4`.
The supplied archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This pass changes declarative art recipes and a focused source-corpus test.
The integration pass owns the combined preview, bundle, installation, full
repository checks and live-game testing. No game source or generated images
are staged by this workflow.

## Hair-fringe correction

User review found warm pixels around the wrist hairs that the initial pass
had classified as hair. The corrected mask retains the core gray strokes and
maps the warmer infill to the adjacent skin roles. The same distinction applies
to the Summer chest hair.

Each Spring/Summer frame adds eight `#A77E6A` pixels at `[106..107,161]`,
`[109..112,162]`, and `[110..111,164]`, plus two `#D3B296` pixels at
`[219,146..147]`. Summer adds five chest pixels at `[157,100]`, `[158,101]`,
`[164,100]`, `[165,104]`, and `[165,105]`. The second frame repeats those
coordinates with x offset 296. Autumn and Winter have no corresponding shades.

The updated test first failed at Spring `[106,161]`. An arm-only candidate then
failed at Summer chest `[157,100]`, before applying the same rule there.
Three earlier protected-landmark expectations were corrected to check actual
gray strokes beside those warm pixels instead. The final test adds all 450
corrected locations across all expressions and both frames, retaining its
other skin, mouth, eye, clothing and alpha checks. In particular, Summer
`[156,100]` stays gray beside the corrected `[157,100]`.

An exact before/after comparison passes all four targets: precisely 450 pixels
change per target across 18 strips, with no other pixel or metadata changes.
This also verifies that all core hair strokes, face details and accessories
retain their accepted output. Independent enlarged Blue/Ryis wrist and chest
review passes. All 72 authored gallery frames are reused, and the reconstructed
Blue output matches the corrected pixels and metadata exactly.

Formatting, Clippy, all 87 active tests, the Elsie/Errol local tests and release
build pass. All 6,256 combined variants validate. Every other character's
complete output tree matches `generated/characters-elsie-errol-trial` byte-for-byte.
The updated bundle is `generated/characters-errol-arm-trial`; the compact PNG
is `generated/errol-arm-preview/summary.png`.

Final profile SHA-256 is
`99805e40f14be4a18f18b9ec34efe6467bfc49f4d39ccd858e06fc129bda29ec`.
Evidence includes `tmp/errol-arm-red.log`, `tmp/errol-arm-chest-red.log`,
`tmp/errol-arm-final-checks.log`, `tmp/errol-arm-combined-delta.json`,
`tmp/errol-arm-review-final.md`, `tmp/errol-arm-gallery-roundtrip-check.log`,
`tmp/errol-arm-all-variants-validation.log`,
`tmp/errol-arm-previous-output-check.log`, and
`tmp/errol-arm-bundle-inputs.sha256`.

A fresh CLI installation into `tmp/errol-arm-playtest` verified installed
Vanilla/variant atlas pixels, metadata and scripts. The completed headless game
run checked all 36 Errol portraits across all five choices, plus one Elsie
portrait: exactly 37 distinct sources and 185 source/preset pairs. X cycling,
season wraps, independent choices, fractional phase and a hidden console pass.
The complete character loop preserves Errol's choice. The log audit passes
without script errors or disabled callbacks, and the game exits gracefully.
An earlier interrupted run was discarded before this complete run.

The updated summary, wrist close-up and Ryis game screenshot were inspected.
`tmp/play-characters` now opens this corrected isolated copy on Errol; F7 opens
the preview and X cycles his palette. The Nix launcher was rebuilt and checked
as an executable shell script. The source archive's hash remains unchanged.
Desktop play, user visual acceptance and uninstall were not rerun. Nothing was
staged or committed, and all game-derived files remain ignored.

Installation and live evidence are `tmp/errol-arm-install-report.json`,
`tmp/errol-arm-live-run.log`, and `tmp/errol-arm-live-audit.json`.

The user accepted the corrected Elsie/Errol checkpoint. Fresh formatting,
Clippy, all 87 active tests, both local portrait tests and the release build
passed before the commit in `tmp/elsie-errol-commit-checks.log`. Installation
and gameplay were not repeated for the commit.
