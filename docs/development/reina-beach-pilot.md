# Reina Beach idle and walk

Six strips add Beach idle and walking North/South/East. They contain 15 source
frames and five native West mirrors, giving 20 review cases. Reina's world
profile grows from 246 to 252 sources and produces 1,008 variants. Choices
remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved the
offline artwork on 2026-09-28.

Sources are under `assets/animations/NPCs/Reina/Sprites/Beach/`, named
`spr_npc_reina_beach_{idle,walk}_{north,south,east}.png`. The fresh corpus is
`extracted/reina-beach-pilot-study`; raw sidecars are recorded in
`tmp/reina-beach-pilot-author-metadata.json`. All six retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Idle retains single-frame defaults;
walking retains four frames at 0.15 seconds. Source hashes are strict; no
earlier pin was refreshed or relaxed.

The 648 new component seeds select 1,212 skin pixels per target, covering faces,
necklines, torso, arms, hands, legs and exposed feet. All 15 actual frames were
inspected in Vanilla and all four targets. The three green swimsuit colors
`#30792E`, `#47C744` and `#9EF671`, coral sandal straps `#BE5B52`/`#F5897F`,
hair, eyes and outlines remain original. All four existing world shades belong
to skin in these strips; no source colors, groups, mappings or special material
exclusions were added.

Literal landmarks distinguish the three-pixel leg highlight starting at
`[38,49]` in idle North from the sandal strap at `[38,52]`. East-facing sandals
retain their darker coral edge as well as the bright strap. Swimsuit fabric at
idle South `[38,42]` stays distinct from the chest, midriff and arm shading.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East | 85 |
| Idle North | 65 |
| Idle South | 100 |
| Walk East | 85, 91, 85, 77 |
| Walk North | 65, 56, 65, 56 |
| Walk South | 100, 91, 100, 91 |

- [Five-choice summary](../../generated/reina-beach-pilot-preview/summary.png):
  idle North/South and second-frame walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-beach-pilot-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_BEACH_PILOT_PRESETS` supports
ignored control copies. One removes the leg-highlight seed `[38,49]` from
idle North. Another adds an ignored green alias and selects swimsuit fabric
`[38,42]` in idle South. Production palette definitions keep their existing
colors and mappings.

Both controls failed at the intended material assertion after successful
generation. The production focused test and targeted Clippy passed. Logs are
`tmp/reina-beach-pilot-author-focused.log` and
`tmp/reina-beach-pilot-author-{omission,spill}.log`.

All 1,008 final variants passed exact recipe validation. All 246 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,460 earlier
original and variant PNG/metadata files match the accepted Winter baseline
byte for byte; all 48 new variant files match the visually inspected candidate.
The six raw sidecars match the independent archive read, and the canonical
Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-beach-pilot-author-compare.json`,
`tmp/reina-beach-pilot-author-final-vs-candidate.json` and
`tmp/reina-beach-pilot-author-frozen.json`.

The complete preview is 284,406 bytes, about 278 KiB. Crop `[31,27,18,29]`
contains every opaque source pixel, including the extended hands and walking
feet. Saved-image checks compared 3,233,790 exact rendered pixels across
all-target evidence, complete review sheets and 20 summary bindings, including
source/palette identity and exact West reversal. Chromium checked all five
HTML pages, all 20 cases, image decodes and local links, with no horizontal
overflow. Reports are `tmp/reina-beach-pilot-author-preview.log`,
`tmp/reina-beach-pilot-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-beach-pilot.md) for native selection
and isolated installation.

Artwork remains ignored; this slice changes no runtime behavior. Static images
do not establish animation timing, scheduling or transitions. Other Beach
actions remain for later batches.
