# Valen's Summer reading and writing

9 strips add 26 source frames and 0 native West mirrors: 26 review cases.
Valen now has 163 sources and 652 variants, covering Summer 31/34 strips.
The user approved the offline artwork on 2026-09-29.

Fresh PNGs come from `assets/animations/NPCs/Valen/Sprites/Summer/`.
Every new region records strict source hashes and dimensions in the tracked
profile. Raw animation metadata remains unchanged. The [shared notes](balor-autumn-valen-writing-eiland-standard.md)
record archive provenance, direction coverage and native cycle timing.

The 723 new component seeds select 1,384 skin pixels per target.
All source frames were inspected in Vanilla and Debug Blue; five-choice
summaries cover representative poses. Existing world roles suffice, with no
new source colors or material exceptions.

Exposed faces, necks, arms, fingers and sandal skin change. The pink reading
book, blue writing book, pages, pen, clothing, sandal straps, goggles, eyes
and hair remain original.

- [Five-choice summary](../../generated/valen-summer-writing-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/valen-summer-writing-preview/blue-review/index.html)

The gallery checks all 26 frame/direction cases against raw metadata,
complete artwork crops and palette bindings. Saved-image checks compare
4,880,928 exact displayed pixels; Chromium checks 11 HTML pages, loaded images,
local links and horizontal overflow. Final variant PNGs and metadata match
the inspected candidates byte for byte.

The focused material test checks 308 literal source-grid landmarks, every pixel
in every target, per-frame skin counts, alpha, metadata and prior output.
Both negative controls fail their intended pixel assertion after successful
generation: omission at strip [37,40]; spill at strip [42,49] (#6264A0).
Definitions are in `tmp/bve-autumn-writing-standard-controls.json`; failure logs use
`tmp/valen-summer-writing-author-{omission,spill}-red.log`. Nine retained corpus
tests keep their earlier material assertions, changing only corpus paths and totals.

All 652 standalone variants pass strict recipe validation. The shared comparison
checks them against the complete collection and preserves 1540 earlier original
and variant PNG/metadata files. Evidence:
`tmp/bve-autumn-writing-standard-valen-final-output.log`, the character local
logs and shared integration report.

Static previews do not establish live scheduling or rendering. Game-derived
artwork and packages remain ignored.
