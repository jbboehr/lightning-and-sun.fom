# Juniper's Summer regular actions

This slice adds eleven Summer strips: blink South/East and sit, eat and drink
North/South/East. Their 31 source frames plus twelve native West mirrors make
43 review cases. Juniper now has 190 sources and 760 variants. All 179 earlier
region objects, pins, color groups and target mappings remain unchanged,
including the accepted Spring material-edge exceptions. Portrait-only recipes
are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-actions-study`.
All 358 earlier original PNG/metadata files match the retained
`characters-reina-juniper-march-summer-trial` combined bundle. No old source
pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, using
prefix `spr_npc_juniper_summer_`. All retain 80×80 frames, Default atlas and
Middle/54 origin. Blink has three frames at `[0.075, 0.125, 0.075]`; sitting
uses single-frame defaults; drink and eat North have three frames at `1.0`;
eat South/East have five at `[0.125, 0.15, 0.175, 0.125, 0.6]`. The East
packs supply West by mirroring. Raw sidecars are in
`tmp/juniper-summer-actions-author-metadata.json` and agree with the independent
shared inventory.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
eyelid skin, neckline, midriff, bare arms, raised hands, slit-visible legs and
skin between sandal straps change. Circlet corners and separable gold bracer
edges remain original. Hair, cosmetics, mouth interiors, pink gems, blouse,
blue skirt, sandal wraps and outlines are preserved. Existing `E8B171` mapping
covers the drink-arm midtone; its two separable bracer-edge occurrences at
Drink East frames 1/3 `[40,44]` remain original. No new source colors, groups
or target mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Blink South | 62, 66, 62 |
| Blink East | 51, 55, 51 |
| Sit North | 10 |
| Sit South | 53 |
| Sit East | 33 |
| Eat North | 7, 4, 7 |
| Eat South | 56, 45, 46, 53, 53 |
| Eat East | 34, 38, 32, 34, 38 |
| Drink North | 7, 4, 7 |
| Drink South | 53, 49, 53 |
| Drink East | 33, 33, 33 |

The eleven masks use 557 new seeds and recolor 1,162 pixels per target;
120 matching-color material pixels remain excluded. Evidence:
`tmp/juniper-summer-actions-author-refined-components.json`,
`tmp/juniper-summer-actions-author-exclusions.json` and the thirteen
`tmp/juniper-summer-actions-author-refined-art-{action}_{direction}-{page}.png`
sheets. Profile SHA-256:
`180cb1b11dce5b442ed9746400ea9928204e591a3a81803a49bf493cd9138c49`.

## Known art exceptions

Fifteen small bracer-edge pixels share a source-color component with exposed
hands or forearm. The existing mask format selects complete components.
Keeping full skin coverage therefore also changes these edges, following the
accepted earlier tradeoff without changing the schema or existing groups.
This is local to the new strips. All separable gold-border components remain
excluded. Frame numbers below are one-based and coordinates are local to the
80×80 source frame.

| Source | Frame | Bracer-edge pixels |
| --- | --- | --- |
| Sit South | 1 | `[34,45]`, `[45,45]` |
| Eat South | 1 | `[36,45]`, `[45,45]` |
| Eat South | 2 | `[37,46]`, `[45,45]` |
| Eat South | 3 | `[45,45]` |
| Eat South | 4 | `[45,45]` |
| Eat South | 5 | `[34,45]`, `[45,45]` |
| Eat East | 5 | `[39,44]` |
| Drink South | 1, 2, 3 | `[45,45]` in each frame |
| Drink East | 2 | `[36,43]` |

The first fourteen use `E3BF7F` and touch hand/wrist pixels. The last uses
`E8B171` and joins forearm pixel `[36,44]`. Component evidence is recorded in
`tmp/juniper-summer-actions-author-exceptions.json`. The test asserts every
exception and the protected separable borders literally. The gallery includes
a visible review note and every affected frame.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-actions-preview/summary.png)
shows eat South frame 2, eat East frame 5 and drink East frame 2, including
examples of the known bracer-edge tradeoff. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-actions-preview/blue-review/index.html)
covers all 43 cases across eighteen detail pages, at most four cases per page,
with enlarged face details and exact native West mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including moving hair,
hands and sandals. Exact checks verified 8,558,880 preview pixels, 86 full-frame
bindings, fifteen summary bindings, source frame counts, raw metadata and
mirrored coordinates. Chromium checked all twenty pages, including both
indexes: every image and local link loaded, with no horizontal overflow.
Evidence: `tmp/juniper-summer-actions-author-preview-check.log` and
`tmp/juniper-summer-actions-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy
passed before profile promotion; the final focused test in
`tests/juniper_summer_actions.rs` then passed. It checks every new pixel in four
targets, per-frame counts, metadata, 63 literal skin/material landmarks and
all 1,432 earlier variant PNG/metadata files. Practical controls remove a
finger-shadow component and merge the color groups; the test detects missed
skin and spill into a protected bracer. Evidence:
`tmp/juniper-summer-actions-author-candidate-test.log`,
`tmp/juniper-summer-actions-author-clippy.log` and
`tmp/juniper-summer-actions-author-final-test.log`.

All 760 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,790 earlier original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-actions-author-preservation.log` and
`tmp/juniper-summer-actions-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static source previews do not verify attached food/drink
props, live animation timing, natural schedules, outfit transitions or gameplay.
The user approved this artwork for commit on 2026-09-26. Game-derived images remain ignored.
