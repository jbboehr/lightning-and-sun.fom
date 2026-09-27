# Reina Autumn idle and walk

Six strips add Autumn idle and walk North, South and East: 15 source frames and
five native West mirrors, for 20 review cases. Reina's world profile grows from
176 to 182 sources and produces 728 variants. The choices remain Vanilla,
Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork for commit
on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_autumn_{idle,walk}_{north,south,east}.png`. The complete local
corpus is `extracted/reina-autumn-world-study`; raw sidecars are recorded in
`tmp/reina-autumn-world-author-metadata.json`. All six retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Idle strips keep their one-frame
defaults; walking uses four frames at 0.15 seconds. Native West mirrors East.
The archive was exported afresh with strict source hashes; no old source pin
was refreshed or relaxed.

The 288 new component seeds select 570 skin pixels per target, covering face,
neckline, the small exposed nape and hands. Hair, eyes, checkered shirt,
trousers, gold trim, boots and outlines keep their original colors. No source
colors, color groups or target mappings changed.

Boot soles reuse the skin midtone `#8E4538` in walking North and South, frames
two and four. The eight sole pixels at strip coordinates `[117,51]`,
`[118,51]`, `[281,51]` and `[282,51]` in each strip are deliberately excluded.
This is a material distinction, not an omitted skin component. Every actual
frame was inspected in Vanilla and all four targets, using source grids and
ignored all-target contact sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East | 46 |
| Idle North | 17 |
| Idle South | 54 |
| Walk East | 46, 48, 46, 45 |
| Walk North | 17, 13, 17, 13 |
| Walk South | 54, 50, 54, 50 |

- [Five-choice summary](../../generated/reina-autumn-world-preview/summary.png):
  idle North/South and second-frame walk East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-world-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The focused local corpus test uses 70 literal skin/material landmarks and
checks every new source pixel against the inspected material classes, exact
per-frame counts, alpha, sidecars, common masks and all previous outputs.
`FOM_REINA_AUTUMN_WORLD_PRESETS` supports ignored control recipes. The omission
control removes the one-pixel nape at idle North `[39,40]`; the spill control
selects the boot sole at walk North `[117,51]`. Both failed their intended
material assertion after successful full-corpus generation, without changing
production recipes. The production test and targeted Clippy passed; logs are
`tmp/reina-autumn-world-author-focused.log` and
`tmp/reina-autumn-world-author-{omission,spill}.log`.

All 728 final variants passed exact recipe validation. All 176 earlier region
objects and all groups/mappings remain unchanged, and all 1,760 previous
original/variant PNG and metadata files match the accepted March Summer
injured baseline byte for byte. All 48 new variant PNG/metadata files match
the visually inspected candidate. The six raw sidecars match the independent
archive inventory. The canonical Debug Blue recipe matches the preset set.
Evidence is in `tmp/reina-autumn-world-author-compare.json`,
`tmp/reina-autumn-world-author-final-vs-candidate.json` and
`tmp/reina-autumn-world-author-frozen.json`.

The complete preview is 287,100 bytes, about 280 KiB. Crop `[31,27,18,29]`
contains every opaque pixel. Saved-image checks compared 3,233,790 exact
rendered pixels across all-target evidence sheets, full review sheets and 20
summary bindings, including source/palette identity and exact West reversal.
Chromium checked all five HTML pages, every case, image decode and local link,
with no horizontal overflow. Reports are
`tmp/reina-autumn-world-author-preview-check.log`,
`tmp/reina-autumn-world-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-autumn.md) for native selection
and isolated installation.
Static images do not establish animation timing, scheduling or state
transitions. Artwork remains ignored; this slice changes no runtime behavior.
Further Autumn actions remain for later batches.
