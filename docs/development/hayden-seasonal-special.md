# Hayden's Summer special animations

Eight strips add seated reading (start/loop/end South), hammer, harvest,
till and water East, and wipebrow South. The 41 source frames produce 66
review cases including 25 native West mirrors. The profile now covers 216
sources and 864 variants. All 208 accepted region objects, groups, mappings
and 2,080 prior original/variant PNG and metadata files are unchanged.
The user accepted this offline artwork on 2026-09-16.

The read-only source is `tmp/momi-lab/assets.bak.zip`, under Hayden's Summer
sprite directory with the `spr_npc_hayden_specialanimation_summer_` prefix.
The full corpus is `extracted/hayden-seasonal-special-study`; raw sidecars
are in `tmp/hayden-seasonal-special-metadata.json`. Every frame retains
80×80 geometry, Default atlas and Middle/54 origin. Native cycles retain:

| Cycle | Source frames | Durations (seconds) |
| --- | ---: | --- |
| `read_sit` start / loop / end | 3 / 4 / 3 | 0.1 / `[3,0.1,3,0.1]` / 0.1 |
| `hammer` | 6 | `[0.1,0.2,0.1,0.1,0.1,0.1]` |
| `harvest` | 10 | `[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]` |
| `till` | 5 | `[0.125,0.125,0.125,0.125,1.2]` |
| `water` | 4 | `[0.15,1,0.15,2.5]` |
| `wipebrow` | 6 | `[0.15,0.15,0.45,0.1,0.075,1]` |

The 682 reviewed component seeds select 1,744 skin pixels per target and
exclude all 18 matching `#523C26` shirt-seam pixels. No new shade or mapping
was needed. Reviewed boundaries include the raised hammer hand, harvesting
forearms down to zero-based row 53, fingers beside the book, and the hand
crossing the face during wipebrow. Tools, swing effects, book pages/covers,
clothing, straw hat, hair and beard remain original.

All 41 frames were inspected in all four targets on the ignored
`tmp/hayden-seasonal-special-art-*.png` sheets. All 64 reviewed target
PNG/metadata files equal the standalone package. Component decisions and
frozen hashes are recorded in `tmp/hayden-seasonal-special-refinement.json`
and `tmp/hayden-seasonal-special-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-seasonal-special-preview/summary.png):
  reading, hammer, harvest and water samples.
- [Complete Vanilla/Blue review](../../generated/hayden-seasonal-special-preview/blue-review/index.html):
  66 cases, 132 views, nine pages of at most eight cases.

Validation covers full crop `[27,20,38,37]`, 18,559,200 exact review pixels,
449,920 summary pixels, 20 sample bindings, unchanged metadata and West
reversal. Chromium decoded every image and checked every case and link;
evidence is `tmp/hayden-seasonal-special-preview-browser-check.json` and
the review's `coverage.json`. The focused corpus test has 44 literal
material landmarks, full pixel/mask checks, per-frame coverage and prior
output comparisons; its test and targeted Clippy results are recorded in
`tmp/hayden-seasonal-special-test.log`.

See [combined integration](world-seasonal-special.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this character
slice changes no runtime code.
