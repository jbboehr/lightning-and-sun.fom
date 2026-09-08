# Balor portrait authoring

This profile covers all **110 main portrait strips** in the supplied archive:
21 each for spring, summer, autumn, and winter; 20 beach/bathing strips; and six
wedding strips. Their 220 frame occurrences contain **210 unique frames**.
Overworld, UI, and child sprites are outside this profile.

The current mouth comparison is `generated/balor-mouth-preview/index.html` and
its `summary.png`, showing original, previous Ryis, and corrected Ryis frames.
The initial review at `generated/balor-summary.html` and
`generated/balor-summary.png` has six original/Debug Blue pairs showing spring,
summer, autumn embarrassed, winter wink, bathing, and wedding. All 210 unique
frames were inspected in 21 sheets at `tmp/balor-{outfit}-{page}.png`, with enlarged
checks around the face, wrist wraps, torn sleeve, and bathing torso. The optional
editing gallery is `generated/review-balor-mouth-corrected/index.html`. Game-derived
images and temporary authoring helpers remain ignored by Git.

## Palette and masks

`palettes/profiles/balor-portraits.json` binds **66,609 component seeds** to the
original strip hashes and dimensions. Seven source shades map through four roles:

| Source | Role | Debug Blue | Source-art use |
| --- | --- | --- | --- |
| `#FCDEBE` | Light | `#9DB9D4` | Main face and body tone |
| `#EFA67A` | Medium | `#7F9FBD` | Skin shading |
| `#C16640` | Shadow | `#6687AD` | Deeper skin shading |
| `#914A2C` | Dark | `#445F83` | Dark skin contours |
| `#F4BF98` | Medium | `#7F9FBD` | Fine face, torso, and arm shading |
| `#A75939` | Dark | `#445F83` | Small nose detail |
| `#612026` | Dark | `#445F83` | Deep nose, neck, underarm, and finger contours |

The last three shades were absent from the catalog's four sample points. Four
component groups keep `#EFA67A`, `#C16640`, and `#612026` separate from the other
four shades. This lets the skin change while same-color dark mouth contours,
brows, and red clothing stay original. The profile changes **424,386 pixels** and
preserves **34,593 matching-color pixels** outside the selected skin components.

The dark wrist wraps have open gaps that reveal skin, and the pale oval in the
spring sleeve is a tear exposing skin. Both recolor consistently, including deep
shadows between the wraps. The wrap material, sleeve fabric, winter glove, coat,
scarf, clasps, hair, and wedding clothes stay original. The pink diagonal chest
scar (`#F8B5A2` and `#FDC5B6`) retains its pigmentation, as does blush. The
`#EFA67A` and `#C16640` lip edges and lower-lip shadows follow the selected skin
palette. Dark smile corners, open-mouth interiors, tongue, and eyes stay original.

Temporary authoring rules suggested components from local geometry and color
neighbors. Every frame was inspected before export; the profile retains explicit
source-bound seeds. No classifier runs during recoloring.

`palettes/stylized/balor-portraits.json` supplies Debug Blue, and
`palettes/review/balor.json` reloads the authored masks. The standalone set at
`palettes/sets/balor-portraits-trial.json` orders Debug Blue, Hayden, Ryis, then
Seridia. Shared NPC targets copy the catalog's four colors through the same roles.
Vanilla remains the implicit default in the combined package.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test balor_portraits -- --ignored'
target/release/mistria-palette apply \
  --input extracted/balor-portraits-study \
  --palette palettes/stylized/balor-portraits.json \
  --output generated/balor-mouth-study
target/release/mistria-palette validate \
  --original extracted/balor-portraits-study \
  --modified generated/balor-mouth-study \
  --palette palettes/stylized/balor-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/balor.json \
  --output generated/review-balor-mouth-corrected
target/release/mistria-palette build-presets \
  --original extracted/balor-portraits-study \
  --presets palettes/sets/balor-portraits-trial.json \
  --output generated/balor-mouth-trial
```

Use fresh output directories when regenerating. The first opt-in corpus test
checks all 110 strips, dimensions, alpha and transparent RGB, byte-identical metadata,
unmapped colors, and 23 literal skin, clothing, mouth, brow, and scar landmarks
selected independently of the mask seeds. Its `FOM_BALOR_RECIPE` override supports
mutation checks. A catalog-only map failed on the bathing arm's missing `#F4BF98`
shade at `[178,77]`. An unrestricted seven-color map failed on the protected
eyebrow at `[148,53]`. The final masked profile passed the same test.

In the initial authoring pass, all four presets passed exact recipe validation
for all 110 strips. The gallery reused all 210 groups without pending or
conflicting frames. Every applied frame matched the inspected author preview,
and reconstructing the profile from gallery selections reproduced all 220 frame
occurrences exactly.

Initial independent verification passed all four targets against 47 source-inspected
landmarks, source hashes and dimensions, target colors, unchanged alpha and
transparent RGB, metadata, and unrelated pixels. All targets shared changed-mask
digest `09547b6d23a66e44c88bd86eb094e748ecdd8d8e214213827a4a2d334d82aa71`.
The report is `tmp/balor-valen-art-review-balor-verification.json`. This records
the initial profile, before the mouth correction below.

### Mouth correction

Desktop review found peach-colored flecks around the mouth, especially with the
Ryis palette. The initial mask excluded the entire smile, including skin-colored
lip shading and the separate shadow below the lower lip. Adding the reviewed
`#EFA67A` and `#C16640` components fixes those missed areas while preserving
`#612026` mouth contours and the red/pink interior. This adds 1,455 seeds and
1,720 changed pixels across the 220 frame occurrences. All 62 distinct mouth
crops, covering all 210 unique frames, were inspected for open-mouth exceptions.

The second corpus test exercises the actual Ryis preset through `apply`. It
checks nine recolored lip landmarks and three preserved mouth landmarks in each
of the seven neutral strips, covering both animation frames and all outfits.
It first failed on the peach pixel at `[148,70]` and then passed after the mask
change. The former assertion preserving the peach smile center was replaced by
this explicit recolor contract; the protected dark-corner assertions remain.
Both local corpus tests passed alongside formatting, Clippy, the 87 active
synthetic tests, and the release build.

The corrected combined output is `generated/characters-balor-mouth-trial`.
All four Balor presets passed exact recipe validation, and the refreshed gallery
is `generated/review-balor-mouth-corrected/index.html`, with all 220 frame
occurrences reused and none pending or conflicting. Independent pixel comparison
confirmed that each target changes only the same 1,720 additional mouth pixels;
all other characters' images and metadata are byte-identical to the initial
batch. Independent visual review also passed all 48 distinct tight mouth crops
from the actual outputs, comparing original, previous Ryis, corrected Ryis, and
corrected Blue. The corrected changed-mask digest is
`cb81e1c6de5b6c2a6a42edb28614685a3a7c20b56b98fb6c82a697201f5c3e3c`.
The report is `tmp/balor-mouth-final-art-verification.json`. Diagnostic
comparisons and the red/green test logs are under `tmp/balor-mouth-*`.

Other local evidence is in `tmp/balor-test-red-catalog.log`,
`tmp/balor-test-red-unmasked.log`, `tmp/balor-test-green.log`,
`tmp/balor-frame-verification.json`, `tmp/balor-authored-report.json`, and
`tmp/balor-{preset}-validation.json`. The source archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

Combined package integration and live-game verification are recorded in the
[Balor/Valen integration pass](balor-valen-portraits.md).
