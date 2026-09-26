# March Spring injured sprite masks

Adds all fourteen remaining Spring hurt strips: idle and walk North/South/East, blink South/East, sit North/South/East, seated blink South/East and action South. These contain 37 frames, including 24 distinct images. The native East packs also supply twelve West mirror views, giving 49 review cases. March now has 234 total sources and 936 recolored variants; his Spring sprite folder is complete at 53/53 strips.

## Source and material boundaries

Every source PNG is pinned to its exact SHA-256 from the read-only game archive. All fourteen strips use 80×80 frames, the `Default` atlas and numeric offset `[40,54]`. Idle/sit have one frame with default timing; walk has four frames at 0.15 seconds; both blink families have three durations `[0.075,0.125,0.075]`. The seven-frame hurt action retains `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` and its native final-frame hold `[240,360]`. All six cycles are linear; sit and seated blink retain their seated flags.

The new masks use the existing world skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. They add 763 explicit component seeds selecting 1,448 pixels per target. No source color, group or mapping changes. All 220 previous region objects, pins and seeds remain exact.

Every actual frame was inspected in Vanilla and all four targets. The folded hand near the collar, hanging hand, tiny rear-neck opening and the displaced hands in the action animation recolor consistently. Dark-green `#58715C` is a clothing fold and stays original, along with the other green shirt colors, goggles, hair, apron and legs/footwear. North-facing and seated views keep those boundaries when the visible skin shrinks to a handful of pixels. The crop `[28,24,24,34]` includes the walking artwork through y55 and stays symmetric for West mirroring.

## Verification

`tests/march_spring_injured.rs` contains 39 independently observed skin/material landmarks across all fourteen strips. Its opt-in corpus test checks all four choices against the source skin/material inventory, including exact source-or-target pixels, alpha, geometry, raw metadata and source hashes. It compares all 2,200 earlier original/variant PNG and metadata files against `generated/characters-reina-juniper-march-finish-trial/characters/march`.

Temporary negative controls omit the deep finger shadow at hurt action frame 0, `[41,44]`, and recolor the adjacent dark-green fold `[44,44]`. Both failed at the literal boundary landmarks before the accepted candidate passed the full focused test. Formatting, targeted Clippy and the normal data test also passed. All 936 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `b35a55bb66940d3b780258d332cd4afe21c5313f451fe36cbdd7f33e491daf7f`.

```sh
nix-shell --pure --run 'cargo test --test march_spring_injured'
nix-shell --pure --run 'cargo test --test march_spring_injured -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_injured -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-injured-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-injured-trial
```

Local previews:

- [Five-choice summary](../../generated/march-spring-injured-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-spring-injured-preview/blue-review/index.html): all 37 source frames and twelve native West mirrors across nine detail pages.

Preview verification covers exact source/palette bindings, hashes, complete crop coverage, raw metadata, every displayed pixel and West mirroring. The checker passed 7,996,800 gallery pixels and 326,400 summary pixels. Chromium passed all ten pages, fifty images and their links. Shared integration, native hold probes and installation are documented separately in [the integration notes](march-spring-injured.md). Game images and generated artifacts remain ignored and uncommitted.
