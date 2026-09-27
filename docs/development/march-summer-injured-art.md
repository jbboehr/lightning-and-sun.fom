# March Summer injured sprite masks

Adds all fourteen remaining Summer hurt strips: idle and walk North/South/East, blink South/East, sit North/South/East, seated blink South/East and action South. These contain 37 frames, including 25 distinct images. Native East packs also supply twelve West mirror views, giving 49 review cases. March now has 278 total sources and 1,112 recolored variants; his Summer sprite folder is complete at 44/44 strips.

## Source and material boundaries

Every source PNG is pinned to its exact SHA-256 from the read-only game archive. All fourteen strips use 80×80 frames, the `Default` atlas and numeric offset `[40,54]`. Idle/sit have one frame with default timing; walk has four frames at 0.15 seconds; both blink families have three durations `[0.075,0.125,0.075]`. The seven-frame hurt action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` and its native final-frame hold `[240,360]`. Sit and seated blink retain their seated flags. Recoloring changes no native behavior.

The new masks use the existing world skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 1,050 explicit component seeds selecting 1,971 pixels per target. No source color, group or mapping changes. All 264 previous region objects, pins and seeds remain exact.

Every actual frame was inspected in Vanilla and all four targets. The folded hand near the collar, bare arms, ankle strips, tiny rear-neck opening and displaced hands in the action animation recolor consistently. Summer exposes skin where Spring uses sleeves, so those boundaries were inspected from the actual Summer PNGs. The narrow deep contour beside the rear-facing shirt is exposed arm shading; it follows skin. The pale `#E4EEED`/`#9DAEB7` shirt, brown apron, green shorts, red hair, black outlines and footwear remain original. The crop `[28,24,24,34]` includes all artwork through y55 and stays symmetric for West mirroring.

## Verification

`tests/march_summer_injured.rs` contains 44 literal skin/material landmarks across all fourteen strips, including the final action pose. Its opt-in corpus test checks all four choices against the independently inspected skin/material inventory, exact source-or-target pixels, alpha, geometry, raw metadata and strict source hashes. It compares all 2,640 earlier original/variant PNG and metadata files against `generated/characters-reina-juniper-summer-finish-trial/characters/march`.

Temporary negative controls omit the deep finger shadow at hurt action frame 0 `[41,44]` and recolor the adjacent pale shirt edge `[36,44]`. Both controls failed at the intended literal material landmarks. The focused candidate test, targeted Clippy, formatting and normal test all passed. All 1,112 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `786dde3da2a5f75195241dedb040c7f6cae2855b620265db5c3f269a75033b2b`.

```sh
nix-shell --pure --run 'cargo test --test march_summer_injured'
nix-shell --pure --run 'cargo test --test march_summer_injured -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_summer_injured -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-summer-injured-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-summer-injured-trial
```

Local previews:

- [Five-choice summary](../../generated/march-summer-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-summer-injured-preview/blue-review/index.html): all 37 source frames, including the final action pose, and twelve native West mirrors across nine detail pages.

Preview verification covers exact source/palette bindings, hashes, full crop coverage, raw metadata, every displayed pixel and West mirroring. The checker passed 7,996,800 gallery pixels and 326,400 summary pixels. Chromium passed all ten pages, fifty images and their links. Static images do not demonstrate animation timing or natural scheduling; shared integration handles native hold probes and isolated installation separately. Game images and generated artifacts remain ignored and uncommitted.
