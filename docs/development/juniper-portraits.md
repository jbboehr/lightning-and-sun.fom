# Juniper portrait authoring

This pass covers all **132 main portrait strips / 264 unique frames** in the
supplied archive: spring, summer, autumn, winter, beach, beach accident, bathing,
and wedding. Overworld, UI, and child sprites are outside this pass.

The compact review is `generated/juniper-summary.html` and
`generated/juniper-summary.png`. Its six original/Debug Blue pairs show spring
neutral, summer blush, autumn wild laugh, winter wink, bathing, and wedding.
All 264 frame previews were inspected during authoring. The 26 full sheets are
`tmp/juniper-{outfit}-{page}.png`; the optional editing gallery is
`generated/review-juniper-authored/index.html`.

## Source colors and boundaries

`palettes/profiles/juniper-portraits.json` records 26,200 component seeds, original
PNG hashes, and dimensions. `palettes/stylized/juniper-portraits.json` supplies
Debug Blue. The four catalog samples alone miss fine body shading and the
bathing portrait's alternate middle shade.

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#EFD89A` | Main light skin | `#9DB9D4` |
| `#E3BF7F` | Light skin shadow | `#7F9FBD` |
| `#D2AB66` | Middle skin shadow | `#6687AD` |
| `#B87D4C` | Deep skin shadow | `#445F83` |
| `#763F21` | Fine face/body contours | `#445F83` |
| `#592323` | Deepest exposed-skin contours | `#445F83` |
| `#A16C3D` | Fine chest shading | `#445F83` |
| `#E9CC8B` | Fine belly highlights | `#9DB9D4` |
| `#D29C66` | Beach torso details | `#6687AD` |
| `#965532` | Small spring/autumn chest detail | `#445F83` |
| `#DFAB68` | Bathing portrait middle shade | `#6687AD` |
| `#9D5A33` | Bathing shoulder detail | `#445F83` |

Four component groups separate `#763F21`, `#592323`, and `#DFAB68` from the other
nine colors. This permits skin contours to change while preserving nearby
same-color jewelry, clothing, and mouth details. No authoring classifier is used
at runtime; only the explicit source-bound component seeds are retained.

The masks preserve tiara, pendant, bracelet and armband outlines; wedding dress
ties and bodice borders; the bathing towel's lower fringe and hem; ordinary and
wide-laugh mouth interiors; eye makeup, lips, blush and tears; and all hair.
Winter's dark mesh and gloves remain original, while its small exposed upper-arm
window and side contours recolor. The green beach-accident coating remains
original, with the visible skin around it recolored. The light peach blush
transition `#F2C596` and pale tear detail `#FAE7B3` are intentionally outside the
source-color set.

The final masks change **642,258 pixels** and preserve **9,996 matching-color
pixels** outside the selected skin components. Independent enlarged review found
and corrected wedding ties, armband rims, and a winter bracelet pixel. The
shifted arm and armband in laughing poses were checked separately.

## Presets

`palettes/sets/juniper-portraits-trial.json` lists Debug Blue first, followed by
`npc_hayden`, `npc_ryis`, and `npc_seridia`. Each catalog target's four colors are
copied through the same shading roles above. All presets use the same profile.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test juniper_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/juniper-portraits-study \
  --palette palettes/stylized/juniper-portraits.json \
  --output generated/juniper-portraits-study
target/release/mistria-palette validate \
  --original extracted/juniper-portraits-study \
  --modified generated/juniper-portraits-study \
  --palette palettes/stylized/juniper-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/juniper.json \
  --output generated/review-juniper-authored
target/release/mistria-palette build-presets \
  --original extracted/juniper-portraits-study \
  --presets palettes/sets/juniper-portraits-trial.json \
  --output generated/juniper-portraits-trial
```

Use fresh output directories when rerunning generation. The corpus test checks
all strips, alpha, metadata, every unrelated source color, light face shading in
every expression, fine body shades, 30 protected landmarks, and shifted laughing
poses. It accepts `FOM_JUNIPER_RECIPE` for mutation checks. The catalog-only map
failed on the small autumn chest shade; the unrestricted map failed on the
tiara outline. Dedicated regressions failed on the spring armband rim and winter
bracelet pixel before their corrections, then passed with the final profile.

All 132 applied strips passed exact recipe validation. Every applied frame
matched its authored preview. The rebuilt gallery reused all 264 masks with zero
pending or conflicting groups, and reconstructing its profile reproduced the
same output pixels. All four presets built successfully. The six-example sheet
was inspected again after the final corrections.

Independent verification checked all 132 strips in each of the four target
outputs against 37 reviewed landmarks, original source hashes and dimensions,
alpha and transparent RGB, unrelated colors, metadata, exact target colors, and
identical masks. Its evidence is
`tmp/parallel-art-review-juniper-verification.json`.

Local evidence is recorded in `tmp/juniper-catalog-only-red.log`,
`tmp/juniper-unmasked-red.log`, `tmp/juniper-armband-red.log`,
`tmp/juniper-winter-wrist-red.log`, `tmp/juniper-corpus-green.log`,
`tmp/juniper-validation-report.json`, `tmp/juniper-frame-verification.json`,
`tmp/juniper-authored-gallery-report.json`, and
`tmp/juniper-gallery-roundtrip-report.json`. The archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

Game-derived images, extracted strips, galleries, and authoring helpers remain
ignored. This authoring slice changes data and focused tests; package integration,
full repository verification, and live-game checks belong to the combined
Reina/Juniper integration pass. No unresolved art defect is known from this
review; small pixel-boundary judgments remain subject to the user's visual
checkpoint. The user subsequently accepted the combined trial for committing,
with possible mouth-color issues deferred; see the
[batch record](parallel-portraits.md) for that shared follow-up.
