# March portrait authoring

This pass covers all **181 main portrait strips / 362 frame occurrences / 361
unique frames** in the supplied archive. Spring, summer, autumn and winter each
contain 36 strips; beach and bathing contain 25; wedding contains 12. The first
wedding neutral frame exactly matches the wedding happy frame. Overworld, UI and
child sprites are outside this profile.

The compact review is `generated/march-summary.html` and
`generated/march-summary.png`: six original/Debug Blue pairs spanning the six
outfits, including bathing and an open mouth. All 361 unique frames were
inspected in the 31 sheets at `tmp/march-{outfit}-{page}.png`, with enlarged
face, neck, body and accessory comparisons. The optional component editor is
`generated/review-march-authored/index.html`. These game-derived images and
local authoring helpers remain ignored by Git.

## Source colors and boundaries

`palettes/profiles/march-portraits.json` records 43,971 component seeds, original
PNG hashes and dimensions. Thirteen source colors cover the catalog's four
samples plus nine fine skin shades:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#EEDDA5` | Main light skin | `#9DB9D4` |
| `#E6BD7C` | Light skin shadow | `#7F9FBD` |
| `#DCA966` | Middle skin shadow | `#6687AD` |
| `#CC8155` | Deep skin shadow | `#445F83` |
| `#AE542E` | Fine ear contours | `#445F83` |
| `#B36436` | Hand and body contours | `#445F83` |
| `#B4613E` | Fine face and body contours | `#445F83` |
| `#DFAB68` | Bathing/wedding alternate middle shade | `#6687AD` |
| `#E9C487` | Fine body highlights | `#7F9FBD` |
| `#EDCA92` | Fine arm highlight | `#7F9FBD` |
| `#7D3B14` | Deep exposed-skin contours | `#445F83` |
| `#B65932` | Wedding ear contours | `#445F83` |
| `#D0A179` | Hurt-expression cheek/body shading | `#6687AD` |

Four connected-color groups separate `#CC8155`, `#B4613E` and `#7D3B14` from
the other ten shades. This lets facial and body contours change while the same
colors in eyebrows, mouth interiors and necklace cords remain original. Only
explicit source-bound seeds are retained; temporary authoring rules are not
part of the runtime.

Independent enlarged review identified the spring/summer necklace cord among
those shared colors. The two cord sides remain original, including their
continuation toward the pendant, while the exposed skin between the cord and
undershirt recolors. Summer's top-right cord joins a few same-color pixels at
the neck edge; that connected boundary follows the accessory. Eyebrows and eye
details remain original. Open-mouth interiors, tongue, blush and tears remain
original; the separate lower-lip skin shadow changes with the face.

The masks preserve shirts, aprons, jacket sleeves, gloves, wedding clothing,
hair, earrings, hardware and bathing towel. Exposed fingertips, wrist windows,
body hatching and chest shading recolor. Debug Blue changes **834,678 pixels**
and preserves **9,717 other matching-color pixels** across the source strips.

## Presets

`palettes/sets/march-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. Each shared NPC target copies its catalog's four colors through the
roles above, using the same profile. Vanilla is implicit in the installed
control; integration assigns March to **U**.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test march_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/march-portraits-study \
  --palette palettes/stylized/march-portraits.json \
  --output generated/march-portraits-study
target/release/mistria-palette validate \
  --original extracted/march-portraits-study \
  --modified generated/march-portraits-study \
  --palette palettes/stylized/march-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/march.json \
  --output generated/review-march-authored
target/release/mistria-palette build-presets \
  --original extracted/march-portraits-study \
  --presets palettes/sets/march-portraits-trial.json \
  --output generated/march-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all strips, dimensions, alpha, metadata, unrelated colors and every expression's
main face tone, plus 28 independently chosen skin landmarks, 18 accessory/brow
landmarks, an open-mouth interior and a separate lower-lip shadow. It accepts
`FOM_MARCH_RECIPE` for mutation checks. The catalog-only mapping failed on the
omitted hurt-cheek shade. The unrestricted mapping failed on the protected
open-mouth pixel. A dedicated regression failed on the recolored spring
necklace before its mask correction; the final profile passed the same test.

All four targets passed exact recipe validation for all 181 strips. The authored
gallery reused all 362 occurrences with no pending or conflicting unique
frames. Reconstructing a profile from the gallery selections reproduced every
output pixel, and all applied frames matched the inspected author previews.
The six-example summary was inspected after the corrections.

Local evidence is in `tmp/march-catalog-only-red.log`,
`tmp/march-unmasked-red.log`, `tmp/march-necklace-red.log`,
`tmp/march-corpus-green.log`, `tmp/march-*-validation.json`,
`tmp/march-frame-verification.json`, `tmp/march-authored-gallery-report.json`
and `tmp/march-final-inputs.sha256`. The archive used is SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This authoring pass changes declarative recipes and a focused corpus test.
Combined package generation, installation, full repository verification and
live-game checks belong to the Celine/March integration pass. No source game
files or generated images are staged by this workflow.

Independent final art verification passed all four targets against 47 additional
source-inspected landmarks, original PNG hashes/dimensions, alpha and transparent
RGB, unchanged metadata, exact target mappings and identical changed masks. Its
changed-mask digest is
`d8f5fdcdfda2c50ecf6f4cd21e9211f187cf2e510ee5470f59d61b39181098e6`;
evidence is `tmp/celine-march-art-review-march-verification.json`. Final enlarged
Blue necklace and Ryis-palette wedding brow/mouth crops were inspected. No
further art correction was requested by that review; small pixel-boundary
judgments remain subject to the user's compact visual checkpoint.
