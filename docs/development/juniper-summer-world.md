# Juniper's Summer idle and walk sprites

This slice adds six Summer idle/walk strips facing North, South and East.
Their fifteen source frames plus five native West mirrors make twenty review
cases. Juniper now has 179 sources and 716 variants. All 173 earlier region
objects, pins, color groups and target mappings remain unchanged, including
the accepted Spring eating, laugh and hair-flip edge exceptions. Portrait-only
recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-summer-world-study`.
All 346 earlier original PNG/metadata files match the retained
`characters-march-spring-injured-trial` combined bundle. No old source pin was
refreshed or bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Summer/`, named
`spr_npc_juniper_summer_{idle,walk}_{north,south,east}.png`. All retain 80×80
frames, Default atlas and Middle/54 origin. Idle uses single-frame defaults;
walking has four frames at duration `0.15`. The East packs supply West by
mirroring. Raw exported sidecars are in
`tmp/juniper-summer-world-author-metadata.json` and agree with the independent
shared inventory.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
neckline, midriff, bare shoulders and arms, hands, slit-visible legs and skin
between sandal straps change. The Summer outfit exposes additional upper-arm
highlights and dark underarm pixels beside the bracers; these are included.
Matching shades on circlet corners and gold bracers remain original. Purple
hair, eye details, pink gems, the blouse, blue skirt and lavender sandal wraps
are preserved. No new source colors, groups, mapping roles or inseparable
material-edge exceptions were needed.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Idle North | 19 |
| Idle South | 62 |
| Idle East | 51 |
| Walk North | 19, 17, 19, 11 |
| Walk South | 62, 56, 62, 57 |
| Walk East | 51, 53, 51, 51 |

The six masks use 333 new seeds and recolor 641 pixels per target;
68 matching-color material pixels remain excluded. Evidence:
`tmp/juniper-summer-world-author-refined-components.json`,
`tmp/juniper-summer-world-author-exclusions.json` and the six
`tmp/juniper-summer-world-author-refined-art-{action}_{direction}-0.png` sheets.
Profile SHA-256:
`76db2ab16e9cd239aade32e13f393ed01ec71790a6781562bad20e79d23598f2`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-summer-world-preview/summary.png)
shows idle South, walk East frame 2 and walk North frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-summer-world-preview/blue-review/index.html)
covers all twenty cases across eight detail pages, at most four cases per
page, with enlarged face details and exact native West mirrors.

Crop `[25,20,32,39]` includes every visible pixel, including moving hair,
hands and sandals. Exact checks verified 4,231,200 preview pixels, forty
full-frame bindings, fifteen summary bindings, source frame counts, raw
metadata and mirrored coordinates. Chromium checked all ten pages, including
both indexes: every image and local link loaded, with no horizontal overflow.
Evidence: `tmp/juniper-summer-world-author-preview-check.log` and
`tmp/juniper-summer-world-author-preview-browser-check.json`.

The independent candidate test passed before promotion. Targeted Clippy
passed before profile promotion; the final focused test in
`tests/juniper_summer_world.rs` then passed. It checks every new pixel in four
targets, per-frame counts, metadata, 44 literal skin/material landmarks and
all 1,384 earlier variant PNG/metadata files. Practical controls remove a
finger-shadow component and merge the color groups; the test detects missed
skin and spill into a protected bracer. Evidence:
`tmp/juniper-summer-world-author-candidate-test.log`,
`tmp/juniper-summer-world-author-clippy.log` and
`tmp/juniper-summer-world-author-final-test.log`.

All 716 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 1,730 earlier original and
variant PNG/metadata files remain byte-identical. Evidence:
`tmp/juniper-summer-world-author-preservation.log` and
`tmp/juniper-summer-world-author-stylized-validation.json`.

The shared slice handles broader checks, combined packaging, native probes
and installation. Static previews do not verify live animation timing,
natural schedules, outfit transitions or gameplay. The user approved this artwork for commit on 2026-09-26. Game-derived images remain ignored.
