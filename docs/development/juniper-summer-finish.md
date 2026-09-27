# Juniper's remaining Summer sprites

This slice completes Juniper's Summer folder with spell casting (start, loop,
end), hair flip and snooze. The five strips contain 22 South-facing source
frames and review cases. Juniper now has 206 sources and 824 variants. All 201
earlier region objects, pins, groups and target mappings remain unchanged,
including the approved Spring and Summer bracer exceptions. Portrait-only
recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-finish-study`.
All 402 earlier original PNG/metadata files match the retained
`characters-reina-juniper-march-summer-specials-trial` bundle. No old source
pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, using
prefix `spr_npc_juniper_specialanimation_summer_`. All retain 80×80 frames,
Default atlas and numeric origin `40.0 / 54.0`. The numeric horizontal field
is preserved literally. Spell start has two frames at `0.125`, loop has four
at `0.125`, and end uses single-frame defaults. Hair flip uses six durations
`[0.15, 0.15, 0.125, 0.125, 0.5, 0.125]`; snooze uses nine durations
`[0.3, 0.8, 0.3, 3.0, 0.15, 1.5, 0.15, 0.125, 1.5]`. Raw sidecars are in
`tmp/juniper-summer-finish-author-metadata.json`. The native inventory configures
hair flip and snooze as linear animations and spell casting as a complex cycle;
the shared runtime checks cover these bindings and the detached sound setting.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
exposed forehead and temple shadows, neck, chest, midriff, fingers, arms and skin
between sandal straps change. Circlet corners and separable cuff edges remain
original. Hair, cosmetics, gems, blouse, blue skirt, sandal wraps and outlines
remain unchanged. Raised hands and rotating cuffs were checked against the
actual Summer geometry. Warm wrist and arm outlines adjoining cuffs remain
skin; cuff corners sharing their skin component use the exception below.
Existing source colors cover the exposed forehead as the hair moves; no source
colors, groups or target mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Spell cast start South | 65, 70 |
| Spell cast loop South | 82, 81, 78, 76 |
| Spell cast end South | 65 |
| Hair flip South | 55, 68, 70, 65, 65, 66 |
| Snooze South | 69, 65, 69, 65, 69, 52, 53, 48, 65 |

The masks use 694 new seeds and recolor 1,461 pixels per target; 127
matching-color jewelry pixels remain excluded. Evidence:
`tmp/juniper-summer-finish-author-refined-components.json`,
`tmp/juniper-summer-finish-author-exclusions.json` and the eight
`tmp/juniper-summer-finish-author-refined-art-*.png` sheets. Profile SHA-256:
`244851c38c03a84581267fcfdc08bde9470bf969624c5f8e53f3e862481e9d6a`.

## Known art exceptions

Thirteen cuff-edge occurrences share components with exposed hands or forearms.
They retain the established full-skin-coverage compromise without changing the
schema or color groups. Frame numbers below are one-based; coordinates are
zero-based and local to the frame.

| Strip / frames | Recolored cuff-edge coordinates | Source shade |
| --- | --- | --- |
| Spell loop 1–4 | `[33,43]`, `[46,43]` | `E3BF7F` |
| Hair flip 3 | `[45,41]` | `E3BF7F` |
| Hair flip 4–5 | `[31,45]` | `E3BF7F` |
| Hair flip 6 | `[33,45]` | `EFD89A` |
| Snooze 9 | `[44,44]` | `E3BF7F` |

The spell-loop cuff corners `[35,43]` and `[44,43]` stay original, as do
hair-flip frame 6 cuff borders `[43,44]` and `[45,44]` and snooze frame 9 border
`[46,44]`. Literal test assertions cover the exceptions, adjacent skin and
preserved borders. Component evidence is in
`tmp/juniper-summer-finish-author-exceptions.json`. The summary and complete
gallery show affected frames, with a visible note listing these edges.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-finish-preview/summary.png)
shows spell loop frame 1, hair flip frame 3 and snooze frame 9. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-finish-preview/blue-review/index.html)
shows every source frame across eight detail pages, with enlarged face details
covering the configured South-facing directions.

Crop `[25,20,32,39]` includes every visible pixel, including outstretched fingers,
flying hair and sandals. The exact checker verified 4,607,520 preview pixels,
44 full-frame bindings, fifteen summary bindings, frame counts and raw metadata.
Chromium checked all ten pages including both indexes: every local link and
image loaded, with no horizontal overflow. Evidence:
`tmp/juniper-summer-finish-author-preview-check.log` and
`tmp/juniper-summer-finish-author-preview-browser-check.json`.

The independent candidate test and targeted Clippy passed before profile
promotion; the final focused test in `tests/juniper_summer_finish.rs` passed
afterward. It checks every new pixel in four targets, per-frame counts,
metadata, 54 literal skin/material landmarks and all 1,608 prior variant
PNG/metadata files. Practical controls drop a finger-shadow component and
merge groups; the test detects missed skin and spill into a protected far-cuff
corner. Evidence:
`tmp/juniper-summer-finish-author-candidate-test.log`,
`tmp/juniper-summer-finish-author-clippy.log` and
`tmp/juniper-summer-finish-author-final-test.log`.

All 824 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 2,010 prior original and variant
PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-finish-author-preservation.log` and
`tmp/juniper-summer-finish-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live timing, natural schedules,
outfit transitions or gameplay. Game-derived images remain ignored.
