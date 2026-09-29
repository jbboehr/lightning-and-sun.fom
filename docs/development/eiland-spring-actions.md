# Eiland's Spring everyday actions

Eleven strips add blinking East/South and sitting, eating and drinking North/South/East. They contain 31 source frames (23 distinct images) and 12 native West mirrors, for 43 review cases. Eiland now has 95 pinned sources and 380 recolored variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia.

[Five-choice summary](../../generated/eiland-spring-actions-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-actions-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-actions-study`. New sources are under `assets/animations/NPCs/Eiland/Sprites/Spring/`, with names `spr_npc_eiland_spring_blink_{east,south}.png` and `spr_npc_eiland_spring_{sit,eat,drink}_{north,south,east}.png`. Raw sidecars are recorded in `tmp/eiland-spring-actions-author-metadata.json` and match independent archive reads. All frames are 80×80, atlas `Default`, offset `Middle`/54. The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The five existing world shades keep their roles: `#E9A980` highlight, `#DE8F5D` middle, `#BA6A4C` shadow, and `#9C5241` / `#7D3B14` deepest. No colors, groups or target mappings were added or changed. All 84 earlier region objects and strict source pins remain unchanged; all 840 earlier original/variant PNG and metadata files match `generated/characters-balor-valen-eiland-world-trial/characters/eiland` byte-for-byte. Portrait-only recipes remain untouched.

All 31 actual frames were inspected in Vanilla and all four targets. The 431 new component seeds select 868 skin pixels per target, including closed-eye shading, fine forehead/jaw tones, raised drinking fingers and the small hands visible beside the cape from behind. North-facing sit has no visible skin: its empty seed list is deliberate and all four outputs stay pixel-identical to the source.

The same skin colors also appear in gold trim and uniform details. Eighty-six such pixels remain original, including the isolated `#7D3B14` trim below the raised sleeve in eat East frame 2 and `#DE8F5D` uniform trim in eat South frame 2. Pink hair, eye details, gold cape and belt trim, uniform, boots, and red mouth interiors remain original. Held food and cups are drawn separately by the game and are absent from these static sprite strips.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Blink East / South | 33, 40, 33 / 39, 46, 39 |
| Sit East / North / South | 30 / 0 / 34 |
| Drink East / North / South | 34, 40, 34 / 2, 3, 2 / 36, 45, 36 |
| Eat East | 29, 36, 24, 35, 29 |
| Eat North | 2, 3, 2 |
| Eat South | 35, 37, 30, 46, 34 |

`tests/eiland_spring_actions.rs` checks 49 literal source-art landmarks, every source-or-target pixel using an independent material inventory, per-frame counts, alpha, geometry, raw metadata, strict source hashes, equal selections across four targets and all 840 prior files. Temporary omission/spill controls fail at their intended assertions: missing drinking-finger shadow in East frame 1 `[39,43]`, and incorrectly mapped uniform trim in eating South frame 2 `[37,44]`. The final corpus test, retained pilot test and targeted Clippy pass. The retained pilot test changes only its expanded corpus location and total count; all first-six assertions remain intact.

All 380 final variants strictly validate and match the inspected candidates. Common new-frame selection SHA-256: `e4710f414c71be08d7a5e42af6e4a7b5d07f77360e50b1e4aea3c53539fab7d5`. Profile SHA-256: `fd09fceef9f5d4473be96b0d175ece4f38f99c396e50d0a3456814b8535153c2`.

The compact gallery covers all 43 cases on eight detail pages. Crop `[28,24,24,34]` contains the full artwork bounds `[30,26,49,54]` and is symmetric for West mirroring. Exact checks verify source/palette/frame bindings, hashes, raw metadata, crop coverage and every displayed pixel: 7,017,600 full-gallery pixels plus 326,400 summary pixels. Chromium checks all nine HTML pages, 44 decoded images and every link. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-valen-eiland-spring-actions.md) for the native probes and isolated installation results.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_actions -- --ignored'
nix-shell --pure --run 'cargo test --test eiland_world -- --include-ignored'
nix-shell --pure --run 'cargo clippy --test eiland_spring_actions --test eiland_world -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-actions-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-actions-trial
```

Local evidence is under `tmp/eiland-spring-actions-author-*`: fresh metadata/source inventory, all-target sheets, source grids and material decisions, literal landmarks, red/green controls, frozen hashes, strict validation, byte-preservation and gallery/browser checks. Generated game images remain ignored. This slice introduces no runtime changes.
