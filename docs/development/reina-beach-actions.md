# Reina Beach blinking, general actions and kiss

Six strips add Beach blinking East/South, general actions North/South/East and
kiss East. They contain 31 source frames and 14 native West mirrors, giving
45 review cases. Reina's world profile grows from 252 to 258 sources and
produces 1,032 variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and
Seridia. This artwork awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Beach/`, named
`spr_npc_reina_beach_{action,blink,kiss}_{direction}.png`. The fresh corpus is
`extracted/reina-beach-actions-study`; raw sidecars are recorded in
`tmp/reina-beach-actions-author-metadata.json`. All six retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Action durations are
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; blinking retains
`[0.075,0.125,0.075]`, and kiss retains `[0.15,0.15,0.8,0.15]`.
Source hashes are strict; no earlier pin was refreshed or relaxed.

The 1,293 new component seeds select 2,521 skin pixels per target, covering
faces, closed eyelids, kiss-mouth shading, necklines, torso, arms, moving hands,
legs and exposed feet. All 31 actual frames were inspected in Vanilla and all
four targets. The three green swimsuit colors `#30792E`, `#47C744` and
`#9EF671`, coral sandal straps `#BE5B52`/`#F5897F`, hair, eyes and outlines
remain original. All four existing world shades belong to skin in these
strips; no source colors, groups, mappings or material exceptions were added.

Literal landmarks cover the seven-pixel hand highlight beginning at strip
coordinate `[125,43]` in action East, the neighboring hand shadows and green
swimsuit fabric at `[121,46]`. Separate checks preserve both coral sandal
shades and the black closed eyelids, while recoloring the cheek, jaw and
kiss-mouth skin shades in the third and fourth kiss frames.

| Strip | Skin pixels per frame |
| --- | --- |
| Action East | 69, 79, 76, 79, 76, 69, 83 |
| Action North | 61, 56, 57, 56, 57, 63, 65 |
| Action South | 90, 90, 87, 90, 87, 90, 100 |
| Blink East | 89, 99, 89 |
| Blink South | 104, 114, 104 |
| Kiss East | 77, 85, 91, 89 |

- [Five-choice summary](../../generated/reina-beach-actions-preview/summary.png):
  second-frame actions South/East, third-frame kiss and second-frame blink South.
- [Complete Vanilla/Blue review](../../generated/reina-beach-actions-preview/blue-review/index.html):
  all 45 cases and 90 views across six pages of at most eight cases.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_BEACH_ACTIONS_PRESETS` supports
ignored control copies. One removes the hand-highlight seed `[125,43]` from
action East. Another adds an ignored green alias and selects swimsuit fabric
`[121,46]`. Production palette definitions retain their existing colors and
mappings. Both controls failed at the intended material assertion after successful
generation. The production focused test and targeted Clippy passed.
Logs are `tmp/reina-beach-actions-author-focused.log` and
`tmp/reina-beach-actions-author-{omission,spill}.log`.

All 1,032 final variants passed exact recipe validation. All 252 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,520 earlier
original and variant PNG/metadata files match the accepted Beach pilot baseline
byte for byte; all 48 new variant files match the visually inspected candidate.
The six raw sidecars match the independent archive read, and the canonical
Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-beach-actions-author-compare.json`,
`tmp/reina-beach-actions-author-final-vs-candidate.json` and
`tmp/reina-beach-actions-author-frozen.json`.

The complete preview is 566,309 bytes, about 553 KiB. Crop `[30,26,21,29]`
contains every opaque source pixel, including the extended fingers and feet.
Saved-image checks compared 8,035,755 exact rendered pixels across all-target
evidence, complete review sheets and 20 summary bindings, including
source/palette identity and exact West reversal. Chromium checked all eight
HTML pages, all 45 cases, image decodes and local links, with no horizontal
overflow. Reports are `tmp/reina-beach-actions-author-preview.log`,
`tmp/reina-beach-actions-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-beach-actions.md) for native selection
and isolated installation.

Artwork remains ignored; this slice changes no runtime behavior. Static images
do not establish animation timing, scheduling or transitions. Beach swimming
remains for a later batch.
