# March Winter action, sleep and kiss

Adds five Winter strips: general action North/South/East, sleep East and kiss East. They contain 26 source frames, including 19 distinct images. Native East packs can also render twelve West mirrors, giving 38 review cases. March now has 347 total sources and 1,388 recolored variants.

[Five-choice summary](../../generated/march-winter-standard-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-standard-preview/blue-review/index.html)

Fresh source hashes pin every PNG. All five strips use 80×80 frames on `Default`, horizontal origin `Middle` and vertical origin `54.0`. Raw metadata remains exact, including the seven-frame action timing, four-frame kiss timing and single-frame sleep defaults. Recoloring changes no native animation behavior or final-frame holds.

Every actual frame was inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. The moving exposed wrists, rear neck, face and kissing lip-edge skin follow the palette. Black gloves, blue sleeves, brown apron, hair, headband, closed eye lines and footwear remain original. The sleeping pose keeps its raised glove dark while recoloring the small exposed wrist beneath it. Separately drawn held objects are outside these source strips.

The five masks add 380 explicit component seeds selecting 645 pixels per target, using four existing skin shades. No source color, group or target mapping changes. All 342 earlier region objects, pins and seeds remain exact, as do all 3,420 earlier original/variant PNG and metadata files compared with `generated/characters-reina-juniper-march-winter-actions-trial/characters/march`.

`tests/march_winter_standard.rs` checks 47 literal skin/material landmarks, every new source-or-target pixel, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. Temporary controls failed at the intended assertions: omitting the exposed wrist at action East frame index 1 `[46,43]`, and selecting black glove material at `[48,45]` in that same frame. The final candidate corpus check, normal test, targeted Clippy and formatting passed. All 1,388 final variants strictly validate and match the actual reviewed candidate bytes. Selection SHA-256: `0b518cd88e7d17d2d7c66ad17a4a23b7f546c9fb8aef0e709ef9267ab1e42bc7`.

The complete Vanilla/Debug Blue gallery covers 26 source frames and twelve West mirrors across six compact detail pages. Its symmetric crop `[28,24,24,34]` includes all source artwork (bounds `[31,26,50,54]`). Exact checks verify every displayed pixel, source/palette/frame binding, file hash, raw sidecar and West mirror: 6,201,600 gallery pixels plus 326,400 summary pixels. Chromium passed all seven pages, 39 decoded images and their links. These static checks do not claim an interactive game playtest or natural West dispatch; shared integration verifies native playback and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_standard'
nix-shell --pure --run 'cargo test --test march_winter_standard -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_standard -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-standard-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-standard-trial
```

Local evidence under `tmp/march-winter-standard-author-*` includes raw metadata, source inventory, all-target sheets, test/control logs, frozen inputs and preservation/validation/browser reports. Source art and generated files stay ignored. Read-only archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
