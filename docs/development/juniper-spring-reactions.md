# Juniper's Spring shocked and seated reading sprites

This slice adds six Spring strips: shocked start/loop/end South and seated
reading start/loop/end South. Their thirteen source frames are thirteen review
cases; these cycles have no West variants. Juniper now has 160 sources and
640 variants. All 154 previous region objects, pins, color groups and mapping
roles remain unchanged, including the three accepted eating-edge exceptions.
Portrait-only definitions remain untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-spring-reactions-study`.
All 308 earlier original PNG/metadata files match the retained Spring-standard
combined bundle; no old pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Spring/`.
Shocked names use `spr_npc_juniper_spring_shocked_`; reading names use
`spr_npc_juniper_specialanimation_spring_read_sit_`. All retain 80×80 frames,
Default atlas and Middle/54 origin. Shocked strips have single-frame defaults.
Reading start/end each have three frames at `0.1`; its four-frame loop uses
`[3.0, 0.1, 3.0, 0.1]`. Direct raw sidecars are recorded in
`tmp/juniper-spring-reactions-author-metadata.json`.

All thirteen actual frames were inspected in Vanilla and all four targets.
Face, ears, forehead above closed eyes, neck, chest, midriff, raised arms,
hands and exposed legs change. Circlet corners, gold bracer edges and skirt
trim reuse skin shades and are excluded by their reviewed components. The
book's warm page colors `F6E4D7` and `C9AF9C`, pink cover, enlarged eye whites,
cosmetic eyelids, mouth interior, tongue, hair, fabric, gems, boots and outlines
stay original. No source shades or mapping roles were added, and no new
inseparable skin/material boundary exceptions were identified.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Shocked start South | 52 |
| Shocked loop South | 66 |
| Shocked end South | 52 |
| Reading start South | 30, 24, 28 |
| Reading loop South | 20, 28, 20, 28 |
| Reading end South | 32, 20, 30 |

The masks use 207 component seeds and recolor 430 pixels per target;
85 matching-color jewelry/hem pixels remain excluded. Evidence:
`tmp/juniper-spring-reactions-author-refined-components.json`,
`tmp/juniper-spring-reactions-author-exclusions.json` and the six
`tmp/juniper-spring-reactions-author-refined-art-{cycle}_south.png` sheets.
Profile SHA-256:
`79f34c7e1b1e73c5a5b98539b9b188ac6670633883944d20d17d4fdc9b61cbd0`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-spring-reactions-preview/summary.png)
shows shocked loop, reading start frame 2 and reading loop frame 1. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-spring-reactions-preview/blue-review/index.html)
covers all thirteen cases across six detail pages, at most four cases per
page, with enlarged face details. It contains no invented West mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including raised hands,
spread hair, book edges and boots. The exact checker verified 2,914,080
preview pixels, 26 full-frame bindings, fifteen summary bindings, source
frame counts and raw metadata. Chromium checked all eight pages, including
both indexes: every local link and image loaded, with no horizontal overflow.
Evidence: `tmp/juniper-spring-reactions-author-preview-check.log` and
`tmp/juniper-spring-reactions-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy
passed before profile promotion, and the final focused test in
`tests/juniper_spring_reactions.rs` passed afterward. It checks all new pixels
in four targets, counts, metadata, 37 literal skin/material landmarks and all
1,232 earlier variant PNG/metadata files. Practical controls remove an
upper-arm component or merge the color groups; the test detects missed skin
and spill into a protected bracer. Evidence:
`tmp/juniper-spring-reactions-author-candidate-test.log`,
`tmp/juniper-spring-reactions-author-clippy.log` and
`tmp/juniper-spring-reactions-author-final-test.log`.

All 640 final variants passed exact-palette validation, with a separate check
using the stylized Debug Blue recipe. All 1,540 earlier original and variant
PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-spring-reactions-author-preservation.log` and
`tmp/juniper-spring-reactions-author-stylized-validation.json`.

The [shared slice](reina-juniper-march-reactions.md) handles broader checks,
combined packaging, native probes and installation. Static previews do not
verify live animation timing, natural schedules or gameplay. User artwork
review was approved on 2026-09-25. Game-derived images remain ignored.
