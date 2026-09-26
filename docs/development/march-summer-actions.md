# March Summer action sprite masks

Adds eleven regular Summer strips: blink East/South and sit, eat and drink North/South/East. These contain 31 source frames (23 distinct images), plus twelve native West mirrors for 43 review cases. March now has 251 sources and 1,004 recolored variants.

## Source and material boundaries

Every source PNG is pinned to its exact SHA-256 from the read-only archive. All eleven strips retain 80×80 frames, the `Default` atlas and Middle/54 offset.

| Animation | Frames per direction | Source duration |
| --- | --- | --- |
| Blink East/South | 3 | `[0.075,0.125,0.075]` |
| Sit North/South/East | 1 | Native default |
| Drink North/South/East | 3 | `1.0` |
| Eat North | 3 | `1.0` |
| Eat East/South | 5 | `[0.125,0.15,0.175,0.125,0.6]` |

The native animator mirrors East packs for West. Sitting, eating and drinking retain their seated flags; eating and drinking also retain the native final-frame hold `[240,360]`.

The masks use the existing five world skin shades: `#EEDDA5`, `#E8B271`, `#D37A57`, `#7D3B14` and drinking alias `#E8B171`. They add 825 explicit component seeds selecting 1,422 pixels per target. No source colors, groups or target mappings change. All 240 previous region objects and pins remain exact.

Every actual frame was inspected in Vanilla and all four targets. This covers closed-eye skin, raised and lowered hands, exposed arms, the neck, small seated leg patches and skin bordering the open mouth. The red mouth interior (`#410808`, `#9E2626`), black eye/outline pixels, brown apron and straps, pale collar, shorts, hair and shoes stay original. Tiny near-identical material colors in the drinking strips also stay original. Food and cups are drawn separately by the game; these previews show the complete character strips without adding those attachments.

The symmetric review crop `[28,24,24,34]` includes all new artwork, whose bounds are x31–48 and y26–54.

## Verification

`tests/march_summer_actions.rs` uses 40 independently observed source-art landmarks across all eleven strips. Its local corpus test checks the exact source-or-target value of every new pixel, alpha, dimensions, raw metadata and strict source hashes in all four choices. It compares all 2,400 earlier original/variant PNG and metadata files with `generated/characters-reina-juniper-march-summer-trial/characters/march`.

Temporary negative controls omit the fine raised-arm shade in drinking East frame 1 at `[40,40]` and recolor the red mouth interior in eating East frame 2 at `[42,37]`. Both controls failed at the literal material landmarks before the accepted candidate passed. Formatting, targeted Clippy and the normal data test passed. All 1,004 final variants strictly validate and match the reviewed candidate bytes.

Selection SHA-256: `4b75c54e04429bd19be8577b29299a9204c8784590ba5452f20460e2f21ab543`.

```sh
nix-shell --pure --run 'cargo test --test march_summer_actions'
nix-shell --pure --run 'cargo test --test march_summer_actions -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_summer_actions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-summer-actions-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-summer-actions-trial
```

Local previews:

- [Five-choice summary](../../generated/march-summer-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-summer-actions-preview/blue-review/index.html): all 43 frame/direction cases across eight detail pages.

Preview checks cover exact source/palette bindings, full crop coverage, metadata, hashes and every displayed pixel, including West mirroring. The exact checker passed 7,017,600 gallery pixels and 326,400 summary pixels. Chromium passed all nine pages, 44 images and their links. Game images and generated artifacts remain ignored and uncommitted.
