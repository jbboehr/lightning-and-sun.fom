# Reina Wedding actions, blinking, sitting and kiss

Nine strips finish all 15 PNG paths in Reina's current Wedding folder: general
actions North/South/East, blinking East/South, sitting North/South/East and kiss
East. They contain 34 source frames and 15 native West mirrors, giving 49 review
cases. Reina's world profile grows from 269 to 278 sources and produces 1,112
variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. This
artwork awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Wedding/`, named
`spr_npc_reina_wedding_{action,blink,sit,kiss}_{direction}.png`. The fresh
corpus is `extracted/reina-wedding-finish-study`; raw sidecars are recorded in
`tmp/reina-wedding-finish-author-metadata.json`. All nine retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Action durations remain
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, blinking `[0.075,0.125,0.075]`, kiss
`[0.15,0.15,0.8,0.15]` and sitting single-frame defaults.

The mounted archive SHA256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Source hashes remain strict; no earlier pin was refreshed or relaxed. Fresh
full-folder inventory is recorded in `tmp/reina-wedding-finish-author-folder.txt`.

The 856 new component seeds select 1,545 skin pixels per target. All 34 actual
frames were inspected in Vanilla and all four targets. Masks cover the moving
arms and hands, faces, closed eyelids, kiss-mouth skin shading and exposed feet.
Seated North exposes exactly four hand pixels, all included. Pale dress fabric,
pink trim and hair decoration, gold accessories, hair, eyes and outlines remain
original. Thirty-two new North-facing gold accessory-shadow pixels reuse
portrait skin color `#B36644`, four per frame. Those components remain excluded,
as do the pilot's earlier 20 accessory pixels. No source colors, groups or
mappings were added or changed.

Literal landmarks cover the seven-pixel extended-hand highlight beginning at
strip coordinate `[125,43]` in action East, the neighboring hand shadows and
pale dress fabric. All four tiny North sit hand pixels are asserted separately,
as are the four protected accessory pixels in each North frame. Blink and kiss
landmarks distinguish skin shading from the original black eye/mouth lines.

| Strip | Skin pixels per frame |
| --- | --- |
| Action East | 47, 55, 52, 55, 52, 47, 52 |
| Action North | 9, 13, 13, 13, 13, 12, 16 |
| Action South | 55, 55, 56, 55, 56, 55, 63 |
| Blink East | 56, 64, 56 |
| Blink South | 67, 75, 67 |
| Kiss East | 47, 55, 63, 59 |
| Sit East | 42 |
| Sit North | 4 |
| Sit South | 46 |

- [Five-choice summary](../../generated/reina-wedding-finish-preview/summary.png):
  second-frame actions South/East, third-frame kiss and seated North.
- [Complete Vanilla/Blue review](../../generated/reina-wedding-finish-preview/blue-review/index.html):
  all 49 cases and 98 views across seven pages.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_WEDDING_FINISH_PRESETS` supports
ignored control copies. One removes hand-highlight seed `[125,43]` from action
East. Another selects gold accessory shadow `[35,38]` in seated North,
exercising the actual portrait-color collision without adding a palette alias.

Both controls failed at the intended material assertion after successful
generation: omitted hand shading at `[125,43]`, and recolored accessory shadow
at `[35,38]`. The production focused test and targeted Clippy passed. Logs are
`tmp/reina-wedding-finish-author-focused.log` and
`tmp/reina-wedding-finish-author-{omission,spill}.log`.

All 1,112 final variants passed exact recipe validation. All 269 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,690 earlier
original and variant PNG/metadata files match the accepted Wedding pilot
baseline byte for byte; all 72 new variant files match the visually inspected
candidate. The nine raw sidecars match the independent archive read, and the
canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-wedding-finish-author-compare.json`,
`tmp/reina-wedding-finish-author-final-vs-candidate.json` and
`tmp/reina-wedding-finish-author-frozen.json`.

The complete preview is 628,180 bytes, about 613 KiB. Crop `[31,26,21,29]`
contains every opaque source pixel, including extended fingers and dress folds.
Saved-image checks compared 8,751,330 exact rendered pixels across all-target
evidence, complete review sheets and 20 summary bindings, including
source/palette identity and exact West reversal. Chromium checked all nine
HTML pages, all 49 cases, image decodes and local links, with no horizontal
overflow. Reports are `tmp/reina-wedding-finish-author-preview.log`,
`tmp/reina-wedding-finish-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-wedding-finish.md) for native selection
and isolated installation.

Artwork remains ignored; this slice changes no runtime behavior. Static images
do not establish animation timing, scheduling, transitions or seating alignment.
