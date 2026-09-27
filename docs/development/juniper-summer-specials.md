# Juniper's Summer laugh sprites

This slice adds the Summer laugh start, loop and end strips. Their five
South-facing source frames are five review cases. Juniper now has 201 sources and 804 variants. All 198 earlier region
objects, pins, groups and target mappings remain unchanged, including the
approved Spring and Summer bracer exceptions. Portrait-only recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-specials-study`.
All 396 earlier original PNG/metadata files match the retained
`characters-reina-juniper-march-summer-reading-trial` bundle. No old source
pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, using
prefix `spr_npc_juniper_specialanimation_summer_laugh_`. All retain 80×80
frames, Default atlas and numeric origin `40.0 / 54.0`. The numeric horizontal
field is preserved literally. Start uses single-frame defaults; the two-frame
loop uses `[0.15, 0.125]`; end has two frames at `0.125`. Raw sidecars are in
`tmp/juniper-summer-specials-author-metadata.json`.

All five actual frames were inspected in Vanilla and all four targets. Face,
ears, eyelid skin, neck, chest, midriff, hands, arms and skin between sandal
straps change. Circlet corners and separable gold cuff edges stay original.
Hair, cosmetics, gems, blouse, blue skirt, sandal wraps and outlines remain
unchanged. The Summer loop and first end frame show bare arms in places where
the Spring art had bracers; those actual Summer skin pixels are included.
No source colors, groups or target mappings were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Laugh start South | 66 |
| Laugh loop South | 70, 67 |
| Laugh end South | 70, 66 |

The masks use 155 new seeds and recolor 339 pixels per target; 28 matching-color
material pixels remain excluded. Evidence:
`tmp/juniper-summer-specials-author-refined-components.json`,
`tmp/juniper-summer-specials-author-exclusions.json` and the three
`tmp/juniper-summer-specials-author-refined-art-laugh_{phase}_south-0.png`
sheets. Profile SHA-256:
`c92e78c28bb5122fb8685cb5bcfc601e9a2ef29f22656b189414b44d8321af3d`.

## Known art exceptions

Laugh start frame 1 and laugh end frame 2 each have two edge pixels beside
the raised gold cuff that share components with exposed skin. Local `[34,44]`
uses `763F21` and joins hand shadow `[34,43]`; `[36,44]` uses `E3BF7F` and
joins upper-arm highlight `[36,43]`. These four instances retain the established
full-skin-coverage compromise without changing the schema or color groups.
Frame numbers here are one-based.

Both separable raised-cuff bottom pixels `[34,45]` and `[35,45]` remain original,
as do far-cuff corners `[44,46]` and `[46,46]`. Literal test assertions cover
the exceptions, adjacent skin and preserved borders. Component evidence is in
`tmp/juniper-summer-specials-author-exceptions.json`. The summary and gallery
show the affected frames, with a visible note about these edges.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-specials-preview/summary.png)
shows start frame 1, loop frame 1 and end frame 2. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-specials-preview/blue-review/index.html)
shows every source frame across three detail pages with enlarged face details
covering the configured South-facing cycle.

Crop `[25,20,32,39]` includes every visible pixel, including raised hands, hair
and sandals. The exact checker verified 1,408,800 preview pixels, ten full-frame
bindings, fifteen summary bindings, frame counts and raw metadata. Chromium
checked all five pages including both indexes: every local link and image
loaded, with no horizontal overflow. Evidence:
`tmp/juniper-summer-specials-author-preview-check.log` and
`tmp/juniper-summer-specials-author-preview-browser-check.json`.

The independent candidate test and targeted Clippy passed before profile
promotion; the final focused test in `tests/juniper_summer_specials.rs` passed
afterward. It checks every new pixel in four targets, per-frame counts,
metadata, 32 literal skin/material landmarks and all 1,584 prior variant
PNG/metadata files. Practical controls drop a finger-shadow component and
merge groups; the test detects missed skin and spill into a protected far-cuff
corner. Evidence:
`tmp/juniper-summer-specials-author-candidate-test.log`,
`tmp/juniper-summer-specials-author-clippy.log` and
`tmp/juniper-summer-specials-author-final-test.log`.

All 804 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,980 prior original and variant
PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-specials-author-preservation.log` and
`tmp/juniper-summer-specials-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live timing, natural schedules,
outfit transitions or gameplay. Game-derived images remain ignored.
