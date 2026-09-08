# Ryis portrait authoring

Ryis is the next character in the one-character-at-a-time portrait plan. His
darker source skin palette provides a different authoring case from Adeline and
Hayden. This pass covers all **109 main portrait strips / 218 unique frames** in
the supplied archive: spring, summer, autumn, winter, beach, bathing, and wedding.
Overworld, UI, and child sprites are outside this pass.

The user-facing preview is `generated/ryis-summary.html`, with the same six
original/Debug Blue pairs in `generated/ryis-summary.png`. Mask selection and
the full-corpus review were handled during authoring; the user does not need to
approve individual components. The user reviewed the six-example sheet and
accepted this art checkpoint.

## Recipe

- `palettes/profiles/ryis-portraits.json` contains source hashes, dimensions, and
  1,078 component seeds across the 109 strips.
- `palettes/stylized/ryis-portraits.json` supplies Debug Blue.
- `palettes/review/ryis.json` rebuilds the optional editing gallery with these
  masks already loaded.

The four catalog samples are `#B06C57`, `#814A3A`, `#63342A`, and `#491F1B`.
Source inspection added `#995B48` for fine face/body shading, `#A5624E` for chest
and arm details, and `#D6906A` / `#C27B55` for exposed fingertips in the gloves.
Those extra colors were absent from the catalog sample ramp. Debug Blue changes
427,816 pixels across both frames of every strip.

The masks preserve shared-color glove seams, scarf details, jacket stitching,
wedding lapels and vest seams, and towel edging. Hair, shaved side hair, eyes,
mouth interiors, and pink blush remain original. The eight source colors share
one connected-component group; separate groups were not needed for this corpus.
Temporary selection rules were checked against all 218 frame previews. Only the
explicit source-bound seeds and palette data are retained for recoloring.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test ryis_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/ryis-portraits-study \
  --palette palettes/stylized/ryis-portraits.json \
  --output generated/ryis-portraits-study
target/release/mistria-palette validate \
  --original extracted/ryis-portraits-study \
  --modified generated/ryis-portraits-study \
  --palette palettes/stylized/ryis-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/ryis.json \
  --output generated/review-ryis-authored
```

Use fresh output directories when rerunning generation. The opt-in corpus test
checks face shading in every expression, a forehead landmark in every frame,
body/fingertip details, 24 shared-color clothing/accessory landmarks, dimensions,
alpha, metadata, and every pixel outside the source-color set. It accepts
`FOM_RYIS_RECIPE` for mutation checks. A catalog-only mapping failed on a missed
face shade; an unrestricted swap failed on an autumn jacket detail.

All 109 applied strips passed exact recipe validation. Every applied frame
matched its authored preview. The rebuilt gallery reused all 218 frame masks,
with no pending or conflicting groups. Reconstructing the profile from those
gallery selections reproduced every output pixel. The six-example sheet was
also visually inspected.

Final verification passed formatting, Clippy, all 86 normal tests, the Ryis
corpus test, and the release build. This pass changes recipe data and tests;
the shared Rust and GML runtime are unchanged.

Local evidence lives in `tmp/ryis-catalog-only-red.log`,
`tmp/ryis-unmasked-red.log`, `tmp/ryis-final-checks.log`,
`tmp/ryis-validation-report.json`, `tmp/ryis-frame-verification.json`,
`tmp/ryis-authored-gallery-report.json`, and
`tmp/ryis-gallery-roundtrip-report.json`. The source archive still has SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Extracted portraits, generated recolors, previews, and authoring programs are
ignored by Git.

The accepted art checkpoint was committed before integration. Ryis now has a
standalone preset set and is included in the combined trial with independent
Vanilla / Debug Blue choices on F10. The installable recolors match these offline
outputs exactly. See [character configuration](characters.md#ryis-integration-2026-09-07)
for installation and live-game verification. The user also tried the integrated
trial on their desktop and accepted its appearance.
