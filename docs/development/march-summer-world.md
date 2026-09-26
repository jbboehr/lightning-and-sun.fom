# March Summer idle and walking sprite masks

Adds six regular Summer strips: idle and walk North, South and East. They contain 15 frames, including nine distinct images. Native West views mirror the five East frames, giving 20 review cases. March now has 240 portrait/world sources and 960 recolored variants.

## Source and material boundaries

Every source PNG is pinned to its exact SHA-256 from the read-only archive. These strips use 80×80 frames, the `Default` atlas and `Middle`/54 offset. Idle has one frame with default timing; walk has four frames at 0.15 seconds. The native animator shares the East pack with West and mirrors it horizontally.

The masks use the existing world skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 468 explicit component seeds selecting 728 pixels per target. No colors, groups or target mappings change. All 234 previous region objects and source pins remain exact.

Every actual source frame was inspected in Vanilla and all four targets. The exposed upper arms, fingers, moving hand contours, neck and small leg patches follow the selected palette. The brown apron and shoulder straps (`#402A24`, `#62473B`, `#94715E`), pale collar, hair, shorts and shoes stay original. The review crop `[28,24,24,34]` includes walking artwork through y55 and remains symmetric for West mirroring.

## Verification

The focused test in `tests/march_summer_world.rs` uses 33 literal source-art landmarks across all six strips. It checks all four choices, exact source-or-target pixels, alpha, dimensions, raw metadata and strict source hashes. All 2,340 earlier original/variant PNG and metadata files are compared with `generated/characters-march-spring-injured-trial/characters/march`.

Temporary negative controls omit the deep fingertip at idle East `[35,47]` and recolor the adjacent brown shoulder strap `[38,42]`. Both failed at the independent material landmarks before the accepted candidate passed. Formatting, targeted Clippy and the normal data test passed. All 960 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `7d0181818049e4166bedf614bcd8dbd5bf67eff8396a6688d9ae44049beb4777`.

```sh
nix-shell --pure --run 'cargo test --test march_summer_world'
nix-shell --pure --run 'cargo test --test march_summer_world -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_summer_world -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-summer-world-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-summer-world-trial
```

Local previews:

- [Five-choice summary](../../generated/march-summer-world-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-summer-world-preview/blue-review/index.html): every source frame and native West mirror.

Preview checks cover source/palette bindings, complete crop coverage, raw metadata and every displayed pixel, including mirrored views. The exact checker passed 3,264,000 gallery pixels and 326,400 summary pixels. Chromium passed all four pages, 21 images and their links. Game images and generated artifacts remain ignored and uncommitted.
