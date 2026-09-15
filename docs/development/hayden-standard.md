# Hayden's remaining normal Spring actions

This batch adds five strips to the accepted [Spring action set](hayden-actions.md):
`action_{north,south,east}`, `sleep_east` and `kiss_east`. There are 26 source
frames and 38 direction/frame cases with native West mirrors. The profile now
contains 155 sources and 620 variants. The user accepted the offline artwork.

Sources are read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`, named
`spr_npc_hayden_spring_{cycle}_{direction}.png`. The full local corpus is
`extracted/hayden-standard-study`; `tmp/hayden-standard-metadata.json` records
all raw sidecars. Frames are 80×80, Default atlas, Middle/54 origin. Action has
seven frames with durations `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four
with `[0.15,0.15,0.8,0.15]`; sleep uses the single-frame engine defaults.

All 150 earlier region objects, color groups and source/target mappings remain
unchanged. The additions use 460 component seeds, selecting 1,015 pixels per
target and excluding 298 matching shirt-shadow pixels. Hair, beard and other
non-skin colors stay original. The third kiss frame has one isolated `E8B271`
cheek pixel at `[41,35]`, mapped through the existing portrait highlight entry.
Frame numbers here are one-based and coordinates are frame-local.

All 26 frames were inspected in all four targets on
`tmp/hayden-standard-art-{cycle}-{direction}.png`. The inspected 104 target
frames match `generated/hayden-standard-trial` byte for byte. Component evidence
is in `tmp/hayden-standard-refinement.json`; frozen input hashes are in
`tmp/hayden-standard-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-standard-preview/summary.png):
  four action/kiss/sleep samples, about 58 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-standard-preview/blue-review/index.html):
  all 26 source frames and 12 West mirrors, on five pages with at most eight
  cases each; preview directory about 856 KB.

The crop `[28,23,25,34]` contains every visible pixel. Exact checks cover
6,460,000 full-review pixels, 612,000 summary pixels, all 20 sample bindings,
metadata and West reversal. Chromium decoded every image and checked every
page, case and link. Logs: `tmp/hayden-standard-preview-pixel-check.log` and
`tmp/hayden-standard-preview-browser-check.json`.

The opt-in `hayden_standard` corpus test passes all four targets, checking 22
literal skin/material boundaries including the kissing cheek, shared masks,
alpha, metadata and nonempty frame coverage. Focused Clippy also passes. Exact
comparison preserves 1,500 prior original/variant PNG and metadata files from
`generated/characters-spring-actions-trial/characters/hayden`. Evidence:
`tmp/hayden-standard-focused-test.log`, `tmp/hayden-standard-clippy.log` and
`tmp/hayden-standard-comparison.log`.

See [the combined integration](spring-world-standard.md) for full checks,
native animation probes and isolated installation. Static previews do not
exercise live game timing, schedules or state/outfit changes. Game files and
review artwork remain ignored; this slice changes no Rust or GML runtime code.
