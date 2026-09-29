# Reina Wedding idle and walk

Six strips add Wedding idle and walking North/South/East. They contain 15 source
frames and five native West mirrors, giving 20 review cases. Reina's world
profile grows from 263 to 269 sources and produces 1,076 variants. Choices
remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. This artwork awaits user
review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Wedding/`, named
`spr_npc_reina_wedding_{idle,walk}_{north,south,east}.png`. The fresh corpus is
`extracted/reina-wedding-pilot-study`; raw sidecars are recorded in
`tmp/reina-wedding-pilot-author-metadata.json`. All six retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Idle retains single-frame defaults;
walking retains four frames at 0.15 seconds.

The updated mounted archive has SHA256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 526 earlier original PNG/metadata files are byte-identical to the accepted
Beach completion baseline. Source hashes remain strict; no earlier pin was
refreshed or relaxed. The fresh comparison is recorded in
`tmp/reina-wedding-pilot-author-fresh-source-audit.json`.

The 387 new component seeds select 635 skin pixels per target, covering faces,
arms, hands and exposed feet. All 15 actual frames were inspected in Vanilla
and all four targets. Pale dress fabric, pink trim and hair decoration, gold
accessories, hair, eyes and outlines remain original. Twenty North-facing gold
accessory-shadow pixels reuse portrait skin color `#B36644`: four per frame,
including idle. Those components stay excluded. No source colors, groups or
mappings were added or changed.

Literal landmarks distinguish the three-pixel hand highlight starting at
`[32,46]` in idle North from the gold accessory shadow at `[35,38]`. Every North
frame explicitly checks all four `#B36644` accessory pixels, including their
one-pixel vertical shift while walking. Additional landmarks preserve pale
dress/shoe colors and pink neckline trim beside the recolored skin.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East | 52 |
| Idle North | 16 |
| Idle South | 63 |
| Walk East | 52, 58, 52, 48 |
| Walk North | 16, 11, 16, 12 |
| Walk South | 63, 57, 63, 56 |

- [Five-choice summary](../../generated/reina-wedding-pilot-preview/summary.png):
  idle North/South and second-frame walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-wedding-pilot-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_WEDDING_PILOT_PRESETS` supports
ignored control copies. One removes hand-highlight seed `[32,46]` from idle
North. Another selects gold accessory shadow `[35,38]`, exercising the actual
portrait-color collision without adding a test palette alias.

Both controls failed at the intended material assertion after successful
generation: omitted hand shading at `[32,46]`, and recolored accessory shadow
at `[35,38]`. The production focused test and targeted Clippy passed. Logs are
`tmp/reina-wedding-pilot-author-focused.log` and
`tmp/reina-wedding-pilot-author-{omission,spill}.log`.

All 1,076 final variants passed exact recipe validation. All 263 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,630 earlier
original and variant PNG/metadata files match the accepted Beach completion
baseline byte for byte; all 48 new variant files match the visually inspected
candidate. The six raw sidecars match the independent archive read, and the
canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-wedding-pilot-author-compare.json`,
`tmp/reina-wedding-pilot-author-final-vs-candidate.json` and
`tmp/reina-wedding-pilot-author-frozen.json`.

The complete preview is 294,182 bytes, about 287 KiB. Crop `[31,27,18,29]`
contains every opaque source pixel, including the moving hands and feet.
Saved-image checks compared 3,233,790 exact rendered pixels across all-target
evidence, complete review sheets and 20 summary bindings, including
source/palette identity and exact West reversal. Chromium checked all five
HTML pages, all 20 cases, image decodes and local links, with no horizontal
overflow. Reports are `tmp/reina-wedding-pilot-author-preview.log`,
`tmp/reina-wedding-pilot-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-wedding-pilot.md) for native selection
and isolated installation.

Artwork remains ignored; this slice changes no runtime behavior. Static images
do not establish animation timing, scheduling or transitions. Remaining Wedding
poses are outside this pilot.
