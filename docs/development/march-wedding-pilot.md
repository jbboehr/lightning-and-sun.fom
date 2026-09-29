# March Wedding idle and walking sprite masks

Adds six Wedding strips: idle and walk North, South and East. They contain 15 source frames, including nine distinct images. Five native West mirrors give twenty review cases. March now has 389 portrait/world sources and 1,556 recolored variants; six of the Wedding folder's fifteen PNG strips are covered.

[Five-choice summary](../../generated/march-wedding-pilot-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-wedding-pilot-preview/blue-review/index.html)

Fresh exports use 80×80 frames, the `Default` atlas and `Middle`/54 offset. Idle has one frame with default timing; walk has four frames at 0.15 seconds. The native animator shares East packs with West and mirrors them horizontally. No animation behavior changes.

The current archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`. Although the archive changed since the Beach slice, all 383 earlier PNG pins and all 766 original PNG/metadata files match the accepted baseline exactly. No earlier pins were refreshed. All previous region objects, source colors, groups and target mappings remain unchanged.

The new masks use existing skin shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. There are 343 new explicit component seeds selecting 565 pixels per target. Every actual frame was inspected in Vanilla and all four targets. Face, jaw, bare hands and the small exposed ankle patches between trousers and brown shoes follow the palette. The burgundy suit, white cuffs, belt, shoes, blue accent, hair and actual eye details remain original. The moving ankle patches were checked independently against the trouser and shoe boundaries. No new colors, aliases or material exceptions were needed.

The focused test in `tests/march_wedding_pilot.rs` includes 46 literal skin/material landmarks across every distinct pose, plus exact inventory checks for every source frame. It checks all four palettes, source-or-target pixels, per-frame skin counts, alpha, dimensions, raw metadata and strict source hashes. All 3,830 prior original/variant PNG and metadata files match `generated/characters-reina-juniper-beach-finish-trial/characters/march` byte-for-byte.

Temporary omission and spill controls remove the deep fingertip at idle East `[35,47]` and recolor a brown shoe at `[39,53]`. Both fail at the intended independent material assertions. The final candidate corpus test, normal test, formatting and targeted Clippy pass. All 1,556 final variants strictly validate and match the reviewed candidate bytes. Common selection SHA-256: `98a61f121338319612ac0083f788c00b4176e2927a3b61ee6dfc2bd063be9624`.

The gallery covers every source frame and five native West mirrors across three small detail pages. The crop `[28,24,24,34]` contains all artwork bounds `[31,27,48,55]` and remains symmetric for West mirroring. Exact checks verify source/palette/frame bindings, hashes, complete crop coverage, raw sidecars and every displayed pixel: 3,264,000 gallery pixels and 326,400 summary pixels. Chromium checks all four HTML pages, twenty-one decoded images and their links. Static images do not demonstrate animation timing or natural scheduling; root integration checks native behavior and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_wedding_pilot'
nix-shell --pure --run 'cargo test --test march_wedding_pilot -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_wedding_pilot -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-wedding-pilot-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-wedding-pilot-trial
```

Local evidence is under `tmp/march-wedding-pilot-author-*`: raw metadata, complete Wedding inventory, all-target sheets, control/test logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored.
