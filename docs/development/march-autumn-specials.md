# March Autumn smithing animations

Adds hammer East, wipebrow South and work_sit start/loop/end North: five strips, 23 source frames and 19 distinct images. March now has 308 sources and 1,232 target variants. The native seated-work cycle is complex; hammer and wipebrow are linear. Native rendering supports West using the East hammer pack with `image_xscale = -1`; the review therefore includes seven West mirrors, for 30 cases. This does not assert that the natural work schedule chooses West.

## Source and material boundaries

The game archive remains read-only and every PNG is pinned to its exact SHA-256. All five strips use 80×80 frames and the `Default` atlas with vertical offset 54. Hammer and wipebrow preserve numeric horizontal offset 40; the work_sit strips preserve `Middle`.

Hammer has seven frames at 0.1 seconds. Wipebrow has six durations `[0.15, 0.15, 0.45, 0.1, 0.075, 1.0]`. Work_sit start has one frame with default timing, loop has seven durations `[0.5, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25]`, and end has two durations `[0.25, 0.175]`. Native seated behavior and hammer sounds are retained by the game; the recoloring adds no behavior changes.

The five new regions add 293 explicit component seeds, selecting 519 skin pixels per target using the existing `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14` roles. These cover hammer grips, the raised wipebrow hand, face/neck shading and the tiny exposed fingers in work_sit start and the second ending frame. No source colors, groups or target mappings change.

The handle's `#E3B57F`, `#BB8151` and `#936244` remain wood. Blue metal, pale swing trail, yellow sparks, red hair, brown coat and sleeves, pale shirt, gray scarf and headband, trousers, footwear and mouth interiors remain original. Every actual frame was inspected in Vanilla and all four target palettes. The artwork reaches x76/y63, so the complete review uses crop `[0,22,80,44]`, preserving the entire hammer, trail and sparks in both directions.

The Autumn work_sit loop exposes no skin: all seven frames show only hair, headband and clothing. Its pinned region has no seeds and its PNG remains byte-identical in every target. The first work_sit ending frame also stays pixel-identical; start and the second ending frame each recolor six finger pixels.

## Verification

`tests/march_autumn_specials.rs` uses 36 literal source-art landmarks across moving skin and protected materials. The opt-in test checks strict source hashes, exact source-or-target pixels, alpha, dimensions, raw metadata and the same selected mask in all four targets. All 303 earlier region objects, sixteen source colors, seven groups and mappings remain unchanged. All 3,030 earlier original/variant PNG and metadata files are compared against `generated/characters-reina-juniper-march-autumn-reading-trial/characters/march`.

Temporary negative controls fail at the intended boundaries: an omitted deep grip pixel at hammer frame 0 `[43,47]` and a recolored wooden handle highlight at `[45,45]`. The focused candidate test, targeted Clippy, formatting and normal test all passed. All 1,232 final variants strictly validate and match the reviewed candidate bytes.

Selection SHA-256: `62e253d287da97489f66fcc314d80c78f10ed3374a6dffa674a64282ec824b36`.

```sh
nix-shell --pure --run 'cargo test --test march_autumn_specials'
nix-shell --pure --run 'cargo test --test march_autumn_specials -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_autumn_specials -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-autumn-specials-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-autumn-specials-trial
```

Local previews:

- [Five-choice summary](../../generated/march-autumn-specials-preview/summary.png).
- [Every frame in Vanilla and Debug Blue](../../generated/march-autumn-specials-preview/blue-review/index.html), across five detail pages, including seven native West hammer mirrors.

Preview checks cover exact source/palette bindings, hashes, complete crops, raw metadata, every displayed pixel and West mirroring. The checker passed 5,280,000 gallery pixels and 880,000 summary pixels. Chromium passed all six pages, 31 images and their links. Static images do not demonstrate animation timing or sound. Combined installation and native runtime checks are handled separately by the integration slice. All game images and generated artifacts remain ignored and uncommitted.
