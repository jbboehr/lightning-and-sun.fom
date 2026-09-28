# Juniper's final Autumn sprites

This slice adds spell-casting start, loop and end, hair flip and snooze: five
strips with 22 South-facing source frames. Juniper now has 239 sources and 956
variants, covering all 33 animation strips in the archive's Autumn folder.
The previous 234 region objects and source pins, seventeen source colors, nine
color groups, all target mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-autumn-finish-study`. Its 468 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-autumn-specials-trial` bundle. Pins remain strict.
The independent folder inventory is
`tmp/juniper-autumn-finish-author-autumn-inventory.json`.

Sources use `assets/animations/NPCs/Juniper/Sprites/Autumn/` and prefix
`spr_npc_juniper_specialanimation_autumn_`. All retain 80×80 frames, Default
atlas and numeric origin `40.0,54.0`. Spell start has two frames and loop four,
both at `0.125`; spell end uses single-frame defaults. Hair flip has six frames
with durations `[0.15,0.15,0.125,0.125,0.5,0.125]`. Snooze has nine with durations
`[0.3,0.8,0.3,3.0,0.15,1.5,0.15,0.125,1.5]`. Raw sidecars at
`tmp/juniper-autumn-finish-author-metadata.json` match the root's independent
archive reads. The shared slice checks native cycle configuration separately.

Every actual frame was inspected in Vanilla and all four targets. Face, exposed
forehead, ears, keyhole, midriff, hands and finger shadows change. The existing
`B58E45` and `E0B572` roles cover the keyhole and midriff without further mapping
changes. Hair, cosmetics, mouth details, circlet, gemstones, sleeves, skirt,
boots and separable cuff borders stay original. These strips cover Juniper's
body animation; they do not add separate magic-effect assets.

There are eight new coupled-material exceptions. Spell loop frames 1–4 each
recolor cuff-edge pixels `[33,44]` and `[46,44]` (`BC8B43`). They connect directly
to same-color hand shadows `[32,44]` and `[47,44]`; the existing component masks
cannot separate each pair. Keeping full hand coverage follows the earlier
accepted approach. The gold immediately above at `[33,43]` and `[46,43]` stays
original. The gallery visibly names this compromise. All earlier accepted
exceptions remain unchanged. Frame numbers here are one-based; coordinates
are frame-local.

Other boundaries are separable. Hair flip frames 4–5 preserve cuff borders
`[31,46]` and `[45,41]` while the hand at `[44,40]` changes. Snooze frames 1–5
preserve the cuff at `[43,40]` while the hand edge `[45,40]` changes. In snooze
frame 8, `[45,40]` is instead an exposed ear under the displaced hair and changes;
the gold cuff lower down remains original. Spell loop's exposed forehead above
the moving circlet is included.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Spell start South | 48, 52 |
| Spell loop South | 64, 63, 60, 58 |
| Spell end South | 48 |
| Hair flip South | 32, 44, 44, 41, 41, 41 |
| Snooze South | 45, 41, 45, 41, 45, 29, 27, 24, 45 |

The five masks add 472 seeds and recolor 978 pixels per target, including the
eight documented edges. Another 105 matching-color jewelry pixels remain
protected. Evidence: `tmp/juniper-autumn-finish-author-refined-components.json`,
`tmp/juniper-autumn-finish-author-exclusions.json`,
`tmp/juniper-autumn-finish-author-exceptions.json` and the eight
`tmp/juniper-autumn-finish-author-refined-art-*.png` sheets. Profile SHA-256:
`c4f95463965b664b9a7867a2200b4e12f71d69d9854d4c6b8f8f6508c2938315`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-finish-preview/summary.png)
shows spell loop frame 1, hair flip frame 3 and snooze frame 9. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-finish-preview/blue-review/index.html)
shows all 22 source frames across eight detail pages, with enlarged face views.

Crop `[25,20,32,39]` includes every visible pixel. Exact checks verified
4,607,520 preview pixels, 44 full bindings, fifteen summary bindings, frame
counts and raw metadata. Chromium checked all ten pages, including both
indexes: local links and images loaded without horizontal overflow. Evidence:
`tmp/juniper-autumn-finish-author-preview-checks.log` and
`tmp/juniper-autumn-finish-author-preview-browser-check.json`.

The candidate material test and targeted Clippy passed before promotion. The
final focused test in `tests/juniper_autumn_finish.rs` passed afterward in
23.42 seconds. It checks every new pixel in four targets, per-frame counts,
unchanged metadata, 80 literal material landmarks and all 1,872 prior variant
PNG/metadata files. Four effective controls omit a finger-shadow component,
omit each keyhole alias component and merge groups; they expose missed skin
and cuff spill. Evidence: `tmp/juniper-autumn-finish-author-candidate-test.log`,
`tmp/juniper-autumn-finish-author-clippy.log` and
`tmp/juniper-autumn-finish-author-test.log`.

All 956 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,340 prior original/variant PNG and metadata files are
byte-identical; all 1,912 final variant files match the inspected candidates.
Evidence: `tmp/juniper-autumn-finish-author-preservation.log`,
`tmp/juniper-autumn-finish-author-stylized-validation.json` and
`tmp/juniper-autumn-finish-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.

The user approved this artwork for commit on 2026-09-27, including the eight
documented cuff-edge pixel occurrences.
