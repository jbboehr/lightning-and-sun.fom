# Eiland's Summer general actions, sleep and kiss

5 strips add 26 source frames and 12 native West mirrors: 38 review cases.
Eiland now has 147 sources and 588 variants, covering Summer 22/41 strips.
The user approved the offline artwork on 2026-09-29.

Fresh PNGs come from `assets/animations/NPCs/Eiland/Sprites/Summer/`.
Every new region records strict source hashes and dimensions in the tracked
profile. Raw animation metadata remains unchanged. The [shared notes](balor-autumn-valen-writing-eiland-standard.md)
record archive provenance, direction coverage and native cycle timing.

The 438 new component seeds select 842 skin pixels per target.
All source frames were inspected in Vanilla and Debug Blue; five-choice
summaries cover representative poses. Existing world roles suffice, with no
new source colors or material exceptions.

Exposed faces, necks, arms and fingers change. Pink clothing, gold trim,
pale sleeves, trousers, boots, eyes and hair remain original.

- [Five-choice summary](../../generated/eiland-summer-standard-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/eiland-summer-standard-preview/blue-review/index.html)

The gallery checks all 38 frame/direction cases against raw metadata,
complete artwork crops and palette bindings. Saved-image checks compare
6,917,664 exact displayed pixels; Chromium checks 14 HTML pages, loaded images,
local links and horizontal overflow. Final variant PNGs and metadata match
the inspected candidates byte for byte.

The focused material test checks 212 literal source-grid landmarks, every pixel
in every target, per-frame skin counts, alpha, metadata and prior output.
Both negative controls fail their intended pixel assertion after successful
generation: omission at strip [39,40]; spill at strip [43,47] (#533061).
Definitions are in `tmp/bve-autumn-writing-standard-controls.json`; failure logs use
`tmp/eiland-summer-standard-author-{omission,spill}-red.log`. Nine retained corpus
tests keep their earlier material assertions, changing only corpus paths and totals.

All 588 standalone variants pass strict recipe validation. The shared comparison
checks them against the complete collection and preserves 1420 earlier original
and variant PNG/metadata files. Evidence:
`tmp/bve-autumn-writing-standard-eiland-final-output.log`, the character local
logs and shared integration report.

Static previews do not establish live scheduling or rendering. Game-derived
artwork and packages remain ignored.
