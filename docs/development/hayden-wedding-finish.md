# Hayden's remaining Wedding sprites

Nine strips add blink East/South, sit and action North/South/East, and kiss
East. Their 34 source frames produce 49 review cases with 15 native West
mirrors. Wedding now has all 15 source strips covered; Hayden's profile has
305 sources and 1,220 variants. All 296 accepted regions, source pins, color
groups and palette mappings remain unchanged. The user approved this artwork for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources use the `assets/animations/NPCs/Hayden/Sprites/Wedding/` directory and
`spr_npc_hayden_wedding_` prefix. The full corpus is
`extracted/hayden-wedding-finish-study`; raw sidecars are recorded in
`tmp/hayden-wedding-finish-author-metadata.json` and match the independent
archive read exactly. All retain 80×80 frames, Default atlas and Middle/54
origin. Sit uses native defaults; blink preserves `[0.075,0.125,0.075]`, action
preserves `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, and kiss preserves
`[0.15,0.15,0.8,0.15]`. No source hash was changed or relaxed.

The 507 reviewed component seeds select 750 skin pixels per target. Faces,
eyelids, cheek edges and fingers use the existing three lighter world shades;
14 isolated East face highlights use the existing `#E8B271` mapping. All 152
`#523C26` shoe pixels stay original, including the extra sole row in North
action frames. Cuffs, collar, suit, vest, flower, shoes, hair, beard, eyes,
outlines and transparency stay original. The tiny raised North-action
fingertip is included. Every actual frame was inspected in Vanilla and all
four targets using source material grids and local all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 26, 30, 26 |
| Blink South | 31, 35, 31 |
| Sit East / North / South | 23 / 18 / 32 |
| Action East | 22, 23, 23, 23, 23, 22, 23 |
| Action North | 8, 6, 6, 6, 6, 8, 12 |
| Action South | 25, 26, 28, 26, 28, 25, 28 |
| Kiss East | 21, 24, 28, 28 |

- [Five-choice summary](../../generated/hayden-wedding-finish-preview/summary.png):
  closed blink, seated, action and kiss samples with frame numbers.
- [Complete Vanilla/Blue review](../../generated/hayden-wedding-finish-preview/blue-review/index.html):
  all 49 cases and 98 views across seven pages of at most eight cases.

Final sheets and previews use the actual standalone bundle. Crop
`[30,25,22,30]` includes every opaque pixel. Saved-image checks cover
9,484,200 exact pixels across all-target sheets, review sheets and 20 summary
bindings, including source/palette identity and exact West reversal.
Chromium decoded every image across all nine HTML pages, checked every case
and local link, and found no horizontal overflow. The complete preview
directory is about 792 KiB. Evidence is in
`tmp/hayden-wedding-finish-author-preview-check.log`,
`tmp/hayden-wedding-finish-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before candidate
promotion. It checks 93 literal skin/material landmarks, every source pixel
against the independently reviewed material classes, exact per-frame counts,
alpha, metadata, common masks and all prior output bytes. Production data
was checked for exact object equality with that passing candidate. Removing
the North-action fingertip failed at `[47,37]`; selecting a shoe sole failed
at `[117,53]`. Both intended failures occurred in the material assertion after
successful generation. `FOM_HAYDEN_WEDDING_FINISH_PRESETS` supports these
ignored negative-control copies. Logs are
`tmp/hayden-wedding-finish-author-test.log` and
`tmp/hayden-wedding-finish-author-{omission,spill}-test.log`.

All 1,220 final variants passed exact recipe validation. All 2,960 accepted
original/variant PNG and metadata files remain byte-identical, and all 72 new
variant PNG/metadata files match the inspected candidate exactly. Reports
are `tmp/hayden-wedding-finish-author-compare.json` and
`tmp/hayden-wedding-finish-author-final-vs-candidate.json`. Frozen data and test
hashes are in `tmp/hayden-wedding-finish-author-frozen-inputs.sha256`.

See [combined integration](world-wedding-finish.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, NPC scheduling or state transitions. Artwork stays ignored;
this character slice changes no runtime code.
