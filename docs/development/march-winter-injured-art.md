# March Winter injured sprite masks

This slice adds all fourteen remaining Winter hurt strips: idle and walk North/South/East, blink South/East, sit North/South/East, seated blink South/East and action South. They contain 37 source frames, including 24 distinct images. Native East packs also supply twelve West mirror views, giving 49 review cases. March now has 369 total sources and 1,476 recolored variants; his Winter sprite folder is complete at 44/44 strips.

[Five-choice summary](../../generated/march-winter-injured-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-injured-preview/blue-review/index.html)

Every source PNG is pinned to its fresh SHA-256 from the read-only archive. All fourteen strips use 80×80 frames, the `Default` atlas and numeric offset `[40,54]`. Idle/sit have one frame with default timing; walk has four frames at 0.15 seconds; both blink families have three durations `[0.075,0.125,0.075]`. The seven-frame hurt action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` and the native final-frame hold `[240,360]`. Sit and seated blink retain their native seated flags. Recoloring changes no animation behavior.

The masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 661 explicit component seeds selecting 1,036 pixels per target. All 355 earlier region objects, pins, groups, source colors and mappings remain exact; no new aliases or material exceptions are needed.

Every actual frame was inspected in Vanilla and all four targets. The exposed wrists between sleeves and black gloves, the nape, face and collar opening recolor consistently, including the shifted wrist in the final action pose. Winter gloves cover fingers that are exposed in other outfits, so those dark gray shapes remain original. Blue sleeves, the brown apron and straps, goggles/headband, hair, eyes, trousers and footwear also stay original. The full crop `[28,24,24,34]` includes all artwork through y55 and stays symmetric for West mirroring.

`tests/march_winter_injured.rs` contains 48 literal skin/material landmarks across all fourteen strips, including closed eyes, the exposed nape, glove/sleeve edges and final action pose. Its opt-in corpus check verifies every new source-or-target pixel, per-frame skin counts, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. All 3,550 earlier original/variant PNG and metadata files match `generated/characters-reina-juniper-winter-finish-trial/characters/march` byte-for-byte.

Temporary negative controls were observed failing at the intended material assertions: omitting the exposed neck component at action frame 0 `[39,42]`, and selecting adjacent blue sleeve shading at `[36,44]`. The final candidate corpus test, targeted Clippy, formatting and normal test passed. All 1,476 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `96cc42cd98f64b8359893a9774f25081190381cccdbaa4588e418b8df675f95e`.

The complete gallery includes every source frame and twelve native West mirrors across nine small detail pages. Exact preview checks verify source/palette/frame bindings, hashes, full crop coverage, raw sidecars and every displayed pixel: 7,996,800 gallery pixels and 326,400 summary pixels. Chromium passed all ten HTML pages, fifty decoded images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native holds and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_injured'
nix-shell --pure --run 'cargo test --test march_winter_injured -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_injured -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-injured-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-injured-trial
```

Local evidence is under `tmp/march-winter-injured-author-*`: raw metadata, fresh source inventory, all-target sheets, control/test logs, frozen input hashes, variant validation, old-file preservation and preview/browser results. Game images and generated artifacts remain ignored. Archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
