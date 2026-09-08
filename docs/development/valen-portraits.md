# Valen portrait authoring

This pass covers all **92 main portrait strips / 184 frame occurrences / 176
unique frames** in the supplied archive. Spring, summer, autumn, winter and
beach each contain 17 strips; wedding contains seven. Bathing is included in the
beach directory. Autumn, beach, summer and winter have byte-identical decoded
embarrassed/teary frame pairs; their two asset names remain covered separately.
Overworld, UI and child sprites are outside this profile.

The compact review is `generated/valen-summary.html` and
`generated/valen-summary.png`: six original/Debug Blue pairs spanning all six
outfits, including bathing and an open mouth. All 176 unique frames were
inspected in the 17 sheets at `tmp/valen-{outfit}-{page}.png`, with enlarged
face, hand, body and accessory comparisons. The optional component editor is
`generated/review-valen-authored/index.html`. Game-derived images and local
helpers remain ignored by Git.

## Source colors and boundaries

`palettes/profiles/valen-portraits.json` records 25,345 component seeds, original
PNG hashes and dimensions. Eight source colors include the catalog's four
samples plus four fine skin shades:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#F9CDA3` | Main light skin | `#9DB9D4` |
| `#EDA27A` | Light skin shadow | `#7F9FBD` |
| `#CD825A` | Middle skin shadow | `#6687AD` |
| `#B65932` | Deep skin shadow | `#445F83` |
| `#762E21` | Deep face, hand and body contours | `#445F83` |
| `#DC8662` | Fine eye-adjacent skin shading | `#6687AD` |
| `#EDA37B` | Fine facial transitions | `#7F9FBD` |
| `#E69A72` | Panic-expression eye crease | `#7F9FBD` |

Three connected-color groups separate `#B65932` and `#762E21` from the other
six colors. The source uses those deep shades in both skin contours and small
eye/mouth details. Narrow upper-eye components retain their original dark color,
while the nearby lighter skin creases and deep nose/face/hand contours recolor.
This is a consistent art choice; the source's black lashes and gray eyebrows
also remain intact. Only explicit source-bound seeds are retained; temporary
authoring rules are not part of the runtime.

Mouth interiors, tongue, blush and tears remain original. The separate lower-lip
skin shadow recolors, including the shifted shadow in the speaking frame.
`#CD7753` and `#DF8A67` are mouth-only detail shades and are intentionally outside
the source palette. All hair, clothing, cuffs, sunglasses, bracelets, wedding
flowers and bathing towel retain their original colors. Exposed fingers, crossed
hands, wrists, arms, chest and the beach outfit's exposed leg recolor.

Debug Blue changes **324,881 pixels** and preserves **3,543 other matching-color
pixels** across the source strips.

## Presets and atlases

`palettes/sets/valen-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. Each shared NPC target copies its catalog's four colors through the
roles above, using the same profile. Vanilla is implicit in the installed
control; integration assigns Valen to **O**.

The source metadata maps 17 strips each to `PortraitsSpring`, `PortraitsAutumn`
and `PortraitsWinter`, 34 to `PortraitsSummer` (including beach/bathing), and
seven wedding strips to `PortraitsMisc`.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test valen_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/valen-portraits-study \
  --palette palettes/stylized/valen-portraits.json \
  --output generated/valen-portraits-study
target/release/mistria-palette validate \
  --original extracted/valen-portraits-study \
  --modified generated/valen-portraits-study \
  --palette palettes/stylized/valen-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/valen.json \
  --output generated/review-valen-authored
target/release/mistria-palette build-presets \
  --original extracted/valen-portraits-study \
  --presets palettes/sets/valen-portraits-trial.json \
  --output generated/valen-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all strips, dimensions, alpha, metadata, unrelated colors and every expression's
main face tone. It also checks 20 independently chosen skin landmarks, six eye
landmarks, an open-mouth interior and the speaking frame's lower-lip shadow.
It accepts `FOM_VALEN_RECIPE` for mutation checks. The catalog-only mapping
failed on the omitted deep bathing-hand shade. The unrestricted mapping failed
on the protected upper-eye detail. Dedicated regressions failed on the
initially preserved lower-lip skin shadow and recolored eye detail before their
mask corrections; the final profile passed the same test.

All four targets passed exact recipe validation for all 92 strips. The authored
gallery reused all 184 occurrences with no pending or conflicting unique
frames. Reconstructing a profile from the gallery selections reproduced every
output pixel, and all applied frames matched the inspected author previews.
The six-example summary was inspected after the corrections.

Local evidence is in `tmp/valen-catalog-only-red.log`,
`tmp/valen-unmasked-red.log`, `tmp/valen-lower-lip-red.log`,
`tmp/valen-eye-mask-red.log`, `tmp/valen-corpus-green.log`,
`tmp/valen-*-validation.json`, `tmp/valen-frame-verification.json`,
`tmp/valen-authored-gallery-report.json` and `tmp/valen-final-inputs.sha256`.
The archive used is SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This authoring pass changes declarative recipes and a focused corpus test.
Combined package generation, installation, full repository verification and
live-game checks belong to the Balor/Valen integration pass. No source game
files or generated images are staged by this workflow.

Independent final art verification passed all four targets against 44 additional
source-inspected landmarks, original PNG hashes/dimensions, alpha and transparent
RGB, unchanged metadata, exact target mappings and identical changed masks.
The changed-mask digest is
`7641045c78a178bbac46a31cdcf6424d38ddecd35eaf9be93f0b3f50127c5205`;
evidence is `tmp/balor-valen-art-review-valen-verification.json`. Final enlarged
Ryis-palette face, Blue wedding and Seridia-palette winter crops were inspected.
No further correction was requested by that review; small pixel-boundary
judgments remain subject to the user's compact visual checkpoint.
