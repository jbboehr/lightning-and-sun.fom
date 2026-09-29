# Juniper Wedding completion

This slice adds the nine remaining Wedding strips: blink East/South, sit and
general action North/South/East, and kiss East. The current Wedding folder is
now covered completely, 15/15 strips. The additions contain 34 source frames
and fifteen native West mirrors, giving 49 review cases. Juniper now has 305
sources and 1,220 variants.

All 296 earlier region objects, source pins, seventeen source colors, nine
groups and target mappings remain unchanged. This includes all earlier material
decisions, particularly the six Wedding pilot anklet-edge occurrences and
nineteen Beach bangle-edge occurrences.

## Sources and materials

The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The fresh corpus is `extracted/juniper-wedding-finish-study`. All 592 earlier
original PNG/raw-metadata files match the retained
`characters-reina-juniper-march-wedding-pilot-trial` bundle, and all 296 PNG
pins still match. No old hashes or masks were refreshed.

Paths use `assets/animations/NPCs/Juniper/Sprites/Wedding/` and prefix
`spr_npc_juniper_wedding_`. Raw sidecars at
`tmp/juniper-wedding-finish-author-metadata.json` match independent archive
reads. All frames are 80×80 in Default atlas at origin `Middle,54.0`.
Blink has three frames per direction, sitting has one, general actions have
seven and kiss has four. Metadata is copied byte-for-byte. The shared slice
checks native packs, timing, action holds and East-to-West mirroring. A fresh
archive inventory exactly matches the profile's fifteen Wedding paths; see
`tmp/juniper-wedding-finish-author-folder-check.log`.

Every actual frame was inspected in Vanilla and all four target palettes.
The masks cover moving faces, exposed skin around closed eyes, the dress
keyhole, arms, hands, legs and toes. Sitting North is unchanged because hair
covers all exposed skin. North actions cover the shaded feet beneath the hem
and the returning hands. Pink eyelid cosmetics, eye details, hair, fabric,
bright gold and every separable gold border remain original. No new palette
roles were needed, and all source alpha is preserved.

Fifty new `BC8B43` anklet/cuff-edge occurrences share connected components with
foot or hand skin. The existing mask format cannot split those components.
Following the approved full-skin compromise, they recolor with skin while
separable borders stay original. This limitation is stated on the summary
image and every gallery page, and all fifty occurrences have literal tests.
The exact new exceptions are below; frames are one-based and coordinates are
frame-local.

| Strip and frames | Connected edge coordinates | Occurrences |
| --- | --- | --- |
| Blink East 1–3 | `[38,52]`, `[39,52]` | 6 |
| Sit East 1 | `[41,48]`, `[42,48]` | 2 |
| Action South 2–5 | `[44,45]` | 4 |
| Action East 1, 6 | `[38,52]`, `[39,52]`, `[42,52]`, `[43,52]` | 8 |
| Action East 2, 4 | `[37,51]`, `[38,51]`, `[42,52]`, `[43,52]` | 8 |
| Action East 3, 5 | `[42,52]`, `[43,52]` | 4 |
| Action East 7 | `[38,52]`, `[39,52]` | 2 |
| Kiss East 1, 2, 4 | `[37,52]`, `[38,52]`, `[41,52]`, `[42,52]` | 12 |
| Kiss East 3 | `[36,51]`, `[37,51]`, `[41,52]`, `[42,52]` | 4 |

For example, action South frame 2 keeps the distinct gold shadow `[44,44]`
original while covering the connected hand `[44,46]` and cuff edge `[44,45]`.
Action East frame 3 preserves separable anklet edges `[38,51]` and `[39,52]`
while recoloring the bare heel `[37,52]`. Kiss frame 3 preserves pink eyelids
at `[41,35]`, covers the below-eye skin `[41,37]` and projecting cheek contour
`[45,38]`, and preserves the separable gold-band edge `[39,44]`.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Blink South | 59, 63, 59 |
| Blink East | 57, 61, 57 |
| Sit North | 0 |
| Sit South | 54 |
| Sit East | 50 |
| Action North | 0, 4, 4, 4, 4, 0, 2 |
| Action South | 57, 60, 60, 60, 60, 57, 59 |
| Action East | 57, 61, 54, 61, 54, 57, 57 |
| Kiss East | 52, 56, 63, 60 |

The masks add 666 seeds, recolor 1,523 pixels per target and protect 182
material pixels sharing skin shades. Evidence:
`tmp/juniper-wedding-finish-author-refined-components.json` and the refined
four-palette art sheets. Profile SHA-256:
`8a59af659f54dfb24e5ebf63dd57ff07243786938d882d51e6213143f0993e91`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-wedding-finish-preview/summary.png)
shows action South frame 2, action East frame 2 and kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-wedding-finish-preview/blue-review/index.html)
includes all 49 cases across seventeen detail pages. The full crop
`[25,20,32,39]` contains every visible source pixel. Enlarged `[28,30,24,26]`
views reach the feet so the connected ankle edges can be reviewed.

Exact checks passed for 8,784,672 preview pixels, 98 full bindings, fifteen
summary bindings, source metadata, frame counts and native mirrors. Chromium
checked both indexes and all seventeen detail pages: all local links and
images loaded without horizontal overflow. The preview directory is 927,771
bytes; the largest PNG is 61,079 bytes. Evidence:
`tmp/juniper-wedding-finish-author-preview-checks.log` and
`tmp/juniper-wedding-finish-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_wedding_finish.rs` (24.59 seconds). It checks
every new RGBA pixel in all four targets, per-frame coverage, unchanged
metadata, 74 literal landmarks, all 182 protected same-shade pixels, the fifty
accepted exceptions and all 2,368 prior variant PNG/metadata files. Four
omission controls independently remove forehead, keyhole, hand and toe seeds.
An unrestricted-mapping control changes protected hairpin, arm-band and
anklet pixels, demonstrating effective spill detection. These mutations stay
in test-local recipes. Evidence:
`tmp/juniper-wedding-finish-author-candidate-test.log` and
`tmp/juniper-wedding-finish-author-final-checks.log`.

All 1,220 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,960 prior original/variant PNG and metadata files are
byte-identical; all 2,440 final variant files match the inspected candidates.
Evidence: `tmp/juniper-wedding-finish-author-preservation.log`,
`tmp/juniper-wedding-finish-author-stylized-validation.json` and
`tmp/juniper-wedding-finish-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes
and isolated installation. Live gameplay and transitions were not exercised
here. All game-derived art remains ignored; nothing was staged or committed
by this author.
