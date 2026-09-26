# Juniper's Spring laugh and charm sprites

This slice adds six Spring strips: laugh start/loop/end South and charm
start/loop/end South. Their fourteen source frames are fourteen review cases;
these cycles have no West variants. Juniper now has 166 sources and 664
variants. All 160 previous region objects, pins, color groups and mapping roles
remain unchanged. Portrait-only definitions and the three accepted eating-edge
exceptions are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-spring-specials-study`.
All 320 earlier original PNG/metadata files match the retained Spring-reactions
combined bundle; no old source pin was refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Spring/`, using
prefix `spr_npc_juniper_specialanimation_spring_`. All retain 80×80 frames,
Default atlas and numeric origin `40.0 / 54.0`. The numeric horizontal field
is preserved literally rather than rewritten as `Middle`. Laugh start uses
the single-frame defaults, loop has two frames at `[0.15, 0.125]`, and end
has two at `0.125`. Charm start has four at `[0.125, 0.1, 0.125, 0.125]`,
loop has four at `[0.5, 0.1, 0.15, 2.0]`, and end uses single-frame defaults.
Raw sidecars are in `tmp/juniper-spring-specials-author-metadata.json`.

All fourteen actual frames were inspected in Vanilla and all four targets.
Face, ears, forehead above eyelids, neck, chest, midriff, hands, arms and
exposed legs change. Matching shades on circlet corners, bracers, skirt
clasps and hem points stay original, subject to the three small exceptions
below. Hair, eye/cosmetic details, fabric, gemstones, boots and outlines
remain original. No new source colors or mapping roles were added.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Laugh start South | 53 |
| Laugh loop South | 52, 50 |
| Laugh end South | 52, 53 |
| Charm start South | 59, 70, 55, 54 |
| Charm loop South | 52, 52, 56, 52 |
| Charm end South | 52 |

The six masks use 376 component seeds and recolor 762 pixels per target;
99 matching-color material pixels remain excluded. Evidence:
`tmp/juniper-spring-specials-author-refined-components.json`,
`tmp/juniper-spring-specials-author-exclusions.json` and the six
`tmp/juniper-spring-specials-author-refined-art-{cycle}_south.png` sheets.
Profile SHA-256:
`99c29ebc8411d6e4288c33dfbbf29c36b15b55576c6b45adb0fa556092a666b2`.

## Known art exceptions

Three `E3BF7F` pixels on the raised bracer edge share a source-color component
with the `EFD89A` wrist highlight immediately above. The existing mask format
selects entire components. Keeping that wrist highlight means these three
edge pixels also change. This preserves full wrist coverage without changing
the mask schema or existing color groups. Frame numbers below are one-based;
coordinates are local to the 80×80 frame.

| Source | Frame | Bracer-edge pixel | Connected wrist pixel |
| --- | --- | --- | --- |
| Laugh loop South | 1 | `[37,42]` | `[37,41]` |
| Laugh loop South | 2 | `[37,41]` | `[37,40]` |
| Laugh end South | 1 | `[37,42]` | `[37,41]` |

The focused test names all three exceptions explicitly. The summary includes
laugh loop frame 1, and the complete gallery shows the affected laugh strips in full.

## Offline review and verification

The [five-choice summary](../../../generated/juniper-spring-specials-preview/summary.png)
shows laugh loop frame 1, charm start frame 2 and charm loop frame 3. The
[complete Vanilla/Debug Blue gallery](../../../generated/juniper-spring-specials-preview/blue-review/index.html)
covers all fourteen cases across six detail pages, at most four cases per
page, with enlarged face details. It contains no invented West mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including outstretched
hands, hair and boots. Exact checks verified 3,102,240 preview pixels,
28 full-frame bindings, fifteen summary bindings, source frame counts and
raw metadata. Chromium checked all eight pages, including both indexes:
every image and local link loaded, with no horizontal overflow. Evidence:
`tmp/juniper-spring-specials-author-preview-check.log` and
`tmp/juniper-spring-specials-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy
passed before profile promotion; the final focused test in
`tests/juniper_spring_specials.rs` then passed. It checks every new pixel in
four targets, per-frame counts, metadata, 36 literal skin/material landmarks
and all 1,280 prior variant PNG/metadata files. Practical controls remove an
upper-arm shadow component and merge the color groups; the test detects
missed skin and spill into a protected bracer. Evidence:
`tmp/juniper-spring-specials-author-candidate-test.log`,
`tmp/juniper-spring-specials-author-clippy.log` and
`tmp/juniper-spring-specials-author-final-test.log`.

All 664 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,600 previous original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-spring-specials-author-preservation.log` and
`tmp/juniper-spring-specials-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live animation timing,
attached effects, natural schedules or gameplay. The user approved the artwork
for commit on 2026-09-26. Game-derived images remain ignored.
