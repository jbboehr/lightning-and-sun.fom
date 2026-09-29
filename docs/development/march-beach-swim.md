# March Beach swimming masks

Adds the two Beach `bath_swim` strips, East and South. They contain eight distinct source frames; the East pack supplies four native West mirrors, giving twelve review cases. March now has 383 portrait/world sources and 1,532 recolored variants. The fresh archive inventory confirms his Beach folder is complete at 14/14 PNG strips.

[Five-choice summary](../../generated/march-beach-swim-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-beach-swim-preview/blue-review/index.html)

Every source PNG is pinned to its fresh SHA-256 from the read-only archive. Both strips use four 80×80 frames at 0.15 seconds, the `Default` atlas and `Middle`/54 offset. Native swimming is a linear East/South cycle; the East pack also renders the West mirror. Recoloring changes no animation behavior.

The masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 132 explicit component seeds selecting 224 pixels per target. All 381 earlier region objects, pins, groups, source colors and target mappings remain exact; no new aliases or material exceptions are needed.

Every actual frame was inspected in Vanilla and all four targets. Only the exposed face and jaw follow the selected palette. Hair, eyes and black outlines remain original. Both water colors (`#328BC9` and `#9DEBFC`), the changing foam line and detached droplets are preserved. The torso and swimwear are submerged in these strips. The crop `[28,40,24,24]` includes the complete artwork bounds `[30,41,49,62]` and stays symmetric for West mirroring; a normal walking crop would cut off some water effects.

`tests/march_beach_swim.rs` contains 27 literal skin/material landmarks covering every source frame, including deep jaw shading, eye details, the waterline and low droplets. Its opt-in corpus check verifies every new source-or-target pixel, per-frame skin counts, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. All 3,810 earlier original/variant PNG and metadata files match `generated/characters-reina-juniper-march-beach-actions-trial/characters/march` byte-for-byte.

Temporary negative controls fail at the intended material assertions: omitting the deep jaw component in East frame 0 `[37,52]`, and recoloring a water ripple at `[33,55]`. The final candidate corpus test, targeted Clippy, formatting and normal test passed. All 1,532 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `73b542da5788c7b6db3670e18a10c4a68b1f26003de264485a693622aa5f4669`.

The gallery includes every source frame and four native West mirrors across three small detail pages. Exact preview checks verify source/palette/frame bindings, hashes, full crop coverage, raw sidecars and every displayed pixel: 1,382,400 gallery pixels and 184,320 summary pixels. Chromium passed all four HTML pages, thirteen decoded images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native behavior and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_beach_swim'
nix-shell --pure --run 'cargo test --test march_beach_swim -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_beach_swim -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-beach-swim-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-beach-swim-trial
```

Local evidence is under `tmp/march-beach-swim-author-*`: raw metadata, complete Beach inventory, all-target sheets, control/test logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored. Archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
