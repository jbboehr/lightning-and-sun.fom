# Juniper's first Winter world sprites

This slice adds Winter idle and walk North, South and East: six strips with
fifteen source frames. Five native West mirrors bring the offline review to
twenty cases. Juniper now has 245 sources and 980 variants. All 239 earlier
region objects and source pins, seventeen source colors, nine color groups,
target mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-study`. Its 478 earlier original
PNG/metadata files match the retained `characters-march-autumn-injured-trial`
bundle. Pins remain strict.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_winter_`. All retain 80×80 frames, Default atlas and origin
`Middle,54.0`. Idle uses single-frame defaults; walk has four frames per strip
at `0.15`. Raw sidecars at `tmp/juniper-winter-author-metadata.json` match the
root's independent archive reads. The shared slice verifies native direction,
frame timing and East-to-West mirroring.

Every actual frame was inspected in Vanilla and all four targets. Only face,
ears and their shadows change. The Winter outfit covers the hands and body;
North idle and walk therefore have explicit empty masks and remain byte-identical
in every preset. The warm `645049`/`836C64` bodice and leg details are clothing,
not additional skin shades. Gloves, shoulder decorations, cuffs, cyan gemstones,
gold circlet, hair and eye details stay original. No new source roles or
coupled-material exceptions are needed. All previously accepted exceptions
remain unchanged.

Literal examples distinguish the shared shades: idle South `[37,39]` is a
`763F21` cheek shadow that changes, while walk East frame 2 uses the same shade
at cuff corners `[44,45]` and `[46,45]`, which stay original. Idle South's ear
`[35,36]` changes; shoulder decoration `[35,40]` and cuff border `[33,45]`, both
`BC8B43`, do not. Idle East's upper cuff corner `[36,43]` also stays original.
Frame numbers here are one-based and coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Idle North | 0 |
| Idle South | 22 |
| Idle East | 19 |
| Walk North | 0, 0, 0, 0 |
| Walk South | 22, 21, 22, 21 |
| Walk East | 19, 19, 19, 19 |

The masks add 78 seeds and recolor 203 pixels per target. Another 98
matching-color jewelry pixels remain protected. Evidence:
`tmp/juniper-winter-author-refined-components.json`,
`tmp/juniper-winter-author-exclusions.json`,
`tmp/juniper-winter-author-inventory.log` and the six
`tmp/juniper-winter-author-refined-art-*.png` sheets. Profile SHA-256:
`5c6649fa63879fe592c62645d6332a7585e252a84cc581a01a5c6fab7e683443`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-preview/summary.png)
shows idle South, walk East frame 2 and walk North frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-preview/blue-review/index.html)
shows all fifteen source frames and five West mirrors across eight detail
pages, with enlarged face views. A visible note explains why North stays unchanged.

Crop `[25,20,32,39]` includes every visible pixel in both native and mirrored
views. Exact checks verified 4,231,200 preview pixels, forty full bindings,
fifteen summary bindings, frame counts and raw metadata. Chromium checked all
ten pages, including both indexes: local links and images loaded without
horizontal overflow. Evidence: `tmp/juniper-winter-author-preview-checks.log`
and `tmp/juniper-winter-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_winter_world.rs` (23.89 seconds). It checks every
new pixel in four targets, per-frame counts, unchanged metadata, 57 literal
material landmarks, explicit empty North masks and all 1,912 prior variant
PNG/metadata files. Three effective controls omit a cheek-shadow component,
omit an ear component and remove region restrictions. They expose missed skin
and accidental circlet, shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-author-candidate-test.log` and
`tmp/juniper-winter-author-test.log`.

All 980 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,390 prior original/variant PNG and metadata files are
byte-identical; all 1,960 final variant files match the inspected candidates.
Evidence: `tmp/juniper-winter-author-preservation.log`,
`tmp/juniper-winter-author-stylized-validation.json` and
`tmp/juniper-winter-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
