# Reina portrait authoring

This pass covers all **103 main portrait strips / 206 unique frames** in the
supplied archive: spring, summer, autumn, winter, beach, bathing, and wedding.
Overworld, UI, and child sprites are outside this profile.

The compact visual review is `generated/reina-summary.html` and
`generated/reina-summary.png`. The full original/Debug Blue sheets are in
`generated/reina-author-sheets/index.html`; the optional component editor is
`generated/review-reina-authored/index.html`. All 206 frames were inspected
during authoring. These generated game-derived images remain ignored by Git.

## Palette and masks

`palettes/profiles/reina-portraits.json` binds 2,846 component seeds to the
original strip hashes and dimensions. One explicit connected-color group
contains these eight colors:

| Source | Role | Debug Blue | Source-art use |
| --- | --- | --- | --- |
| `#B36644` | Light | `#9DB9D4` | Main face and body tone |
| `#9E512F` | Medium | `#7F9FBD` | Skin shading |
| `#7F382B` | Shadow | `#6687AD` | Deeper skin shading |
| `#541C1E` | Dark | `#445F83` | Fine skin outlines and shadows |
| `#AD5F3C` | Medium | `#7F9FBD` | Fine nose shading |
| `#A95835` | Medium | `#7F9FBD` | Fine bathing/beach body shadows |
| `#CB815C` | Light | `#9DB9D4` | Exposed fingertip highlights |
| `#B36844` | Light | `#9DB9D4` | Warm skin base beneath blush |

The last four shades were missing from the catalog sample ramp. Debug Blue
changes 425,164 pixels across the 103 strips while preserving 13,424 other
pixels that share those source colors. The exact excluded components are
recorded in the local `tmp/reina-exclusions.json` report. Only explicit source-bound seeds
and palette data are used by recoloring; temporary selection helpers are ignored
authoring artifacts.

The masks preserve shared-color spring cardigan stitching, inner shadows,
cuff buttons and belt buckle; autumn shirt ties; summer buttons; winter knit
textures and cuffs; and wedding earring shading, veil/curl accents and dress
detail. The spring midriff above the buckle, wrist and lower exposed triangle,
summer skin beneath the blouse, and wedding ear flesh are included. Independent
source inspection confirmed the midriff and six wedding earring pixels before
the final masks were generated.

Pink blush, eyes, lips and mouth interiors remain original. In particular,
`#581D1F`, `#762E21`, and `#B2684A` are preserved as mouth details; their small
warm pixels are deliberate. Wedding mesh, choker, lace and veil colors also
remain original as clothing. No remaining mask corrections were identified in
the author inspection; newly changed source artwork requires fresh review.

`palettes/stylized/reina-portraits.json` supplies Debug Blue, and
`palettes/review/reina.json` reloads the authored masks. The standalone set at
`palettes/sets/reina-portraits-trial.json` orders Debug Blue, Hayden, Ryis, then
Seridia. Each shared NPC palette copies the catalog's four target colors through
the shading roles above.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test reina_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/reina-portraits-study \
  --palette palettes/stylized/reina-portraits.json \
  --output generated/reina-portraits-study
target/release/mistria-palette validate \
  --original extracted/reina-portraits-study \
  --modified generated/reina-portraits-study \
  --palette palettes/stylized/reina-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/reina.json \
  --output generated/review-reina-authored
```

Use fresh output directories when regenerating. The opt-in corpus test checks
every expression's forehead/nose colors, independently selected exposed-skin
landmarks, 34 clothing/jewelry landmarks, dimensions, alpha, metadata, and all
pixels outside the source-color set. `FOM_REINA_RECIPE` selects a mutation recipe:
a catalog-only mapping failed on the omitted blush-base shade, and an
unrestricted eight-color mapping failed on an autumn shirt detail. The final
profile passed the same test.

All four target palettes passed exact recipe validation for all 103 strips.
The authored gallery reused all 206 masks without pending or conflicting frames.
Every applied frame matched the inspected author preview, and reconstructing the
profile from the gallery selections reproduced every output pixel.

Local evidence is in `tmp/reina-catalog-only-red.log`,
`tmp/reina-unmasked-red.log`, `tmp/reina-corpus-check.log`,
`tmp/reina-validation-report.json`, `tmp/reina-frame-verification.json`, and
`tmp/reina-authored-gallery-report.json`. The source archive retained SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Repository-wide checks and package integration are performed separately after
both parallel character authoring passes; this authoring pass did not install
or launch the game.

The user subsequently accepted the combined Reina/Juniper trial for committing,
while deferring possible mouth-color issues. See the
[batch record](parallel-portraits.md) for that shared art follow-up.
