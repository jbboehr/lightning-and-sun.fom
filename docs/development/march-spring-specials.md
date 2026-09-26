# March Spring smithing animations

Adds special-animation hammer East, wipebrow South and work_sit start/loop/end North: five strips, 23 frames and 19 distinct images. March now has 214 sources and 856 recolored variants. The native seated-work cycle is complex; hammer and wipebrow are linear. Native West uses the East hammer pack with `image_xscale = -1`, so the review includes seven West mirrors, for 30 cases total.

## Source and material boundaries

The original archive remains read-only. Every PNG is pinned to its exact SHA-256. All five strips use 80×80 frames and the `Default` atlas with vertical offset 54. Hammer and wipebrow use numeric horizontal offset 40; the work_sit strips use `Middle`.

Hammer has seven frames at 0.1 seconds. Wipebrow has six durations `[0.15, 0.15, 0.45, 0.1, 0.075, 1.0]`. Work_sit start has one frame with default timing, loop has seven durations `[0.5, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25]`, and end has two durations `[0.25, 0.175]`. Hammer retains its native swing/hit sound configuration.

All exposed skin uses existing `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. The new regions add 342 explicit component seeds, selecting 605 pixels per target. This includes the moving hands gripping the hammer, the raised wipebrow hand and the two-pixel neck opening visible below the hair during the seated-work loop. No colors, groups or mappings change.

The hammer handle's `#E3B57F`, `#BB8151` and `#936244` remain wood. The blue metal, pale swing trail and yellow sparks also stay original. Goggles, hair, green shirt/cuffs, apron, mouth interiors and footwear are protected. Every actual frame was inspected in Vanilla and all four target palettes. The effect silhouette reaches x76/y63, so the complete review uses crop `[0,22,80,44]`, preserving the full hammer, trail and sparks in both directions.

## Verification

`tests/march_spring_specials.rs` contains 31 literal source-art landmarks across grip fingers, wood, metal, swing effects, face, neck, hair and clothing. The opt-in test checks strict source hashes, exact source-or-target output, alpha, dimensions and unchanged metadata in all four targets. It compares all 2,090 earlier original/variant PNG and metadata files against `generated/characters-reina-juniper-march-reactions-trial/characters/march`.

Two temporary negative controls fail at the intended boundaries: omitting the deep grip pixel at hammer frame 0, `[43,47]`, and selecting the wooden handle highlight at `[45,45]`. The accepted candidate passes the focused test and targeted Clippy; the normal data test also passes after promotion. All 856 final variants strictly validate and match the reviewed candidate bytes. The previous 209 region objects, sixteen source colors, seven groups and all mappings remain unchanged. Selection SHA-256: `8a2c63fd4345ae6d58363eefcf1cc3c9ac1472ff8176e19b8b519a11713dd5e5`.

```sh
nix-shell --pure --run 'cargo test --test march_spring_specials'
nix-shell --pure --run 'cargo test --test march_spring_specials -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_specials -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-specials-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-specials-trial
```

Local previews:

- `generated/march-spring-specials-preview/summary.png`: five choices across hammer, wipebrow and seated-work poses.
- `generated/march-spring-specials-preview/blue-review/index.html`: every source frame and native West hammer mirror in Vanilla and Debug Blue, across five detail pages.

Preview checks verify source/palette bindings, hashes, full crop coverage, raw metadata, every displayed pixel and West mirroring. The checker passed 5,280,000 full-review pixels and 880,000 summary pixels. Chromium passed all six pages, 31 images and their links. Parent integration covers the combined package, installation and native probes separately. All game images and generated artifacts remain ignored and uncommitted.
