# Juniper's Summer action, sleep and kiss sprites

This slice adds five Summer strips: general action North/South/East, sleep
East and kiss East. Their 26 source frames plus twelve native West mirrors
make 38 review cases. Juniper now has 195 sources and 780 variants. All 190
earlier region objects, pins, color groups and target mappings remain unchanged,
including the accepted Spring and Summer material-edge exceptions. Portrait-only
recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-standard-study`.
All 380 earlier original PNG/metadata files match the retained
`characters-reina-juniper-march-summer-actions-trial` combined bundle. No old
source pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, using
prefix `spr_npc_juniper_summer_`. All retain 80×80 frames, Default atlas and
Middle/54 origin. General action has seven frames per direction at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss has four at
`[0.15, 0.15, 0.8, 0.15]`; sleep uses single-frame defaults. Raw sidecars in
`tmp/juniper-summer-standard-author-metadata.json` agree with the independent
shared inventory. Action uses a directional collection with a horizontal East
pack. Sleep and kiss use single linear East packs; the native NPC renderer
still flips them when cardinality is West. The review includes those mirrors
without claiming that every direction occurs naturally in gameplay.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
closed eyelid skin, neckline, midriff, bare arms, hands, slit-visible legs and
skin between sandal straps change. Circlet corners and separable gold bracer
borders remain original. Hair, cosmetics, pink gems, blouse, blue skirt,
sandal wraps and outlines are preserved. No source colors, groups or target
mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Action North | 14, 14, 14, 14, 14, 16, 19 |
| Action South | 58, 53, 52, 53, 52, 58, 62 |
| Action East | 43, 54, 45, 54, 45, 43, 51 |
| Sleep East | 51 |
| Kiss East | 45, 45, 51, 49 |

The five masks use 539 new seeds and recolor 1,069 pixels per target;
108 matching-color material pixels remain excluded. Evidence:
`tmp/juniper-summer-standard-author-refined-components.json`,
`tmp/juniper-summer-standard-author-exclusions.json` and the eight
`tmp/juniper-summer-standard-author-refined-art-{action}_{direction}-{page}.png`
sheets. Profile SHA-256:
`080588a29a7e95c4105778142579c624e5768dd4e2d976afa854424c2698f0ad`.

## Known art exceptions

Fourteen small bracer-border pixels share a source-color component with exposed
hands or arms. The existing mask format selects complete components. Keeping
full skin coverage also changes these borders, following the accepted earlier
tradeoff without changing the schema or existing groups. This is local to the
new strips. All separable gold-border components remain excluded. Frame numbers
below are one-based and coordinates are local to the 80×80 source frame.

| Source | Frames | Bracer-border pixels | Source color |
| --- | --- | --- | --- |
| Action South | 1, 6 | `[44,46]`, `[45,46]`, `[46,46]` in each frame | `BC8B43` |
| Action South | 3, 5 | `[38,45]` in each frame | `E3BF7F` |
| Action East | 1, 6 | `[40,46]` in each frame | `E3BF7F` |
| Action East | 3, 5 | `[44,45]` in each frame | `E3BF7F` |
| Sleep East | 1 | `[40,41]` | `BC8B43` |
| Sleep East | 1 | `[41,42]` | `E3BF7F` |

The South three-pixel borders join hand pixel `[45,47]`. Other action borders
join adjoining hand highlights. The sleep borders join upper-arm `[39,41]`
and forearm `[40,42]` respectively. Component evidence is recorded in
`tmp/juniper-summer-standard-author-exceptions.json`. Literal test assertions
cover every exception and protected separable borders, including Action East
frames 3/5 `[42,44]` and `[43,45]`, and Kiss East frame 3 `[38,45]`.
The gallery has a visible review note and includes every affected frame.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-standard-preview/summary.png)
shows Action South frame 3, Sleep East and Kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-standard-preview/blue-review/index.html)
covers all 38 cases across twelve detail pages, at most four cases per page,
with enlarged face details and exact native West mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including moving hair,
hands and sandals. Exact checks verified 7,618,080 preview pixels, 76 full-frame
bindings, fifteen summary bindings, source frame counts, raw metadata and
mirrored coordinates. Chromium checked all fourteen pages, including both
indexes: every image and local link loaded, with no horizontal overflow.
Evidence: `tmp/juniper-summer-standard-author-preview-check.log` and
`tmp/juniper-summer-standard-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy passed
before profile promotion; the final focused test in
`tests/juniper_summer_standard.rs` then passed. It checks every new pixel in four
targets, per-frame counts, metadata, 58 literal skin/material landmarks and
all 1,520 earlier variant PNG/metadata files. Practical controls remove a
finger-shadow component and merge color groups; the test detects missed skin
and spill into a protected bracer. Evidence:
`tmp/juniper-summer-standard-author-candidate-test.log`,
`tmp/juniper-summer-standard-author-clippy.log` and
`tmp/juniper-summer-standard-author-final-test.log`.

All 780 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,900 earlier original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-standard-author-preservation.log` and
`tmp/juniper-summer-standard-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static source previews do not verify attached props, live
animation timing, natural schedules, outfit transitions or gameplay.
Game-derived images remain ignored.
