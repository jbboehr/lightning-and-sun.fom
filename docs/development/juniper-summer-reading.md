# Juniper's Summer seated reading sprites

This slice adds the Summer seated reading start, loop and end strips. Their
ten South-facing source frames are the ten review cases. Juniper now has 198
sources and 792 variants. All 195 earlier region
objects, pins, groups and target mappings remain unchanged, including the
approved Spring and Summer bracer exceptions. Portrait-only recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-reading-study`.
All 390 earlier original PNG/metadata files match the retained
`characters-reina-juniper-march-summer-standard-trial` bundle. No old source
pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, using
prefix `spr_npc_juniper_specialanimation_summer_read_sit_`. All retain 80×80
frames, Default atlas and Middle/54 origin. Start and end each have three
frames at `0.1`; the four-frame loop uses `[3.0, 0.1, 3.0, 0.1]`. Raw sidecars
are in `tmp/juniper-summer-reading-author-metadata.json` and match the
independent shared inventory.

All ten actual frames were inspected in Vanilla and all four targets. Face,
ears, eyelid skin, neck, chest above the book, shoulders, hands, midriff,
slit-visible shin and skin between sandal straps change. Circlet corners and
separable bracer borders stay original. Hair, cosmetic eyelids, blouse, blue
skirt, sandal wraps, gems and outlines remain unchanged. The book's warm page
colors `C9AF9C` and `F6E4D7`, pink cover and binding stay original. No source
colors, groups or target mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Reading start South | 50, 24, 34 |
| Reading loop South | 24, 28, 24, 28 |
| Reading end South | 38, 20, 50 |

The masks use 150 new seeds and recolor 320 pixels per target; 44 matching-color
material pixels remain excluded. Evidence:
`tmp/juniper-summer-reading-author-refined-components.json`,
`tmp/juniper-summer-reading-author-exclusions.json` and the three
`tmp/juniper-summer-reading-author-refined-art-read_sit_{phase}_south-0.png`
sheets. Profile SHA-256:
`b113cf051f40ec15109f6cff4585e84c561cdcb38d3ed42f3a29dca294094a5e`.

## Known art exceptions

Reading start frame 1 and reading end frame 3 each have one `E3BF7F` bracer
corner at local `[34,45]`. Each joins hand pixels `[34,46]` and `[33,46]` in
one source-color component. Keeping the whole hand recolored therefore changes
that corner too, following the established compromise without a schema or
group change. These frame numbers are one-based. Both instances have literal
test assertions, component evidence in
`tmp/juniper-summer-reading-author-exceptions.json` and a visible gallery note.
The separable bracer edges beside the closed book remain protected.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-reading-preview/summary.png)
shows the first frame of start, loop and end. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-reading-preview/blue-review/index.html)
shows every source frame across three detail pages with enlarged face details.

Crop `[25,20,32,39]` includes every visible pixel, including the spread book,
hair, fingers and sandals. The exact checker verified 2,349,600 preview pixels,
twenty full-frame bindings, fifteen summary bindings, frame counts and raw
metadata. Chromium checked all five pages including both indexes: every local
link and image loaded, with no horizontal overflow. Evidence:
`tmp/juniper-summer-reading-author-preview-check.log` and
`tmp/juniper-summer-reading-author-preview-browser-check.json`.

The independent candidate test and targeted Clippy passed before profile
promotion; the final focused test in `tests/juniper_summer_reading.rs` passed
afterward. It checks every new pixel in four targets, per-frame counts,
metadata, 32 literal skin/material landmarks and all 1,560 prior variant
PNG/metadata files. Practical controls drop a finger-shadow component and
merge groups; the test detects missed skin and spill into a protected bracer
beside the closed book. Evidence:
`tmp/juniper-summer-reading-author-candidate-test.log`,
`tmp/juniper-summer-reading-author-clippy.log` and
`tmp/juniper-summer-reading-author-final-test.log`.

All 792 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,950 prior original and variant
PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-reading-author-preservation.log` and
`tmp/juniper-summer-reading-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live timing, natural schedules,
outfit transitions or gameplay. Game-derived images remain ignored.
