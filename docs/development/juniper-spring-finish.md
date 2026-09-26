# Juniper's remaining Spring specials

This slice finishes Juniper's Spring folder with seven strips: pose South,
hair flip South, spell cast start/loop/end South, gremlin East and snooze South.
Their 29 source frames plus six native West gremlin mirrors make 35 review
cases. Juniper now has 173 sources and 692 variants. All 166 earlier region
objects, pins, color groups and mapping roles remain unchanged. Portrait-only
recipes and the six accepted eating/laugh edge exceptions are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-spring-finish-study`.
All 332 earlier original PNG/metadata files match the retained Spring-specials
combined bundle. No old source pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Spring/`, using
prefix `spr_npc_juniper_specialanimation_spring_`. All retain 80×80 frames,
Default atlas and numeric origin `40.0 / 54.0`. Pose and spell cast end use
single-frame defaults. Hair flip has six frames with durations
`[0.15, 0.15, 0.125, 0.125, 0.5, 0.125]`; spell cast start/loop have two/four
frames at `0.125`; gremlin has six at `[0.125, 1.6, 0.125, 1.6, 0.125, 1.6]`;
snooze has nine at `[0.3, 0.8, 0.3, 3.0, 0.15, 1.5, 0.15, 0.125, 1.5]`.
Raw sidecars are in `tmp/juniper-spring-finish-author-metadata.json`.
The native East gremlin pack supplies West by mirroring. The other strips
only face South.

All 29 actual frames were inspected in Vanilla and all four targets. Face,
ears, forehead, neck, chest, midriff, hands, arms and exposed legs change.
This includes the dark `763F21` forehead exposed above and beside the circlet
in spell cast loop frames 1–2. Existing mapping roles already cover it.
Matching shades on circlet corners, bracers, skirt clasps and hem points stay
original, subject to the seven small exceptions below. Hair, cosmetics,
expression details, fabric, gems, boots and outlines remain original.
No new source colors, groups or mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Pose South | 49 |
| Hair flip South | 43, 58, 59, 53, 53, 53 |
| Spell cast start South | 52, 60 |
| Spell cast loop South | 76, 72, 68, 68 |
| Spell cast end South | 52 |
| Gremlin East | 31, 27, 31, 28, 31, 30 |
| Snooze South | 62, 58, 62, 58, 62, 45, 45, 41, 55 |

The masks use 683 new component seeds and recolor 1,482 pixels per target;
222 matching-color material pixels remain excluded. Evidence:
`tmp/juniper-spring-finish-author-refined-components.json`,
`tmp/juniper-spring-finish-author-exclusions.json` and the eleven
`tmp/juniper-spring-finish-author-refined-art-{strip}-{page}.png` sheets.
Profile SHA-256:
`85ce8aaa321266c24ed046476482860db3e58441277e8dd9c15e73a89f72000d`.

## Known art exceptions

Seven `E3BF7F` pixels on hair-flip bracer edges share a source-color component
with exposed wrist or upper-arm pixels. The existing mask format selects
entire components. Preserving full skin coverage therefore also recolors
these small edges, following the accepted eating/laugh tradeoff without a
schema change. Frame numbers below are one-based; coordinates are local to
the 80×80 frame.

| Hair flip South frame | Bracer-edge pixels |
| --- | --- |
| 1 | `[43,42]`, `[34,45]` |
| 2 | `[44,40]`, `[35,43]` |
| 3 | `[33,43]` |
| 6 | `[32,44]`, `[46,43]` |

The focused test names all seven exceptions explicitly. The summary includes
hair flip frame 3, and the complete gallery shows every affected frame.

## Offline review and verification

The [five-choice summary](../../generated/juniper-spring-finish-preview/summary.png)
shows hair flip frame 3, spell cast loop frame 1 and gremlin East frame 6.
The [complete Vanilla/Debug Blue gallery](../../generated/juniper-spring-finish-preview/blue-review/index.html)
covers all 35 cases across thirteen detail pages, at most four cases per page,
with enlarged face details. The six West cases use exact native frame mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including moving hair,
outstretched hands and boots. Exact checks verified 7,053,600 preview pixels,
70 full-frame bindings, fifteen summary bindings, source frame counts,
raw metadata and mirrored coordinates. Chromium checked all fifteen pages,
including both indexes: every image and local link loaded, with no horizontal
overflow. Evidence: `tmp/juniper-spring-finish-author-preview-check.log` and
`tmp/juniper-spring-finish-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy
passed before profile promotion; the final focused test in
`tests/juniper_spring_finish.rs` then passed. It checks every new pixel in
four targets, per-frame counts, metadata, 52 literal skin/material landmarks
and all 1,328 prior variant PNG/metadata files. Practical controls remove an
upper-arm shadow component and merge the color groups; the test detects
missed skin and spill into a protected bracer. Evidence:
`tmp/juniper-spring-finish-author-candidate-test.log`,
`tmp/juniper-spring-finish-author-clippy.log` and
`tmp/juniper-spring-finish-author-final-test.log`.

All 692 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,660 previous original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-spring-finish-author-preservation.log` and
`tmp/juniper-spring-finish-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live animation timing,
attached effects, natural schedules or gameplay. The user approved this artwork
for commit on 2026-09-26. Game-derived images remain ignored.
