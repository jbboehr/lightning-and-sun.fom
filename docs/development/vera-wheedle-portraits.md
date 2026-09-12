# Vera and Wheedle portraits

This batch adds all 32 main portrait strips for Vera and all 32 for Wheedle.
Each character has eight expressions in Spring, Summer, Autumn and Winter:
embarrassed, happy, mad, neutral, sad, think, ugh and wink. The paths match both
the original archive inventory and the game's native portrait definitions.
Every strip has two 296×180 frames with duration 0.2. There are 128 frame
occurrences and 126 unique frames; Wheedle's Autumn wink duplicates neutral.

Vera uses `PortraitsMisc` in every season. Wheedle uses the four corresponding
seasonal atlases. The registry retains these original metadata values. The
combined collection now contains 29 characters, 1,892 source animations
(1,875 portrait strips and 17 Adeline world animations), and 7,568 variants.
G cycles Vera's palette; T cycles Wheedle's. Both start on Vanilla, followed
by Debug Blue, Hayden, Ryis and Seridia. Shared Rust and GML runtime code is
unchanged.

## Art boundaries

The [Vera profile notes](vera-portraits.md) document six skin shades, including
fine eyebrow/nose blends and one brown outer speaking-mouth pixel per strip.
The latter was corrected after the author and independent reviewer identified
a warm fringe. Lipstick, mouth interiors, magenta hair and nails, earrings,
eyes, expression marks and clothes retain their source colors. All targets
use standard ramp roles and select the same 119,084 pixels.

The [Wheedle profile notes](wheedle-portraits.md) document ten shades, including
explicit target-ramp blends around the eyes and mouth. Hair and goatee retain
their original shading. The first mask recolored shared brown Winter clothing
details; the final mask preserves 1,248 glove, cuff and tie pixels while still
covering both exposed wrist patches. Each target selects 78,108 skin pixels.
These are source-bound component selections, with no runtime color guessing.

Two authors worked on separate character files. An independent art reviewer
inspected all distinct full Source/Blue/Ryis frames and enlarged faces, seasonal
body/hand comparisons and distinct mouth crops across all four targets. Final
outputs passed both art reviews after the Vera mouth and Wheedle Winter fixes.
The user accepted the offline previews. Original makeup contrast remains a
subjective choice; this acceptance does not establish in-game rendering.

## Offline review

- [Compact five-palette summary](../../../generated/vera-wheedle-preview/summary.png):
  the first Spring neutral frame for each character.
- [Complete Vanilla/Debug Blue review](../../../generated/vera-wheedle-preview/blue-review/index.html):
  eight small pages, one per character and season, each with eight expressions.

Each expression shows both frames side by side, full portraits at 2× and faces
at 4×. The detailed pages cover all 128 frame occurrences, or 256 Vanilla/Blue
views. They are static local HTML with separate PNG sheets and need no server,
JavaScript or game launch. The five-palette overview remains a separate spot check.

The review input manifest was built from the final combined bundle. An independent
pixel comparison checked all 256 distinct asset/frame/palette tuples against the
registry, all 26,005,248 placed pixels, source hashes, nearest-neighbor scaling
and unclipped opaque bounds. Chromium loaded all eight pages, decoded all 64
sheets and resolved every navigation link. The index and an expression page were
visually inspected in the browser.

## Verification

Fresh checks passed:

- Formatting, Clippy with warnings denied, all 87 active tests, both new opt-in
  corpus tests and the release build through the pinned Fenix Nix shell.
- All three existing opt-in GML interpreter tests.
- Registry integration alone and alongside the earlier characters. The new case
  first failed with `Unsupported character: vera`, then passed after registration.
- Exact validation of all 7,568 generated variants, including source geometry,
  alpha, metadata and recipe-selected pixels.
- Original PNG/metadata inventory, atlas and native portrait-definition checks
  for both new characters; all four standard/custom target shade rules.
- Byte comparisons showing all previous 27 character trees unchanged and both
  new characters identical to their reviewed standalone variants and originals.
- A fresh isolated MOMI installation into `tmp/vera-wheedle-playtest`, including
  the installer's packed vanilla/variant atlas and runtime-table verification.

The corpus tests include observed failing controls for Vera's missed mouth pixel
and accidental hair recoloring, and Wheedle's Winter glove spill and omitted eye
blend. Both final tests pass. Gallery reconstruction also reproduces each final
Blue output and metadata exactly.

The isolated installation's archive SHA-256 is
`7ccd10ddec84ecb1b6155e81f12070a4ef09a0284208f1cf00b881c4b173f4c1`.
The receipt retains the original SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The original `tmp/momi-lab/assets.bak.zip` was used because the supplied game
mount was empty; its hash remained unchanged. Native gameplay and a fresh
uninstall roundtrip were not run for this slice.

`tmp/play-characters` was rebuilt with a Nix output link. F7 starts the preview
on Vera; Shift+F4 changes character, Shift+F5 changes season, and Page Up goes
backward through expressions. The preview helper preserves the earlier world
scheduling fix, uses a separate state directory and is excluded from the player
package. The launcher requires the mounted game executable to be available.

Local evidence is retained under `tmp/vera-wheedle-*`: `final-checks.log`,
`integration-audit.json`, `native-coverage.log`, `all-variants-validation.log`,
`bundle-comparison.log`, `preview-check.log`, `blue-browser-check.json`,
`install-report.json`, `trial-prep.log` and the independent art review reports.
The bundle is `generated/characters-vera-wheedle-trial`. Game images, packages,
review pages and temporary helpers remain ignored; the source changes contain
only recipes, registry entries, tests and documentation.
