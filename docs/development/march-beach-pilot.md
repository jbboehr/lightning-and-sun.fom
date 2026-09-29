# March Beach idle and walking sprite masks

Adds six Beach strips: idle and walk North, South and East. They contain 15 frames, including nine distinct images. Native West views mirror the five East frames, giving 20 review cases. March now has 375 portrait/world sources and 1,500 recolored variants.

[Five-choice summary](../../generated/march-beach-pilot-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-beach-pilot-preview/blue-review/index.html)

Every source PNG is pinned to its exact SHA-256 from the read-only archive. These strips use 80×80 frames, the `Default` atlas and `Middle`/54 offset. Idle has one frame with default timing; walk has four frames at 0.15 seconds. The native animator shares the East pack with West and mirrors it horizontally.

The masks use the existing skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 581 explicit component seeds selecting 1,276 pixels per target. No colors, groups or target mappings change. All 369 previous region objects and source pins remain exact.

Every actual source frame was inspected in Vanilla and all four targets. The bare torso, shoulders, palms, deep fingertip contours, legs and toes follow the selected palette. Green swim shorts (`#346232`, `#769E5A`, `#CAF0B5`) and burgundy sandal straps (`#612934`, `#8B273B`) stay original, including the straps crossing the lifted feet. Hair, actual eye details and black outlines also remain original. The review crop `[28,24,24,34]` includes all artwork through y55 and stays symmetric for West mirroring.

The focused test in `tests/march_beach_pilot.rs` uses 43 literal source-art landmarks across all six strips. It checks all four choices, exact source-or-target pixels, per-frame skin counts, alpha, dimensions, raw metadata and strict source hashes. All 3,690 earlier original/variant PNG and metadata files are compared with `generated/characters-march-winter-injured-trial/characters/march`.

Temporary negative controls omit the deep fingertip at idle East `[35,47]` and recolor the burgundy sandal strap at `[38,52]`. Both failed at the intended independent material landmarks before the accepted candidate passed. Formatting, targeted Clippy and the normal data test passed. All 1,500 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `153f45866cd5889466c9a63d7e30c8a42bc60368c2c13f2f115a3001caa0a5aa`.

Preview checks cover every source/palette/frame binding, complete crop coverage, raw metadata and displayed pixels, including all mirrored views. The exact checker passed 3,264,000 gallery pixels and 326,400 summary pixels. Chromium passed all four HTML pages, 21 images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native behavior and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_beach_pilot'
nix-shell --pure --run 'cargo test --test march_beach_pilot -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_beach_pilot -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-beach-pilot-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-beach-pilot-trial
```

Local evidence is under `tmp/march-beach-pilot-author-*`: raw metadata, source inventory, all-target sheets, control/test logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored. Archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
