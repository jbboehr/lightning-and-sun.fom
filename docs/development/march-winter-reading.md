# March Winter seated reading

Adds the three Winter `read_sit` phases: start, loop and end, all facing South. They contain 3, 4 and 3 frames respectively, with eight distinct images across the ten source frames. March now has 350 total sources and 1,400 recolored variants. No West views are added for this South-only cycle.

[Five-choice summary](../../generated/march-winter-reading-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-reading-preview/blue-review/index.html)

Fresh source hashes pin every PNG. All strips use 80×80 frames on `Default`, horizontal origin `Middle` and vertical origin `54.0`. Raw sidecars remain exact: start/end duration is `0.1`, and the loop durations are `[3.0, 0.1, 3.0, 0.1]`. Recoloring changes no native animation behavior or seated state.

Every actual frame was inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. Exposed wrists in the opening and closing poses, face shading and neck skin follow the palette. The gloves, blue sleeves, apron, moving pages, book cover and binding remain original. The open book hides the hands during the reading loop; visible dark clothing above its edge is preserved.

The three masks add 162 explicit component seeds selecting 284 pixels per target, using four existing skin shades. No source color, group or target mapping changes. All 347 earlier region objects, pins and seeds remain exact, as do all 3,470 earlier original/variant PNG and metadata files compared with `generated/characters-reina-juniper-march-winter-standard-trial/characters/march`.

`tests/march_winter_reading.rs` checks 39 literal skin/material landmarks, every new source-or-target pixel, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. Temporary controls failed at the intended assertions: omitting the deep wrist at start frame index 0 `[44,45]`, and recoloring paper at start frame index 2 `[38,42]`. The final corpus check, normal test, targeted Clippy and formatting passed. All 1,400 final variants strictly validate and match the reviewed candidate bytes. Selection SHA-256: `20a2dffca121b38848c3f68b1d9ab7b735a02dc9075f15508061cfe8e5e7d888`.

The complete Vanilla/Debug Blue gallery covers all ten source frames across three compact detail pages. Crop `[28,24,24,34]` includes all source artwork (bounds `[29,27,50,51]`). Exact checks verify every displayed pixel, source/palette/frame binding, file hash and raw sidecar: 1,632,000 gallery pixels plus 326,400 summary pixels. Chromium passed all four pages, eleven decoded images and their links. These static checks do not claim an interactive game playtest; shared integration verifies native playback and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_reading'
nix-shell --pure --run 'cargo test --test march_winter_reading -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_reading -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-reading-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-reading-trial
```

Local evidence under `tmp/march-winter-reading-author-*` includes raw metadata, source inventory, all-target sheets, test/control logs, frozen inputs and preservation/validation/browser reports. Source art and generated files stay ignored. Read-only archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
