# Juniper's Autumn seated reading sprites

This slice adds the Autumn seated reading start, loop and end strips. Their ten
South-facing source frames are the ten review cases. Juniper now has 231 sources
and 924 variants. All 228 earlier region objects, pins, groups and target
mappings remain unchanged, including `F1E791` and accepted earlier bracer-edge
exceptions. Portrait-only recipes are untouched.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The full corpus is `extracted/juniper-autumn-reading-study`. All 456 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-autumn-standard-trial` bundle; pins remain strict.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Autumn/`, using prefix
`spr_npc_juniper_specialanimation_autumn_read_sit_`. All retain 80×80 frames,
Default atlas and Middle/54 origin. Start and end each have three frames at
`0.1`; the four-frame loop uses `[3.0,0.1,3.0,0.1]`. Raw sidecars in
`tmp/juniper-autumn-reading-author-metadata.json` match the root's independent
archive reads. The native configuration uses a seated complex cycle with
Single packs; the gallery covers the ten South-facing source frames.

Every frame was inspected in Vanilla and all four targets. Face, ears, eyelid
skin, neck, blouse keyhole, exposed midriff and fingers change. Hair, cosmetics,
sleeves, skirt, boots, circlet, gems, cuffs and outlines remain original. Book
pages use `C9AF9C`/`F6E4D7`, outside the skin palette; their folds, pink cover and
binding remain unchanged. No colors, groups or target mappings were added.

There are no new coupled-material exceptions. Unlike the Summer reading art,
Autumn start frame 1 and end frame 3 use separable `BC8B43` cuff corners at
local `[34,45]`. These stay original while the hand directly below at `[34,46]`
changes. The closed-book cuff borders at `[34,45]` and `[45,45]` remain protected
in start/end frame 2. Finger shadows below the book and the narrow opposite
finger in the resting pose stay covered. These frame numbers are one-based.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Reading start South | 39, 24, 28 |
| Reading loop South | 20, 28, 20, 28 |
| Reading end South | 32, 20, 39 |

The masks add 138 seeds and recolor 278 pixels per target; 46 matching-color
jewelry pixels remain protected. Evidence:
`tmp/juniper-autumn-reading-author-refined-components.json`,
`tmp/juniper-autumn-reading-author-exclusions.json` and the three
`tmp/juniper-autumn-reading-author-refined-art-read_sit_{phase}_south-0.png`
sheets. Profile SHA-256:
`d85e4731c2966d7ba7599693d3841afb551687784f7ba5c95b1a322f5baf4a23`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-reading-preview/summary.png)
shows the first frame of start, loop and end. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-reading-preview/blue-review/index.html)
shows every frame across three detail pages, with enlarged face views.

Crop `[25,20,32,39]` includes every visible pixel, including the spread book,
hair and fingers. Exact checks verified 2,349,600 preview pixels, twenty full
bindings, fifteen summary bindings, frame counts and raw metadata. Chromium
checked all five pages including both indexes: local links and images loaded
without horizontal overflow. Evidence:
`tmp/juniper-autumn-reading-author-preview.log` and
`tmp/juniper-autumn-reading-author-preview-browser-check.json`.

The candidate test and targeted Clippy passed before profile promotion; the
final focused test in `tests/juniper_autumn_reading.rs` passed afterward. It
checks every new pixel in all four targets, per-frame counts, unchanged metadata,
48 literal skin/material landmarks and all 1,824 prior variant PNG/metadata
files. Effective controls omit a finger-shadow component, omit the midriff
highlight and merge color groups; they expose missed skin and cuff spill.
Evidence: `tmp/juniper-autumn-reading-author-candidate-test.log`,
`tmp/juniper-autumn-reading-author-clippy.log` and
`tmp/juniper-autumn-reading-author-focused-test.log`.

All 924 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,280 prior original/variant PNG and metadata files remain
byte-identical; all 1,848 final variant files match the inspected candidates.
Evidence: `tmp/juniper-autumn-reading-author-preservation.log`,
`tmp/juniper-autumn-reading-author-stylized-validation.json` and
`tmp/juniper-autumn-reading-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
