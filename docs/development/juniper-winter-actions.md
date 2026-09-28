# Juniper's Winter blink, sit, eat and drink sprites

This slice adds eleven Winter strips: blink South/East and sit, eat and drink
North/South/East. Their 31 source frames and twelve native West mirrors make
43 review cases. Juniper now has 256 sources and 1,024 variants. All 245 earlier
region objects and source pins, seventeen source colors, nine color groups,
target mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-actions-study`. Its 490 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-winter-trial` bundle. Pins remain strict.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_winter_`. All retain 80×80 frames, Default atlas and origin
`Middle,54.0`. Blink has three frames at `[0.075,0.125,0.075]`; sit uses
single-frame defaults. Eat North has three frames at `1.0`; eat South/East have
five at `[0.125,0.15,0.175,0.125,0.6]`. Drink has three frames per direction at
`1.0`. Raw sidecars at `tmp/juniper-winter-actions-author-metadata.json` match
the root's independent archive reads. The shared slice checks native direction,
mirroring and the eat/drink final-frame holds separately.

Every actual frame was inspected in Vanilla and all four targets. Exposed face,
eyelid skin, ears and their shadows change. The Winter gloves cover the hands,
and the raised cuffs retain bright jewelry highlights even where they use skin
colors. North sit/eat/drink have explicit empty masks and remain byte-identical
in every preset. Warm bodice and leg details, gloves, shoulder decorations,
cuffs, cyan gems, gold circlet, hair, cosmetics and mouth colors stay original.
No new source roles or coupled-material exceptions are needed. All previous
accepted exceptions remain unchanged.

The new poses require different material boundaries from earlier outfits:

- Eat South frame 2 preserves cuff highlights `[37,44]` (`EFD89A`) and `[36,45]`
  (`E3BF7F`), while its moving cheek `[37,40]` (`763F21`) changes.
- Eat East frames 1–5 preserve the bright cuff while it rotates below the glove.
  Examples are `[40,42]` in frame 1, `[41,41]` in frame 2 and `[41,42]` in frame 3,
  all `EFD89A`. The face above changes independently.
- Drink East frame 2 preserves cuff pixels `[36,43]` (`763F21`), `[36,44]`
  (`BC8B43`) and `[37,44]` (`E8B171`). The same dark shade at jaw `[36,39]`
  changes. The existing alias therefore needs no new target mapping.
- Eat South/East frame 3 preserves the large red mouth interior and dark rim.
  The visible face around the mouth changes; no separate food or cup asset is
  added by this body-animation slice.

Frame numbers here are one-based and coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Blink South | 22, 26, 22 |
| Blink East | 19, 23, 19 |
| Sit North | 0 |
| Sit South | 22 |
| Sit East | 19 |
| Eat North | 0, 0, 0 |
| Eat South | 22, 20, 13, 26, 22 |
| Eat East | 19, 15, 12, 19, 19 |
| Drink North | 0, 0, 0 |
| Drink South | 22, 25, 22 |
| Drink East | 19, 19, 19 |

The masks add 197 seeds and recolor 485 pixels per target. Another 206
matching-color jewelry pixels remain protected. Evidence:
`tmp/juniper-winter-actions-author-refined-components.json`,
`tmp/juniper-winter-actions-author-exclusions.json`,
`tmp/juniper-winter-actions-author-inventory.log` and the thirteen
`tmp/juniper-winter-actions-author-refined-art-*.png` sheets. Profile SHA-256:
`74f144afe21d6d03cb848422b9735f7f60a16708defa5020af3e11584e39859c`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-actions-preview/summary.png)
shows blink South frame 2, eat South frame 3 and drink East frame 2. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-actions-preview/blue-review/index.html)
shows all 31 source frames and twelve West mirrors across eighteen detail
pages, with enlarged face views. A visible note explains the unchanged North
views and the bright cuff highlights.

Crop `[25,20,32,39]` includes every visible pixel in native and mirrored views.
Exact checks verified 8,558,880 preview pixels, 86 full bindings, fifteen
summary bindings, frame counts and raw metadata. Chromium checked all twenty
pages, including both indexes: local links and images loaded without horizontal
overflow. Evidence: `tmp/juniper-winter-actions-author-preview-checks.log` and
`tmp/juniper-winter-actions-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_winter_actions.rs` (24.80 seconds). It checks
every new pixel in four targets, per-frame counts, unchanged metadata, 67
literal material landmarks, explicit empty North masks and all 1,960 prior
variant PNG/metadata files. Three effective controls omit a cheek-shadow
component, omit an ear component and remove region restrictions; they expose
missed skin and accidental circlet, shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-actions-author-candidate-test.log` and
`tmp/juniper-winter-actions-author-test.log`.

All 1,024 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,450 prior original/variant PNG and metadata files are
byte-identical; all 2,048 final variant files match the inspected candidates.
Evidence: `tmp/juniper-winter-actions-author-preservation.log`,
`tmp/juniper-winter-actions-author-stylized-validation.json` and
`tmp/juniper-winter-actions-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
