# Reina Summer idle and walk

Six strips add Summer idle and walk North, South and East: 15 source frames and
five native West mirrors, for 20 review cases. Reina's world profile grows from
141 to 147 sources and produces 588 variants. The choices remain Vanilla,
Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork for commit on 2026-09-26.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_summer_{idle,walk}_{north,south,east}.png`. The complete local
corpus is `extracted/reina-summer-world-study`; raw sidecars are recorded in
`tmp/reina-summer-world-author-metadata.json`. All six match the independent
archive read and retain 80×80 frames, Default atlas and `Middle`/54 offset
(numeric origin `[40.0,54.0]`). Idle strips retain their one-frame defaults; walk strips retain
four frames at 0.15 seconds. Native West uses the East artwork mirrored.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No earlier source pin was refreshed or relaxed.

All four existing world skin shades belong to exposed skin in these six strips;
no source colors, groups or target mappings were added or changed. The 552 new
component seeds select 1,047 skin pixels per target, covering face, neckline,
shoulders, arms, hands, exposed back, legs and toes. Hair, eyes, cream and tan
blouse pixels, green fabric and straps, the gold button, pink hair accessory,
sandal straps and outlines keep their original colors. In particular,
walk-East frame two has an isolated exposed shoulder highlight at strip
coordinate `[117,43]`; idle-East's dark green strap at `[38,41]` stays original.
Every actual source frame was inspected in Vanilla and all four target palettes
using source material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East | 75 |
| Idle North | 55 |
| Idle South | 86 |
| Walk East | 75, 76, 75, 69 |
| Walk North | 55, 46, 55, 46 |
| Walk South | 86, 81, 86, 81 |

- [Five-choice summary](../../generated/reina-summer-world-preview/summary.png):
  idle North/South and second-frame walk East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-summer-world-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The complete preview directory is about 279 KiB. Previews bind to the final
standalone bundle. Crop `[31,27,18,29]` includes every opaque source pixel.
Saved-image checks cover 3,233,790 exact pixels across all-target sheets,
complete review sheets and the 20 summary bindings, including source/palette
identity and exact West reversal. Chromium decoded all images across five HTML
pages, checked every case and local link, and found no horizontal overflow.
Evidence is in `tmp/reina-summer-world-author-preview-check.log`,
`tmp/reina-summer-world-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against the independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control failed at the exposed
shoulder component `[117,43]` in walk East. Because no Summer clothing reuses a
skin shade, the spill control deliberately adds an ignored green-fabric alias
and seed; it fails at idle-East's strap `[38,41]`. Both controls reached the
intended material assertion after successful generation; neither altered the
production recipes. `FOM_REINA_SUMMER_WORLD_PRESETS` supports these ignored
control copies. Logs are `tmp/reina-summer-world-author-focused.log` and
`tmp/reina-summer-world-author-{omission,spill}.log`.

All 588 final variants passed exact recipe validation. All 141 earlier region
objects, groups and mappings remain unchanged, and all 1,410 previous
original/variant PNG and metadata files match the accepted March-injured batch's
Reina baseline byte for byte. All 48 new variant PNG/metadata files exactly match
the visually inspected candidate. Final sidecars also match the independent raw
inventory. The canonical Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-world-author-compare.json` and
`tmp/reina-summer-world-author-final-vs-candidate.json`; data and test hashes are
in `tmp/reina-summer-world-author-frozen.json`.

See [shared integration](reina-juniper-march-summer.md) for native selection
and isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further Summer actions remain for later batches.
