# March Autumn injured sprite masks

This slice adds all fourteen remaining Autumn hurt strips: idle and walk North/South/East, blink South/East, sit North/South/East, seated blink South/East and action South. They contain 37 source frames, including 25 distinct images. Native East packs also supply twelve West mirror views, giving 49 review cases. March now has 325 total sources and 1,300 recolored variants; his Autumn sprite folder is complete at 47/47 strips.

[Five-choice summary](../../generated/march-autumn-injured-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-autumn-injured-preview/blue-review/index.html)

Every source PNG is pinned to its fresh SHA-256 from the read-only archive. All fourteen strips use 80×80 frames, the `Default` atlas and numeric offset `[40,54]`. Idle/sit have one frame with default timing; walk has four frames at 0.15 seconds; both blink families have three durations `[0.075,0.125,0.075]`. The seven-frame hurt action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` and the native final-frame hold `[240,360]`. Sit and seated blink retain their native seated flags. Recoloring changes no animation behavior.

The masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 708 explicit component seeds selecting 1,387 pixels per target. All 311 earlier region objects, pins, groups, source colors and mappings remain exact; no new aliases or material exceptions are needed.

Every actual frame was inspected in Vanilla and all four targets. The cradled hand at the collar, hanging fingers and shifting hands in the action animation recolor consistently, including their isolated deep shadows and the final action pose. Autumn jacket sleeves and rear neckline cover areas exposed in Summer, so those warm brown panels remain original. The jacket uses `#83685F`, `#55423B` and `#2E2220`; scarf/headband, pale shirt, hair, eyes, trousers and footwear also stay original. The full crop `[28,24,24,34]` includes all artwork through y55 and stays symmetric for West mirroring.

`tests/march_autumn_injured.rs` contains 44 literal skin/material landmarks across all fourteen strips, including closed eyes, the covered nape, sleeve edges and final action pose. Its opt-in corpus check verifies every new source-or-target pixel, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. All 3,110 earlier original/variant PNG and metadata files match `generated/characters-march-autumn-poses-trial/characters/march` byte-for-byte.

Temporary negative controls were observed failing at the intended material assertions: omitting the deep folded-finger pixel at action frame 0 `[41,44]`, and selecting adjacent brown jacket shading at `[36,44]`. The final candidate corpus test, targeted Clippy, formatting and normal test passed. All 1,300 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `a8875700bcbfdd17db323a1918b65f1a06980ecf8f70e3078467823eb9c794e1`.

The complete gallery includes every source frame and twelve native West mirrors across nine small detail pages. Exact preview checks verify source/palette/frame bindings, hashes, full crop coverage, raw sidecars and every displayed pixel: 7,996,800 gallery pixels and 326,400 summary pixels. Chromium passed all ten HTML pages, fifty decoded images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native holds and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_autumn_injured'
nix-shell --pure --run 'cargo test --test march_autumn_injured -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_autumn_injured -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-autumn-injured-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-autumn-injured-trial
```

Local evidence is under `tmp/march-autumn-injured-author-*`: raw metadata, fresh source inventory, all-target sheets, control/test logs, frozen input hashes, variant validation, old-file preservation and preview/browser results. Game images and generated artifacts remain ignored. Archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.

The user approved this artwork for commit on 2026-09-27.
