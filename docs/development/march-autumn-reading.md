# March Autumn seated reading

Adds the special-animation seated-reading start, loop and end South strips. These contain ten source frames (eight distinct images). The native cycle is South-only, so the review contains no invented West mirrors. March now has 303 sources and 1,212 recolored variants.

## Source and material boundaries

Every PNG is pinned by its exact SHA-256 from the read-only game archive. The strips retain 80×80 frames, the `Default` atlas and Middle/54 offset. Start and end each have three frames at 0.1 seconds; the four-frame loop uses `[3.0,0.1,3.0,0.1]`. Native metadata defines a complex, seated cycle with South as its only direction.

The existing world skin ramp (`#EEDDA5`, `#E8B271`, `#D37A57`, `#7D3B14`) covers the face, neck and tiny fingers at the moving book edges. The three regions add 178 explicit component seeds selecting 332 pixels per target. No colors, groups or target mappings change; all 300 previous region objects, pins and seeds remain exact.

The book stays a distinct material: paper `#F6E4D7` / `#C9AF9C`, cover `#B67C6F` / `#855053` / `#663A3E`, and binding `#422628` remain original. The brown coat and sleeves, pale shirt, gray scarf and headband, hair, eyes, trousers and footwear also remain unchanged. Every actual frame was inspected in Vanilla and all four targets, including the isolated fingertip shadows during opening/closing and the face/neck above the turning pages. The crop `[28,24,24,34]` includes the complete book and body; actual bounds are x29–50 and y27–51.

## Verification

`tests/march_autumn_reading.rs` contains 28 independently observed source-art landmarks across skin, paper, cover, binding and clothing. The focused corpus test checks all four targets, exact source-or-target pixels, alpha, geometry, source hashes and unchanged raw metadata. It compares all 3,000 previous original/variant PNG and metadata files against `generated/characters-reina-juniper-march-autumn-standard-trial/characters/march`.

Temporary controls omit the deep fingertip at zero-based start frame 2, `[35,47]`, and recolor the adjacent book paper `[38,42]`. Both failed at the intended literal material landmarks. The accepted candidate, normal data test, formatting and targeted Clippy passed. All 1,212 final variants strictly validate and match the reviewed candidate bytes.

Common selection SHA-256: `e4b9df7ce2dbf9e5ad39a95453d57afd195139db8d021a084fce6c6718a48d3d`.

```sh
nix-shell --pure --run 'cargo test --test march_autumn_reading'
nix-shell --pure --run 'cargo test --test march_autumn_reading -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_autumn_reading -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-autumn-reading-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-autumn-reading-trial
```

Local previews:

- [Five-choice summary](../../generated/march-autumn-reading-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-autumn-reading-preview/blue-review/index.html): all ten South frames across three detail pages.

Preview checks verify source/palette identity, hashes, complete crop coverage, raw metadata, every displayed pixel and the absence of invented mirror cases. The exact checker passed 1,632,000 gallery pixels and 326,400 summary pixels. Chromium passed all four pages, eleven images and their links. Game images, generated variants and local review artifacts remain ignored and uncommitted.
