# March Beach blink, action and kiss masks

Adds six Beach strips: blink East/South, general action North/South/East and kiss East. They contain 31 frames, including 22 distinct images. Native East packs also supply fourteen West mirror views, giving 45 review cases. March now has 381 portrait/world sources and 1,524 recolored variants.

[Five-choice summary](../../generated/march-beach-actions-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-beach-actions-preview/blue-review/index.html)

Every source PNG is pinned to its fresh SHA-256 from the read-only archive. These strips use 80×80 frames, the `Default` atlas and `Middle`/54 offset. Blink uses three durations `[0.075,0.125,0.075]`; the seven-frame general actions retain `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss uses `[0.15,0.15,0.8,0.15]`. Native general actions retain the `[240,360]` final-frame hold. West review views use the native East pack and horizontal mirror; they do not imply natural scheduling in that direction. Recoloring changes no animation behavior.

The masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 1,162 explicit component seeds selecting 2,561 pixels per target. All 375 earlier region objects, pins, groups, source colors and target mappings remain exact; no new aliases or material exceptions are needed.

Every actual frame was inspected in Vanilla and all four targets. Raised and lowered hands, isolated fingertip contours, the bare torso, legs, toes and kiss-face skin follow the selected palette. Green swim shorts and burgundy sandal straps remain original as the limbs move, including the lifted heel in the kiss. Actual eye details, closed lashes, hair and black outlines stay original. The tiny kiss-only `#8A273B` shade is hair material and remains unchanged. The full crop `[28,24,24,34]` covers the complete artwork bounds `[31,26,50,54]` and stays symmetric for West mirroring.

`tests/march_beach_actions.rs` contains 43 literal skin/material landmarks across all six strips. Its opt-in corpus check verifies every new source-or-target pixel, per-frame skin counts, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. All 3,750 earlier original/variant PNG and metadata files match `generated/characters-reina-juniper-march-beach-pilot-trial/characters/march` byte-for-byte.

Temporary negative controls fail at the intended material assertions: omitting the deep fingertip at action East frame 0 `[40,48]`, and selecting a burgundy sandal strap at `[39,52]`. The final candidate corpus test, targeted Clippy, formatting and normal test passed. All 1,524 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `68f2dd614e7aeb687dc90238422107f3a2f1c0c60e94ae26b3d52ee60b5346d5`.

The complete gallery includes every source frame and fourteen native West mirrors across seven small detail pages. Exact preview checks verify source/palette/frame bindings, hashes, full crop coverage, raw sidecars and every displayed pixel: 7,344,000 gallery pixels and 326,400 summary pixels. Chromium passed all eight HTML pages, 46 decoded images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native behavior and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_beach_actions'
nix-shell --pure --run 'cargo test --test march_beach_actions -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_beach_actions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-beach-actions-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-beach-actions-trial
```

Local evidence is under `tmp/march-beach-actions-author-*`: raw metadata, source inventory, all-target sheets, control/test logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored. Archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
