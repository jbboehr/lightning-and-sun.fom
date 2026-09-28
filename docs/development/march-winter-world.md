# March Winter world-sprite pilot

This slice adds the six Winter idle/walk strips for North, South and East: 15 source frames, including nine distinct images. Native East packs also render five West mirrors, giving 20 review cases. March now has 331 sources and 1,324 recolored variants.

[Five-choice summary](../../generated/march-winter-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-preview/blue-review/index.html)

Fresh source PNGs are pinned to their SHA-256 hashes. All six use 80×80 frames on `Default`, with horizontal origin `Middle` and vertical origin `54.0`. Idle has one frame and default timing; walk has four frames at 0.15 seconds. This slice changes no animation behavior.

Every actual frame was inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. Winter exposes small wrist strips above black gloves and a two-pixel rear-neck strip below the hair. Those areas, including isolated deep wrist pixels in the stepping poses, follow the selected palette. The gloves (`#252525`/`#36373A`), blue sleeves, brown apron, trousers, hair, headband, eyes and footwear remain original. Winter clothes were inspected from the fresh source art rather than inheriting Autumn exclusions.

The six masks add 234 explicit component seeds selecting 367 skin pixels per target. Existing source shades `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14` cover all exposed skin here; no new aliases, groups or target mappings are required. All 325 earlier region objects, pins and mappings remain exact, as do all 3,250 earlier original/variant PNG and metadata files against `generated/characters-march-autumn-injured-trial/characters/march`.

`tests/march_winter_world.rs` includes 44 literal skin/material landmarks across all six strips. Its opt-in corpus test verifies strict source hashes, every new source-or-target pixel, alpha, geometry, raw metadata, equal selections for all four palettes and previous output bytes. Temporary controls were observed failing at the intended assertions: omitting wrist shadow `#D37A57` at East idle `[34,45]`, and recoloring apron shade `#624036` at `[38,42]`. The final candidate corpus check, targeted Clippy, formatting and normal test passed. All 1,324 final variants strictly validate and match the inspected candidate bytes. Selection SHA-256: `85ff9d0fff10100d0b0bf06679575b8329f8987196ef6fcdddc1e6b121538728`.

The gallery covers all 15 source frames and five West mirrors across three compact detail pages. The symmetric crop `[28,24,24,34]` contains the full artwork through y55. Exact preview checks cover source/palette/frame bindings, raw sidecars, hashes, full crop coverage and every displayed pixel: 3,264,000 gallery pixels plus 326,400 summary pixels. Chromium passed all four pages, 21 decoded images and their links. Static previews do not claim an interactive game playtest or natural West dispatch; shared integration checks native rendering and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_world'
nix-shell --pure --run 'cargo test --test march_winter_world -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_world -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-trial
```

Local raw metadata, inventory, all-target sheets, test/control logs, frozen hashes and validation/browser evidence are under `tmp/march-winter-author-*`. Source art and generated artifacts remain ignored. Read-only archive SHA-256: `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
