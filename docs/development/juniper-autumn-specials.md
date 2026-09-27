# Juniper's Autumn laugh sprites

This slice adds the Autumn laugh start, loop and end strips. Their five
South-facing source frames are the five review cases. Juniper now has 234
sources and 936 variants. All 231 earlier region objects and source pins remain
unchanged. Two newly observed skin shades extend the existing palette roles;
all earlier colors, groups and target values retain their exact entries.
Portrait-only recipes and previously accepted material exceptions are unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The full corpus is `extracted/juniper-autumn-specials-study`. All 462 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-autumn-reading-trial` bundle. Pins remain strict.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Autumn/`, using prefix
`spr_npc_juniper_specialanimation_autumn_laugh_`. All retain 80×80 frames,
Default atlas and numeric origin `40.0,54.0`. Start uses single-frame defaults;
the two-frame loop uses `[0.15,0.125]`; the two-frame end uses `0.125`. Raw
sidecars in `tmp/juniper-autumn-specials-author-metadata.json` match the root's
independent archive reads, including the numeric horizontal origin.

Every frame was inspected in Vanilla and all four target palettes. Face, ears,
eyelid skin, neckline, exposed midriff, raised hand and opposite fingers change.
Hair, cosmetics, mouth interior, circlet, gems, sleeves, skirt, boots and
separable cuff borders stay original. There are no new coupled-material
exceptions. Start frame 1 and end frame 2 preserve the cuff corner `[46,46]`
while the finger shadow `[46,48]` changes. Loop frames retain cuff gold below
the raised hand and the black open-mouth interior. These frame numbers are
one-based and coordinates are frame-local.

## Two additional source shades

The actual Autumn laugh art uses `B58E45` and `E0B572` in the blouse keyhole and
midriff where the earlier Autumn sprites use `BC8B43` and `E3BF7F`. Each of the
five frames contains five shadow-alias pixels and three midtone-alias pixels:
40 additional skin pixels across the batch. The center midriff highlight remains
`F1E791`. Literal examples in start frame 1 are `[39,43]`/`[40,43]` in the
keyhole and `[37,46]`/`[38,46]` at the midriff edge.

The world profile appends `B58E45`, then `E0B572`, with singleton groups. Source
colors increase from fifteen to seventeen and groups from seven to nine. Each
preset appends its existing shadow and midtone targets, at old indices 12 and
1. The stylized map appends `B58E45→6687AD` and `E0B572→7F9FBD`. Singleton
groups and actual component seeds keep these additions local; all previous
portrait/world output bytes remain identical, including books and accessories.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Laugh start South | 50 |
| Laugh loop South | 43, 39 |
| Laugh end South | 43, 50 |

The masks add 122 seeds and recolor 225 pixels per target; 22 matching-color
jewelry pixels remain protected. Evidence:
`tmp/juniper-autumn-specials-author-refined-components.json`,
`tmp/juniper-autumn-specials-author-exclusions.json` and the three
`tmp/juniper-autumn-specials-author-refined-art-laugh_{phase}_south-0.png`
sheets. Profile SHA-256:
`9071bea4f573b937349ba82fc3db68393526606a4957b86cd1ea9eadbe3f7a92`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-specials-preview/summary.png)
shows the first frame of start, loop and end. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-specials-preview/blue-review/index.html)
shows every frame across three detail pages, with enlarged face views.

Crop `[25,20,32,39]` includes every visible pixel. Exact checks verified
1,408,800 preview pixels, ten full bindings, fifteen summary bindings, frame
counts and raw metadata. Chromium checked all five pages including both
indexes: local links and images loaded without horizontal overflow. Evidence:
`tmp/juniper-autumn-specials-author-preview.log` and
`tmp/juniper-autumn-specials-author-preview-browser-check.json`.

Both candidate tests and targeted Clippy passed before promotion; both final
tests in `tests/juniper_autumn_specials.rs` passed afterward. They check exact
append-only palette roles, every new pixel in four targets, per-frame counts,
unchanged metadata, 46 literal material landmarks and all 1,848 prior variant
PNG/metadata files. Controls omit a finger-shadow component, omit each new
keyhole alias component and merge groups; they expose missed skin and cuff
spill. Evidence: `tmp/juniper-autumn-specials-author-candidate-test.log`,
`tmp/juniper-autumn-specials-author-clippy.log` and
`tmp/juniper-autumn-specials-author-focused-test.log`.

All 936 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,310 prior original/variant PNG and metadata files are
byte-identical; all 1,872 final variant files match the inspected candidates.
Evidence: `tmp/juniper-autumn-specials-author-preservation.log`,
`tmp/juniper-autumn-specials-author-stylized-validation.json` and
`tmp/juniper-autumn-specials-author-final-diff-check.log`, which also records
profile, set, style and test hashes.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
