# March Autumn general actions

Adds general action North/South/East, sleep East and kiss East: five strips, 26 source frames (18 distinct images) and twelve native West mirror views. March now has 300 sources and 1,200 recolored variants. The preceding 295 source regions and their outputs remain unchanged.

## Source and material boundaries

Every PNG is pinned by its exact SHA-256 from the read-only archive. All five strips retain 80×80 frames, the `Default` atlas and Middle/54 offset. General actions have seven frames with durations `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four with `[0.15,0.15,0.8,0.15]`; sleep has one using the native default.

Direct archive NPC metadata confirms these Autumn cycles are linear. Action retains the final-frame hold `[240,360]` and speaking fallback `idle`. Kiss and sleep expose East source packs; the native animator can use those same packs for West, with the NPC renderer applying horizontal mirroring. The review includes all twelve such West frames, without asserting which direction the natural scheduler chooses.

The four existing world skin shades (`#EEDDA5`, `#E8B271`, `#D37A57`, `#7D3B14`) cover all skin in this slice. The drinking alias `#E8B171` does not occur. No source colors, groups, target mappings or prior masks change.

The five new masks contain 434 explicit component seeds selecting 828 pixels per target. Every actual frame was inspected in Vanilla and all four targets, including extended fingertips, the hand against the face during sleep, kissing skin around closed eyes, the fine lower-face edges and moving sleeve boundaries. Brown coat/sleeve colors, the pale shirt, gray scarf and headband, red hair, dark eye outlines, trousers and footwear stay original. The kissing frames' slightly different red hair and gray material shades also stay original.

The symmetric crop `[28,24,24,34]` contains all new artwork, whose bounds are x31–50 and y26–54, including extended fingertips and mirrored poses.

## Verification

`tests/march_autumn_standard.rs` contains 37 independently observed skin/material landmarks. Its focused corpus test checks all four targets, every new pixel against its source or exact target, alpha, dimensions, strict source hashes and unchanged raw metadata. It compares all 2,950 previous original/variant PNG and metadata files to `generated/characters-reina-juniper-march-autumn-actions-trial/characters/march`.

Temporary controls omit the deep fingertip at action East frame 1, `[48,45]`, and recolor the brown coat sleeve `[42,41]`. Both failed at the intended literal landmarks. The accepted candidate, normal data test, formatting and targeted Clippy passed. All 1,200 final variants strictly validate and match the reviewed candidate bytes.

Common selection SHA-256: `a8702979766e883b66b45acad2a624c5743cf7576d5c67cd7b1fce00b4801d92`.

```sh
nix-shell --pure --run 'cargo test --test march_autumn_standard'
nix-shell --pure --run 'cargo test --test march_autumn_standard -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_autumn_standard -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-autumn-standard-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-autumn-standard-trial
```

Local previews:

- [Five-choice summary](../../generated/march-autumn-standard-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-autumn-standard-preview/blue-review/index.html): all 38 frame/direction cases across six detail pages.

Preview verification checks exact source/palette identity, hashes, full crop coverage, raw metadata and every displayed pixel, including West mirroring. The exact checker passed 6,201,600 gallery pixels and 326,400 summary pixels. Chromium passed all seven pages, 39 images and their links. Game images, generated variants and local review artifacts remain ignored and uncommitted.
