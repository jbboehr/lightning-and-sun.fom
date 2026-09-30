# Balor's Autumn idle and walking

6 strips add 15 source frames and 5 native West mirrors: 20 review cases.
Balor now has 179 sources and 716 variants, covering Autumn 6/28 strips.
The user approved the offline artwork on 2026-09-29.

Fresh PNGs come from `assets/animations/NPCs/Balor/Sprites/Autumn/`.
Every new region records strict source hashes and dimensions in the tracked
profile. Raw animation metadata remains unchanged. The [shared notes](balor-autumn-valen-writing-eiland-standard.md)
record archive provenance, direction coverage and native cycle timing.

The 258 new component seeds select 441 skin pixels per target.
All source frames were inspected in Vanilla and Debug Blue; five-choice
summaries cover representative poses. Existing world roles suffice, with no
new source colors or material exceptions.

Exposed faces, necks and hands change. The burgundy scarf, shirt, belt,
trousers, boots, eyes and hair remain original.

- [Five-choice summary](../../generated/balor-autumn-world-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/balor-autumn-world-preview/blue-review/index.html)

The gallery checks all 20 frame/direction cases against raw metadata,
complete artwork crops and palette bindings. Saved-image checks compare
3,862,560 exact displayed pixels; Chromium checks 10 HTML pages, loaded images,
local links and horizontal overflow. Final variant PNGs and metadata match
the inspected candidates byte for byte.

The focused material test checks 143 literal source-grid landmarks, every pixel
in every target, per-frame skin counts, alpha, metadata and prior output.
Both negative controls fail their intended pixel assertion after successful
generation: omission at strip [34,44]; spill at strip [39,50] (#262C49).
Definitions are in `tmp/bve-autumn-writing-standard-controls.json`; failure logs use
`tmp/balor-autumn-world-author-{omission,spill}-red.log`. Nine retained corpus
tests keep their earlier material assertions, changing only corpus paths and totals.

All 716 standalone variants pass strict recipe validation. The shared comparison
checks them against the complete collection and preserves 1730 earlier original
and variant PNG/metadata files. Evidence:
`tmp/bve-autumn-writing-standard-balor-final-output.log`, the character local
logs and shared integration report.

Static previews do not establish live scheduling or rendering. Game-derived
artwork and packages remain ignored.
