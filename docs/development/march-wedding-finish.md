# March Wedding blink, sit, action and kiss masks

Adds the nine remaining Wedding strips: blink East/South, sit North/South/East, general action North/South/East and kiss East. They contain 34 source frames, including 26 distinct images. Fifteen native West mirrors give 49 review cases. March now has 398 portrait/world sources and 1,592 recolored variants; the Wedding folder is complete at 15/15 PNG strips.

[Five-choice summary](../../generated/march-wedding-finish-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-wedding-finish-preview/blue-review/index.html)

Fresh exports use 80×80 frames, the `Default` atlas and `Middle`/54 offset. The source timing and metadata remain unchanged. Native East packs also supply West mirrors. Recoloring introduces no animation behavior changes. Archive SHA-256: `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. There are 823 new explicit component seeds selecting 1,366 pixels per target. All 389 earlier region objects, pins, source colors, groups and target mappings remain exact. No new aliases or material exceptions were needed.

Every actual frame was inspected in Vanilla and all four targets, including the full action cycles, closed-eye blinks, seated hands and all kiss poses. Face and jaw shading, fine skin around closed eyes, moving fingertips and exposed ankles follow the palette. The accepted ankle treatment continues between the trousers and brown shoes. Burgundy clothing, white cuffs, belt and shoes stay original, as do hair and true black eye outlines. There is no separately colored mouth interior in the new kiss frames.

`tests/march_wedding_finish.rs` contains 52 literal skin/material landmarks across all nine strips, plus independent color inventories for every source frame. It checks all four palettes, source-or-target pixels, per-frame skin counts, alpha, dimensions, raw metadata and strict source hashes. All 3,890 earlier original/variant PNG and metadata files match `generated/characters-reina-juniper-march-wedding-pilot-trial/characters/march` byte-for-byte.

Temporary controls omit the deep fingertip in action East frame 0 `[40,48]`, or recolor the brown shoe at `[39,53]`. Both fail at the intended independent material assertions. The accepted candidate corpus test, normal test, formatting and targeted Clippy pass. All 1,592 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `2705c31fcc04c70cd22994b858e41325f83c8dbb5edd1692fb8c0d8b05c4eb1c`.

The complete gallery uses eight small detail pages. Its crop `[28,24,24,34]` includes the full artwork bounds `[31,26,50,54]` and stays symmetric for West. Exact checks cover source/palette/frame bindings, hashes, full crop coverage, raw metadata and every displayed pixel: 7,996,800 gallery pixels and 326,400 summary pixels. Chromium checks all nine HTML pages, fifty decoded images and their links. Static images do not demonstrate native timing or natural scheduling; root integration handles those checks and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_wedding_finish'
nix-shell --pure --run 'cargo test --test march_wedding_finish -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_wedding_finish -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-wedding-finish-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-wedding-finish-trial
```

Local evidence is under `tmp/march-wedding-finish-author-*`: raw metadata, complete Wedding inventory, all-target sheets, control/test logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored.
