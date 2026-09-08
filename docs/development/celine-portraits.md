# Celine portrait authoring

This pass covers all **183 main portrait strips / 366 unique frames** in the
supplied archive. Spring, summer, and autumn each have 22 regular and 22 gardening
strips; winter has 22, beach and bathing have 23, and wedding has six. Overworld,
UI, and child sprites are outside this profile.

The compact visual review is `generated/celine-summary.html` and
`generated/celine-summary.png`. Its six original/Debug Blue pairs include the
spring gardening hairstyle, summer neutral, autumn embarrassed, winter wink,
bathing, and wedding. All 366 frames were inspected in 33 full sheets at
`tmp/celine-{outfit}-{page}.png`. The optional editing gallery is
`generated/review-celine-authored/index.html`. These game-derived images and the
local authoring helpers remain ignored by Git.

## Palette and masks

`palettes/profiles/celine-portraits.json` binds 26,329 component seeds to the
original strip hashes and dimensions. Eleven source shades map through four
roles:

| Source | Role | Debug Blue | Source-art use |
| --- | --- | --- | --- |
| `#FCDEBE` | Light | `#9DB9D4` | Main face and body tone |
| `#F5BB90` | Medium | `#7F9FBD` | Skin shading |
| `#EA9866` | Shadow | `#6687AD` | Deeper skin shading |
| `#B35A35` | Dark | `#445F83` | Fine exposed-skin contours |
| `#9A4825` | Dark | `#445F83` | Fine nose shading |
| `#E28A55` | Shadow | `#6687AD` | Ear, nose, brow-adjacent and neck shading |
| `#954D24` | Dark | `#445F83` | Small outer-jaw contours |
| `#FBDFC0` | Light | `#9DB9D4` | Alternate pale hands and face details |
| `#EDA77D` | Medium | `#7F9FBD` | Alternate hand and facial shading |
| `#672115` | Dark | `#445F83` | Deep exposed-skin contours |
| `#D97C56` | Shadow | `#6687AD` | Five fine wedding neck pixels per frame |

The last seven shades were absent from the catalog's four sample points. Six
component groups separate `#B35A35`, `#E28A55`, `#954D24`, `#EDA77D`, and
`#672115` from the other six shades. This permits skin contours to change while
nearby same-color hair, clothing, and mouth details stay original. The final
profile changes **398,464 pixels** and preserves **137,463 matching-color pixels**
outside the selected skin components.

The masks preserve the hair crown, bangs, braids, loose hair and dark wedges
beside the neck; cloak bows, buttons, belts and chains; summer jewelry; swimsuit
frills; winter clothes; and wedding earrings, veil, flowers, gloves and dress.
Independent enlarged review confirmed the shared-color wedding hair wedges and
identified the additional wedding neck shade. The isolated brown pixel beneath
the neutral pink lip is preserved with the mouth. Open-mouth interiors, tongue,
lip highlights, pink blush, eyes and tears also remain original. Warm mouth-only
colors such as `#A5462C`, `#BA5E3F`, `#F3AC87`, and `#772E24` stay outside the
source-color set.

Temporary authoring rules suggested components using local geometry and color
neighbors. Every frame was inspected before export; only explicit source-bound
seeds are retained in the profile. No classifier runs during recoloring.

`palettes/stylized/celine-portraits.json` supplies Debug Blue, and
`palettes/review/celine.json` reloads the authored masks. The standalone set at
`palettes/sets/celine-portraits-trial.json` orders Debug Blue, Hayden, Ryis, then
Seridia. Shared NPC targets copy the catalog's four colors through the same
shading roles. Vanilla remains the implicit default in the combined package.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test celine_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/celine-portraits-study \
  --palette palettes/stylized/celine-portraits.json \
  --output generated/celine-portraits-study
target/release/mistria-palette validate \
  --original extracted/celine-portraits-study \
  --modified generated/celine-portraits-study \
  --palette palettes/stylized/celine-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/celine.json \
  --output generated/review-celine-authored
target/release/mistria-palette build-presets \
  --original extracted/celine-portraits-study \
  --presets palettes/sets/celine-portraits-trial.json \
  --output generated/celine-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks all
183 strips, dimensions, alpha and transparent RGB, byte-identical metadata,
unmapped colors, upper hair throughout the corpus, and literal skin, clothing,
hair and lower-lip landmarks selected independently of the mask seeds. Its
`FOM_CELINE_RECIPE` override supports mutation checks. An unrestricted catalog map
failed on the hair crown; a catalog-only map with the authored mask failed on a
bathing arm contour; the initial candidate failed the neutral lower-lip check;
and a recipe omitting the added wedding neck shade failed on its literal source
landmark. The final profile passed the same corpus test.

All four presets passed exact recipe validation for all 183 strips. The authored
gallery reused all 366 masks without pending or conflicting frames. Every applied
frame matched the inspected author preview, and reconstructing the profile from
the gallery selections reproduced every output pixel.

Independent verification passed all four targets against 47 source-inspected
landmarks, source hashes and dimensions, exact target colors, unchanged alpha
and transparent RGB, metadata, and unrelated pixels. All targets shared the
same changed-mask digest
`f05dd5a490a283106d5d8f0efa3e9296a000f890bddac6667b28c59768e92dd5`.
Its report is `tmp/celine-march-art-review-celine-verification.json`. No concrete
remaining art defect was found; fine lip and brow boundaries remain subject to
user preference.

Local evidence is in `tmp/celine-catalog-only-red.log`,
`tmp/celine-catalog-only-masked-red.log`, `tmp/celine-mouth-neck-red.log`,
`tmp/celine-neck-red.log`, `tmp/celine-corpus-green.log`,
`tmp/celine-final-checks.log`,
`tmp/celine-validation-report.json`, `tmp/celine-frame-verification.json`,
`tmp/celine-authored-gallery-report.json`, and
`tmp/celine-{preset}-validation.json`. The source archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This authoring pass did not install or launch the game. Repository-wide checks,
combined package integration and live-game verification belong to the Celine/March
integration pass. Small mouth-boundary judgments remain subject to the user's
visual checkpoint.
