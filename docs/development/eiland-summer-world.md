# Eiland's Summer idle and walking sprites

Six strips begin Eiland's Summer folder: idle and walk North/South/East.
They contain 15 source frames plus five native West mirrors, giving 20 review
cases. Eiland now has 131 pinned sources and 524 variants, with Summer 6/41
covered. The user approved the offline artwork on 2026-09-29.

Fresh files are named `spr_npc_eiland_summer_{idle,walk}_{direction}.png` under
`assets/animations/NPCs/Eiland/Sprites/Summer/`. Raw sidecars match independent
archive reads: 80×80 frames, Default atlas, Middle/54 origins. Idle uses
single-frame defaults; walking uses four frames at 0.15 seconds. Strict pins
come from archive SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 287 new component seeds select 505 skin pixels per target. All 15 actual
frames were inspected in Vanilla and all four targets. The existing five world
skin shades cover faces, necks, exposed forearms and fingers. Summer gold trim
uses distinct `#DA8B36`/`#F9C94D` shades; pink clothing, pale sleeves, trousers,
boots, eyes and hair remain original. No new roles or material exceptions are
needed. All 125 prior region objects and source-color/group/target definitions
remain unchanged.

- [Five-choice summary](../../generated/eiland-summer-world-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/eiland-summer-world-preview/blue-review/index.html)

The complete preview is 403,964 bytes. It covers 20 cases, 40 full-case bindings
plus enlarged details and 15 summary bindings. Checks compare 3,862,560 exact
pixels against source metadata, frames and mirrors. The full crop
`[25,20,32,39]` includes all artwork. Chromium checks all ten pages, images,
local links and overflow.

The focused test checks 66 literal source-grid landmarks, every source/target
pixel, per-frame counts, alpha, metadata, strict hashes and equal selections
across targets. Removing the finger-outline seed at idle East `[35,47]` fails
the skin assertion. Deliberately mapping gold trim and selecting `[39,42]`
fails the protected-material assertion after successful generation. The seven
retained tests change only corpus paths and counts.

All 524 standalone variants strictly validate and match the inspected candidates.
All 1,250 prior original/variant PNG and metadata files remain exact. Evidence:
`tmp/eiland-summer-world-author-{audit.json,final-output.log,omission-red.log,spill-red.log}`,
`tmp/bve-summer-expansion-new-local.log` and
`tmp/bve-summer-expansion-eiland-retained.log`.

See [shared integration](balor-valen-eiland-summer-expansion.md) for complete
verification and native/installation checks. Static images do not establish
live timing or scheduling. Game-derived artwork remains ignored.
