# Juniper's Autumn idle and walk sprites

This pilot adds Autumn idle and walk in North, South and East. The six strips
contain 15 source frames; five native West mirrors make 20 offline review cases.
Juniper now has 212 sources and 848 variants. All 206 earlier region objects and
pins remain unchanged, including the previously approved bracer exceptions.
Previous color/group/target entries stay exact; one highlight alias is appended.
Portrait-only recipes are untouched.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-autumn-world-study`.
All 412 earlier original PNG/metadata files match the retained
`characters-march-summer-injured-trial` bundle. No previous source pin was
refreshed or bypassed.

Sources use `assets/animations/NPCs/Juniper/Sprites/Autumn/` and the exact names
`spr_npc_juniper_autumn_{idle,walk}_{north,south,east}.png`.
All retain 80×80 frames, Default atlas and Middle/54 origin. Each idle uses
single-frame defaults; every walk has four frames at `0.15`. Raw sidecars in
`tmp/juniper-autumn-world-author-metadata.json` match the independent archive
reads. West review images use the native horizontal mirror of the East strip.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
neckline, exposed midriff and hands change. The Autumn outfit covers the upper
arms and legs, so sleeves, skirt and boots stay original. Circlet corners,
cuff borders, red gems, eye cosmetics, hair and outlines are protected.
In particular, North-view cuff borders `[33,45]` and `[46,45]` are jewelry,
and the turned South-walk cuffs use isolated `E3BF7F` pixels that remain
original. Actual finger and hand outlines below those cuffs still change.
No new coupled-material exceptions are required.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Idle North | 12 |
| Idle South | 44 |
| Idle East | 37 |
| Walk North | 12, 6, 12, 7 |
| Walk South | 44, 39, 44, 39 |
| Walk East | 37, 40, 37, 33 |

The masks use 213 new seeds and recolor 443 pixels per target; 65
matching-color circlet/cuff pixels remain excluded. Evidence:
`tmp/juniper-autumn-world-author-refined-components.json`,
`tmp/juniper-autumn-world-author-exclusions.json` and the six
`tmp/juniper-autumn-world-author-refined-art-*.png` sheets. Profile SHA-256:
`3c53242c6177fb52127262178bd322b66340fba0272b7aae231486135b3aa538`.

## Midriff highlight alias

The South-facing art uses `F1E791` at `[39,45]` and `[40,45]` in idle and
walk frames 1/3, shifted to y46 in walk frames 2/4. These ten pixels sit inside
the exposed midriff, surrounded by `E3BF7F` and `BC8B43` skin shading; equivalent
East-facing geometry uses the existing `EFD89A` highlight. `F1E791` is appended
at source-color index 14 with its own singleton group and the existing highlight
target in each preset: Debug Blue `9DB9D4`, Hayden `E8B271`, Ryis `B06C57`,
Seridia `C1AFA5`. The stylized Blue map receives the same alias. All earlier
indices, groups and map entries remain exact.

The singleton group prevents the alias from extending earlier selected skin
components. Every old original and recolored output was compared byte-for-byte.
Gold cuff colors `F8F960`, `FFF45D` and `FBCC5A` retain their original pixels.
Literal tests distinguish these materials from the new midriff highlight.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-world-preview/summary.png)
shows idle South frame 1, walk East frame 2 and walk North frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-world-preview/blue-review/index.html)
includes every source frame and all five West mirrors across eight detail pages,
with enlarged face views.

Crop `[25,20,32,39]` includes every visible pixel, including moving hair, hands
and boots. Exact checks verified 4,231,200 preview pixels, 40 full-frame bindings,
fifteen summary bindings, raw metadata, frame counts and West mirrors. Chromium
checked all ten pages including both indexes: every local link and image loaded,
with no horizontal overflow. Evidence:
`tmp/juniper-autumn-world-author-preview-check.log` and
`tmp/juniper-autumn-world-author-preview-browser-check.json`.

Both independent candidate tests and targeted Clippy passed before promotion;
the final focused tests in `tests/juniper_autumn_world.rs` passed afterward.
The tests assert every prior palette role plus the appended alias, every new
pixel in four targets, per-frame counts, metadata, 50 literal material/skin
landmarks and all 1,648 prior variant PNG/metadata files. Practical controls omit
a finger-shadow component, omit the new midriff highlight component, and merge
groups; the tests detect both missed skin and spill into a protected cuff.
Evidence: `tmp/juniper-autumn-world-author-candidate-test.log`,
`tmp/juniper-autumn-world-author-clippy.log` and
`tmp/juniper-autumn-world-author-final-test.log`.

All 848 final variants passed exact-palette validation, including a separate
check using the stylized Debug Blue recipe. All 2,060 prior original and variant
PNG/metadata files remain byte-identical. All 1,696 final variant PNG/metadata
files match the inspected candidates. Evidence:
`tmp/juniper-autumn-world-author-preservation.log`,
`tmp/juniper-autumn-world-author-stylized-validation.json` and
`tmp/juniper-autumn-world-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
installation. Static previews do not establish gameplay timing, schedules or
outfit transitions. Game-derived images remain ignored.
