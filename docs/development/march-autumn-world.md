# March Autumn idle and walking sprite masks

Adds six regular Autumn strips: idle and walk North, South and East. They contain 15 frames, including nine distinct images. Native West views mirror the five East frames, giving 20 review cases. March now has 284 portrait/world sources and 1,136 recolored variants.

## Source and material boundaries

Every source PNG is pinned to its exact SHA-256 from the read-only archive. These strips use 80×80 frames, the `Default` atlas and `Middle`/54 offset. Idle has one frame with default timing; walk has four frames at 0.15 seconds. The native animator shares the East pack with West and mirrors it horizontally.

The masks use the existing world skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 284 explicit component seeds selecting 519 pixels per target. No colors, groups or target mappings change. All 278 previous region objects and source pins remain exact.

Every actual source frame was inspected in Vanilla and all four targets. Face shading, the exposed neckline and moving fingers follow the selected palette. The Autumn brown jacket (`#2E2220`, `#55423B`, `#83685F`), pale `#D5DBD1` shirt, dark scarf/headband and trousers remain original, as do hair, eye details and footwear. The rear neckline is covered by the jacket, and the trousers cover the ankle patches exposed in Summer. Those source differences were inspected directly. The review crop `[28,24,24,34]` includes all walking artwork through y55 and remains symmetric for West mirroring.

## Verification

The focused test in `tests/march_autumn_world.rs` uses 38 literal source-art landmarks across all six strips. It checks all four choices, exact source-or-target pixels, alpha, dimensions, raw metadata and strict source hashes. All 2,780 earlier original/variant PNG and metadata files are compared with `generated/characters-march-summer-injured-trial/characters/march`.

Temporary negative controls omit the deep fingertip at idle East `[35,47]` and recolor the adjacent brown jacket edge `[38,42]`. Both controls failed at the intended literal material landmarks. The focused candidate test, targeted Clippy, formatting and normal test all passed. All 1,136 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `869228a51f600999403ba6291d89a317b9fb35d8c8ea63f18fdf03e61780083e`.

```sh
nix-shell --pure --run 'cargo test --test march_autumn_world'
nix-shell --pure --run 'cargo test --test march_autumn_world -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_autumn_world -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-autumn-world-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-autumn-world-trial
```

Local previews:

- [Five-choice summary](../../generated/march-autumn-world-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-autumn-world-preview/blue-review/index.html): every source frame and native West mirror.

Preview checks cover source/palette bindings, complete crop coverage, raw metadata and every displayed pixel, including mirrored views. The checker passed 3,264,000 gallery pixels and 326,400 summary pixels. Chromium passed all four pages, 21 images and their links. Static images do not demonstrate game timing or natural scheduling; integration handles native runtime probes and installation separately. Game images and generated artifacts remain ignored and uncommitted.
