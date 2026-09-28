# March Winter blink, sit, eat and drink

Adds eleven Winter strips: blink East/South and sit, eat and drink North/South/East. These contain 31 source frames, including 23 distinct images. Native East packs also render twelve West mirrors, giving 43 review cases. March now has 342 total sources and 1,368 recolored variants.

[Five-choice summary](../../generated/march-winter-actions-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-actions-preview/blue-review/index.html)

Fresh source hashes pin every PNG. All strips use 80×80 frames on `Default`, horizontal origin `Middle` and vertical origin `54.0`. Raw sidecars remain exact, including each strip's frame count and timing. Recoloring changes no native animation behavior or final-frame holds.

Every actual source frame was inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. The exposed wrists move around the black gloves during eating and drinking; their light patches and disconnected deep contours follow skin. Rear-neck pixels and face/lip-edge skin also map. True mouth interiors (`#410808` and `#9E2626`), closed eye lines, hair, headband, black gloves, blue sleeves, brown apron and footwear remain original. Separately drawn held objects are outside these source strips.

The eleven masks add 484 explicit component seeds selecting 777 pixels per target. They use the existing four skin shades plus the existing `#E8B171` midtone alias on the drinking wrist. No source color, group or target mapping changes. All 331 earlier region objects, pins and seeds remain exact, as do all 3,310 earlier original/variant PNG and metadata files compared with `generated/characters-reina-juniper-march-winter-trial/characters/march`.

`tests/march_winter_actions.rs` checks 56 literal skin/material landmarks across all eleven strips, every new source-or-target pixel, alpha, dimensions, metadata, strict source hashes and equal selections for all four targets. Temporary controls were observed failing at the intended assertions: omitting the fine drinking wrist shade at East frame 1 `[38,43]`, and selecting red mouth interior at eating East frame 2 `[42,37]`. The final candidate corpus check, targeted Clippy, formatting and normal test passed. All 1,368 final variants strictly validate and match the actual reviewed candidate bytes. Selection SHA-256: `fa224f75f06a411b50950102d2c33d69cff94555cd7e982037bc5f95aac2ae42`.

The full Vanilla/Debug Blue gallery covers all 31 source frames and twelve West mirrors across eight compact detail pages. A symmetric crop `[28,24,24,34]` includes all source artwork (bounds `[30,26,48,54]`). Exact checks verify every displayed pixel, source/palette/frame binding, file hash, raw sidecar and West mirror: 7,017,600 gallery pixels plus 326,400 summary pixels. Chromium passed all nine pages, 44 decoded images and their links. These static checks do not claim an interactive game playtest or natural West dispatch; shared integration verifies native playback and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_actions'
nix-shell --pure --run 'cargo test --test march_winter_actions -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_actions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-actions-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-actions-trial
```

Local evidence under `tmp/march-winter-actions-author-*` includes raw metadata, source inventory, all-target sheets, test/control logs, frozen inputs and preservation/validation/browser reports. Source art and generated files stay ignored. Read-only archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
