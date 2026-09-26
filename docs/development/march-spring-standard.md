# March Spring general actions

Adds general action North/South/East, sleep East and kiss East: five strips, 26 source frames (19 distinct images) and twelve native mirrored West views. March now has 203 sources and 812 recolored variants. All 198 earlier source regions and their outputs remain unchanged.

## Materials and metadata

The source remains the read-only mounted `tmp/fields-of-mistria/assets.zip`. Every PNG is pinned by its original SHA-256. All new animations preserve 80×80 frames, the `Default` atlas and Middle/54 offset. General actions have seven frames with durations `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss has four with `[0.15, 0.15, 0.8, 0.15]`; sleep has one using the native default.

These sprites use the existing world ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. All occurrences in these five strips are skin. The drinking alias `#E8B171` does not occur here. No source colors, groups or palette mappings were added or changed.

The five new regions contain 464 explicit component seeds selecting 877 pixels per target. They cover face and neck shading, the extended action hand, disconnected dark knuckles and the hand touching the face during sleep. Red hair, gray-blue goggles and apron, green cuffs and shirt, black eyelids/outlines, trousers and boots stay original. The small near-identical goggle and trouser colors in kissing frames remain material colors.

All actual frames were inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. The compact source grids were used to check moving fingertip/cuff boundaries and the sleep hand/face overlap. The original sixteen source colors, seven groups and every old region object, hash and seed remain intact; the set file is byte-identical to the accepted previous slice.

## Verification and local previews

`tests/march_spring_standard.rs` contains 31 literal source-art landmarks across skin, cuffs, goggles, apron, eyes, hair and footwear. Its local corpus test checks all four targets, every new pixel, alpha, dimensions, strict source hashes and unchanged metadata. It compares all 1,980 previous original/variant PNG and metadata files to `generated/characters-reina-juniper-march-actions-trial/characters/march`.

Two temporary controls failed before the accepted candidate passed: removing the deep fingertip component at action East, zero-based frame 1, `[48,45]`; and deliberately mapping the green cuff at `[45,44]`. The focused test, normal data test, formatting and targeted Clippy passed. The common selection SHA-256 is `e37b63a5a8cce9cc5e37a58c8505910fe135c5fc95b96f38da44cf9ce97b5ba2`.

```sh
nix-shell --pure --run 'cargo test --test march_spring_standard'
nix-shell --pure --run 'cargo test --test march_spring_standard -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_standard -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-standard-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-standard-trial
```

All 812 standalone variants validate and match the inspected candidate bytes. Local review artifacts are:

- `generated/march-spring-standard-preview/summary.png`: five choices across action, sleep and kiss poses.
- `generated/march-spring-standard-preview/blue-review/index.html`: all 38 direction/frame cases in Vanilla and Debug Blue across six detail pages, including twelve West mirrors.

Preview checks verify source/palette identity, hashes, complete crops, metadata and every displayed pixel, including native West mirroring. Chromium loads every page and image and checks all links. Parent integration handles the combined build, install and native game probes. Source images, output variants and review sheets remain ignored local artifacts.
