# Reina Beach seated poses

Three strips add seated poses East, North and South, completing all 17 PNG
paths in Reina's current Beach folder. They contain three source frames and
one native West mirror, giving four review cases. Reina's world profile grows
from 260 to 263 sources and produces 1,052 variants. Choices remain Vanilla,
Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork on 2026-09-29.

Sources are under `assets/animations/NPCs/Reina/Sprites/Beach/`, named
`spr_npc_reina_beach_sit_{east,north,south}.png`. The fresh corpus is
`extracted/reina-beach-finish-study`; raw sidecars are recorded in
`tmp/reina-beach-finish-author-metadata.json`. Each retains one 80×80 frame,
Default atlas and `Middle`/54 offsets. Raw metadata omits frame length and
duration, preserving the native defaults. The archive hash remains
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Source hashes are strict; no earlier pin was refreshed or relaxed. Fresh
full-folder inventory is recorded in `tmp/reina-beach-finish-author-folder.txt`.

The 97 new component seeds select 180 skin pixels per target. All three actual
frames were inspected in Vanilla and all four targets. Faces, neckline, back,
arms, hands, legs and exposed feet recolor; all three green swimsuit shades,
both coral sandal shades, hair, eyes and outlines remain original. All four
existing world shades belong to skin in these strips. No source colors,
groups, mappings or material exceptions were added.

Literal landmarks distinguish the four-pixel back highlight beginning at
`[38,42]` in sit North from the green swimsuit at `[39,41]`. East-facing sandals
retain both `#BE5B52` and `#F5897F`; South-facing feet recolor below the coral
straps. The folded arms and hands retain the same four-shade skin treatment
as the accepted Beach poses.

| Strip | Skin pixels |
| --- | --- |
| Sit East | 63 |
| Sit North | 37 |
| Sit South | 80 |

- [Five-choice summary](../../generated/reina-beach-finish-preview/summary.png):
  all three source poses in all five choices.
- [Complete Vanilla/Blue review](../../generated/reina-beach-finish-preview/blue-review/index.html):
  all four cases and eight views on one page, including West.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_BEACH_FINISH_PRESETS` supports
ignored control copies. One removes the back-highlight seed `[38,42]` from
North. Another adds an ignored green alias and selects swimsuit `[39,41]`.
Production palette definitions retain their existing colors and mappings.

Both controls failed at the intended material assertion after successful
generation: omitted back shading at `[38,42]` and recolored swimsuit fabric
at `[39,41]`. The production focused test and targeted Clippy passed. Logs are
`tmp/reina-beach-finish-author-focused.log` and
`tmp/reina-beach-finish-author-{omission,spill}.log`.

All 1,052 final variants passed exact recipe validation. All 260 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,600 earlier
original and variant PNG/metadata files match the accepted Beach swimming
baseline byte for byte; all 24 new variant files match the visually inspected
candidate. All three raw sidecars match the independent archive read, and the
canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-beach-finish-author-compare.json`,
`tmp/reina-beach-finish-author-final-vs-candidate.json` and
`tmp/reina-beach-finish-author-frozen.json`.

The complete preview is 94,014 bytes, about 92 KiB. Crop `[31,27,18,25]`
contains every opaque source pixel, including the hands and feet. Saved-image
checks compared 636,750 exact rendered pixels across all-target evidence,
complete review sheets and 15 summary bindings, including source/palette
identity and exact West reversal. Chromium checked all three HTML pages,
all four cases, image decodes and local links, with no horizontal overflow.
Reports are `tmp/reina-beach-finish-author-preview.log`,
`tmp/reina-beach-finish-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-beach-finish.md) for native selection
and isolated installation.

Artwork remains ignored; this slice changes no runtime behavior. Static images
do not establish animation scheduling, transitions or seating alignment.
