# Eiland's Spring general actions, sleeping and kissing

Five strips add general actions North/South/East, sleep East and kiss East. They contain 26 source frames (18 distinct images) and 12 native West mirrors, for 38 review cases. Eiland now has 100 pinned sources and 400 recolored variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved the offline artwork on 2026-09-29.

[Five-choice summary](../../generated/eiland-spring-standard-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-standard-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-standard-study`. New sources are under `assets/animations/NPCs/Eiland/Sprites/Spring/`, named `spr_npc_eiland_spring_action_{north,south,east}.png`, `spr_npc_eiland_spring_sleep_east.png` and `spr_npc_eiland_spring_kiss_east.png`. Raw sidecars are recorded in `tmp/eiland-spring-standard-author-metadata.json` and match independent archive reads. Frames are 80×80, atlas `Default`, offset `Middle`/54. General actions have seven frames with durations `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four frames at `[0.15,0.15,0.8,0.15]`; sleep uses one frame with default timing. The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

All existing source colors, groups and target mappings remain unchanged. The five world skin shades are `#E9A980`, `#DE8F5D`, `#BA6A4C`, `#9C5241` and `#7D3B14`. All 95 earlier region objects and strict pins are preserved, and all 950 earlier original/variant PNG and metadata files match `generated/characters-balor-valen-eiland-spring-actions-trial/characters/eiland` byte-for-byte. Portrait-only recipes remain untouched.

All 26 actual frames were inspected in Vanilla and all four targets. The 338 new component seeds select 679 skin pixels per target: face and fine forehead shadows, neck/jaw/lip-edge shading, the moving hands and fingertips, and the hand raised beside the sleeping face. Ninety-five shared `#BA6A4C` pixels remain original because they shade the gold cape, belt and uniform. Pink hair, eye details, actual black mouth/eye marks, pale sleeves, cape, uniform and boots stay original; the existing portrait shade `#6A3126` continues to be clothing trim in these strips.

One small North-action boundary needed a correction. At `[34,44]` in source frames 2–5, the brown pixel initially looked like cape shading. Comparing the same pose in Summer shows the full exposed left hand and identifies that pixel as its upper edge; the Spring cape hides the rest. The final mask recolors those four pixels. A literal positive landmark failed on the provisional exclusion and passed after the correction. This cross-outfit comparison informs the anatomy only; no other outfit recipe was changed.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Action East | 31, 32, 31, 32, 31, 31, 32 |
| Action North | 7, 2, 4, 2, 4, 5, 4 |
| Action South | 36, 35, 35, 35, 35, 36, 38 |
| Kiss East | 31, 35, 39, 39 |
| Sleep East | 37 |

`tests/eiland_spring_standard.rs` checks 54 literal source-art landmarks, every source-or-target pixel using an independent material inventory, per-frame counts, alpha, geometry, raw metadata, strict source hashes, equal selections across four targets and all 950 prior files. Deliberate omission/spill controls fail at the intended assertions: missing fingertip shadow in action East frame 2 `[49,45]`, and incorrectly mapped cape trim in the same frame `[36,49]`. The corrected corpus test, both retained Eiland world tests and targeted Clippy pass. Retained tests change only their expanded corpus paths and total counts.

All 400 final variants strictly validate and match the inspected candidates. Common new-frame selection SHA-256: `be55d6639f9670ca0ef87f0ab3b2cfadc98361871fd081036ec5be2fcf18612f`. Profile SHA-256: `b7e8d370ec28df52a28f9d525c15b0d5ac5ef4f123a7cacd2cf4f6f8cf81ff79`.

The compact gallery covers all 38 cases on six detail pages. Crop `[28,24,24,34]` contains the full artwork bounds `[31,26,51,54]` and is symmetric for West mirroring. Exact checks verify source/palette/frame bindings, hashes, raw sidecars, crop coverage and every displayed pixel: 6,201,600 full-gallery pixels plus 326,400 summary pixels. Chromium checks all seven HTML pages, 39 decoded images and every link. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-valen-eiland-spring-standard.md) for the native animation probes and isolated installation results.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_standard -- --ignored'
nix-shell --pure --run 'cargo test --test eiland_world --test eiland_spring_actions -- --include-ignored'
nix-shell --pure --run 'cargo clippy --test eiland_world --test eiland_spring_actions --test eiland_spring_standard -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-standard-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-standard-trial
```

Local evidence is under `tmp/eiland-spring-standard-author-*`: fresh metadata/source inventory, all-target sheets, literal source grids and material decisions, omission/spill and North hand-edge red/green logs, frozen hashes, validation, byte-preservation and gallery/browser checks. The cross-season source comparison is `tmp/bve-spring-standard-north-compare.png`. Generated game images remain ignored. No runtime behavior changes are introduced.
