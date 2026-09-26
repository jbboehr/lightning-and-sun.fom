# Juniper's Spring action, sleep and kiss sprites

This slice adds five Spring strips: general action North/South/East, sleep
East and kiss East. Their 26 source frames and twelve native West mirrors
make 38 review cases. Juniper now has 154 sources and 616 variants. All 149
previous region objects, source hashes, color groups and mapping roles remain
unchanged. Portrait-only definitions and the three accepted eating-edge
exceptions are untouched.

## Sources and material decisions

The source is the read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-spring-standard-study`.
The 298 earlier original PNG/metadata files match the retained Spring-actions
combined bundle; no old pin was refreshed or bypassed.

The new sources under `assets/animations/NPCs/Juniper/Sprites/Spring/` use
prefix `spr_npc_juniper_spring_`. All retain 80×80 frames, Default atlas and
Middle/54 origin. General action has seven frames with durations
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss has four with
`[0.15, 0.15, 0.8, 0.15]`; sleep uses the single-frame defaults.
Raw sidecars are recorded in
`tmp/juniper-spring-standard-author-metadata.json`.

All 26 actual frames were inspected in Vanilla and all four target palettes.
Face, ears, neck, chest, midriff, hands, arms and exposed skin above the boots
change. Matching shades on the circlet, bracers, hem and trailing skirt points
stay original. Hair, eyes, gemstones, bodice, skirt fabric, boots and outlines
remain original. No source colors or mapping roles were added. No new
inseparable skin/material boundary exceptions were identified.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Action East | 34, 39, 35, 39, 35, 34, 41 |
| Action North | 6, 6, 6, 6, 6, 8, 11 |
| Action South | 46, 45, 44, 45, 44, 46, 50 |
| Kiss East | 38, 37, 42, 41 |
| Sleep East | 41 |

The five masks use 404 component seeds and recolor 825 pixels per target;
160 matching-color material pixels remain excluded. Evidence is in
`tmp/juniper-spring-standard-author-refined-components.json`,
`tmp/juniper-spring-standard-author-exclusions.json` and the five
`tmp/juniper-spring-standard-author-refined-art-{cycle}_{direction}.png` sheets.
Profile SHA-256:
`0928ce012a1efdcb32a13c376d83a0619f0fec8ad8bcc28b6e60bf29f88cd1e8`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-spring-standard-preview/summary.png)
shows action South frame 3, sleep East and kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-spring-standard-preview/blue-review/index.html)
covers all 38 cases across eight detail pages, with at most seven cases per
page and enlarged face details. Both indexes and all detail pages are local.

Crop `[25,20,32,39]` contains every visible pixel, including hands, hair,
boots and West mirrors. Exact checks verified 7,618,080 preview pixels,
76 full-frame bindings, fifteen summary bindings, raw metadata, source frame
counts and West reversal. Chromium checked all ten pages: every image and
local link loaded, with no horizontal overflow. Evidence:
`tmp/juniper-spring-standard-author-preview-check.log` and
`tmp/juniper-spring-standard-author-preview-browser-check.json`.

The candidate test passed before promotion. Targeted Clippy passed before
profile promotion, and the final focused test in `tests/juniper_spring_standard.rs`
passed afterward. It checks every new pixel in all four targets, per-frame
counts, unchanged metadata, 37 literal skin/material landmarks and all 1,192
previous variant PNG/metadata files. Practical controls remove an upper-arm
shadow component and merge the color groups; the test detects the resulting
missed skin and spill into a gold bracer. Evidence:
`tmp/juniper-spring-standard-author-candidate-test.log`,
`tmp/juniper-spring-standard-author-clippy.log` and
`tmp/juniper-spring-standard-author-final-test.log`.

All 616 final variants pass exact-palette validation, including a separate
check using the Debug Blue stylized recipe. All 1,490 earlier original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-spring-standard-author-preservation.log` and
`tmp/juniper-spring-standard-author-stylized-validation.json`.

The [shared slice](reina-juniper-march-standard.md) handles combined packaging,
native probes, installation and broader checks. Static previews do not verify
natural schedules, attached effects, animation timing or live gameplay.
The user approved this artwork for commit on 2026-09-25. Game-derived images remain ignored.
