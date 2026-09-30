# Valen's Summer idle and walking sprites

The user approved the offline artwork on 2026-09-29.

Six strips begin Valen's Summer folder: idle and walk in North, South and
East. They contain 15 source frames plus five native West mirrors, giving
20 review cases. Summer coverage is 6/34 strips; the world profile grows
from 132 to 138 sources and 552 variants. All earlier regions, pins, 11
source roles, six color groups and target mappings remain exact. The preset
set and canonical Debug Blue recipe are unchanged.

Fresh sources are named `spr_npc_valen_summer_{idle,walk}_{north,south,east}.png`
under `assets/animations/NPCs/Valen/Sprites/Summer/`. The six raw metadata
sidecars match independent archive reads: 80×80 frames, Default atlas and
Middle/54 origins. Idle keeps its single-frame defaults; walking keeps
four frames at 0.15 seconds. The read-only `tmp/fields-of-mistria/assets.zip`
retains SHA-256 `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Metadata and folder inventory are in
`tmp/valen-summer-world-author-{metadata,folder}.json`.

The 442 new component seeds select 738 skin pixels per target. Arms, hands,
ankles and sandal skin change in every direction; South and East faces and
neckline openings change too. The pink shirt (`#CD8D8B`, `#AB615F`, `#542F3A`),
tan trousers (`#E3DACA`, `#AFA190`) and belt (`#72665E`) stay original.
Blue wristbands and sandal straps use `#6D70AF`, `#AEB0DF` and `#6264A0`,
distinct from the adjacent fingers, ankles and toes. Goggles, eyes and hair
also remain original. No new source role or material-edge compromise is
needed. Every actual frame was inspected in Vanilla and all four targets.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle North / South / East | 27 / 68 / 57 |
| Walk North | 27, 22, 27, 21 |
| Walk South | 68, 62, 68, 63 |
| Walk East | 57, 59, 57, 55 |

- [Five-choice summary](../../generated/valen-summer-world-preview/summary.png)
  samples idle South, walk East frame 2 and walk North frame 4.
- [Complete Vanilla/Blue review](../../generated/valen-summer-world-preview/blue-review/index.html)
  contains every source frame and native West mirror, with enlarged details.
- [Shared batch review](../../generated/balor-valen-summer-eiland-spring-finish-preview/index.html)
  links to this batch's Balor, Valen and Eiland reviews.

The complete gallery contains eight sheets across ten HTML pages, totaling
405,016 bytes; its largest file is 57,307 bytes. Checks compare 3,862,560
exact preview pixels, 40 full-case source/palette bindings plus enlarged
details and 15 summary bindings. The full crop `[25,20,32,39]` contains every
opaque pixel, including mirrored views. Raw metadata and frame counts
match. Chromium loaded all local links and images without horizontal overflow.

The new material test checks 72 literal landmarks, every pixel in four
targets, per-frame skin counts, alpha and metadata. Four effective omission
controls remove an exposed upper arm, hand, heel below a sandal strap and
finger outline. A spill control maps the pink shirt and proves that two
protected shirt pixels change. The focused test, six retained corpus suites
and targeted Clippy pass. Retained tests change only corpus paths and total count.

All 552 variants pass strict validation. All 1,320 earlier original/variant
PNG and metadata files remain byte-identical; all 1,104 final variant files
match the inspected candidates. Evidence is under
`tmp/valen-summer-world-author-` with `focused.log`, `retained.log`,
`preservation.log`, `preview.log`, `preview-browser-check.json` and
`frozen-inputs.sha256`. The complete corpus is
`extracted/valen-summer-world-study`; artwork stays ignored.

See [shared integration](balor-valen-summer-eiland-spring-finish.md) for
native frame/mirror probes and isolated installation. Static previews do
not establish live scheduling, timing or transitions. This slice changes
no runtime code; the other 28 Summer strips remain for later batches.
