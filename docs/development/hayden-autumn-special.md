# Hayden's Autumn special animations

Eight strips complete Hayden's Autumn sprites: seated reading start/loop/end
South, hammer, harvest, till and water East, and wipebrow South. The 41 source
frames produce 66 review cases with 25 native West mirrors. The profile now
covers 246 sources and 984 variants. All 238 accepted regions, color groups
and mappings remain unchanged. This slice awaits the user's artwork review.

The source is the updated, read-only `tmp/fields-of-mistria/assets.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Autumn/` with the
`spr_npc_hayden_specialanimation_autumn_` prefix. The full corpus is
`extracted/hayden-autumn-special-study`; raw sidecars are recorded in
`tmp/hayden-autumn-special-metadata.json`. The retained accepted build's
2,380 original/variant PNG and metadata files still match byte for byte.
No existing source hash was refreshed to accommodate the update.

Every new frame retains 80×80 geometry, Default atlas and Middle/54 origin.
The sidecars retain these durations:

| Cycle | Source frames | Durations (seconds) |
| --- | ---: | --- |
| `read_sit` start / loop / end | 3 / 4 / 3 | 0.1 / `[3,0.1,3,0.1]` / 0.1 |
| `hammer` | 6 | `[0.1,0.2,0.1,0.1,0.1,0.1]` |
| `harvest` | 10 | `[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]` |
| `till` | 5 | `[0.125,0.125,0.125,0.125,1.2]` |
| `water` | 4 | `[0.15,1,0.15,2.5]` |
| `wipebrow` | 6 | `[0.15,0.15,0.45,0.1,0.075,1]` |

The 608 reviewed component seeds select 1,507 skin pixels per target. Summer
masks helped locate corresponding components, but Autumn sleeves change the
hand boundaries. The detached `#AB7E3F` finger at `[31,44]` in zero-based
reading start frame 0 and end frame 2, plus the `#E7B172` forearm highlight at
`[40,44]` in water frames 0 and 2, require additional selections. Purple sleeve
pixels where Summer had exposed arms stay original. Hair, beard, clothing,
book pages/covers, tool handles/heads and swing particles stay original. No
new source color or palette mapping was needed.

All 41 source frames were inspected in Vanilla and all four targets on the
ignored `tmp/hayden-autumn-special-art-*.png` sheets. The final sheets and
previews use the actual standalone bundle, with exact source/palette bindings.

- [Five-choice summary](../../generated/hayden-autumn-special-preview/summary.png):
  reading, hammer, harvest and water samples.
- [Complete Vanilla/Blue review](../../generated/hayden-autumn-special-preview/blue-review/index.html):
  66 cases and 132 views across nine pages of at most eight cases.

The full crop `[29,22,48,42]` includes the hammer's far-right swing particles.
Saved-image checks cover 37,588,320 exact pixels across all-target art sheets,
complete review sheets and the 20 summary bindings, including West reversal
and no clipped opaque pixels. Chromium decoded every image across all 11
HTML pages, verified every case and local link, and found no horizontal
overflow. Evidence is in `tmp/hayden-autumn-special-preview-check.log`,
`tmp/hayden-autumn-special-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
47 literal material boundaries, full pixels, per-frame coverage, alpha,
metadata and all accepted outputs. All 984 variants also passed exact recipe
validation. Results are recorded in `tmp/hayden-autumn-special-test.log` and
`tmp/hayden-autumn-special-compare.json`.

See [combined integration](world-autumn-special.md) for shared checks,
current-game compatibility and installation. Static images do not establish
live animation timing, natural NPC scheduling or state transitions. Artwork
remains ignored; this character slice changes no runtime code.
