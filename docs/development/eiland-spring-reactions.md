# Eiland's Spring shocked and seated-reading animations

Six strips add shocked and seated-reading start, loop and end phases. They contain 13 South-only source frames (10 distinct images). Eiland now has 106 pinned sources and 424 recolored variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved the offline artwork on 2026-09-29.

[Five-choice summary](../../generated/eiland-spring-reactions-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-reactions-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-reactions-study`. Sources are under `assets/animations/NPCs/Eiland/Sprites/Spring/`, named `spr_npc_eiland_spring_shocked_{start,loop,end}_south.png` and `spr_npc_eiland_specialanimation_spring_read_sit_{start,loop,end}_south.png`. Raw sidecars in `tmp/eiland-spring-reactions-author-metadata.json` match independent archive reads. Frames are 80×80, atlas `Default`, offset `Middle`/54. Reading start/end have three frames at 0.1 seconds; its four-frame loop uses `[3.0,0.1,3.0,0.1]`. Each shocked phase has one frame with default timing. These native complex cycles are South-only, with reading seated; the gallery introduces no West views. The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

Existing source colors, groups and target mappings remain unchanged. The five world skin shades are `#E9A980`, `#DE8F5D`, `#BA6A4C`, `#9C5241` and `#7D3B14`. All 100 earlier region objects and strict pins are preserved, and all 1,000 earlier original/variant PNG and metadata files match `generated/characters-balor-valen-eiland-spring-standard-trial/characters/eiland` byte-for-byte. Portrait-only recipes remain untouched.

All 13 actual frames were inspected in Vanilla and all four targets. The 213 new component seeds select 447 skin pixels per target, including forehead and jaw shading, closed-eye skin, and fingers opening or closing the book. The reading loop conceals both hands behind the book. Thirty-two shared `#BA6A4C` pixels remain original as cape, belt and uniform trim. Pink hair, gray/white eyes, actual red mouth interiors, sleeves and boots stay original. The teal book cover and tan/cream pages use distinct material colors and remain unchanged. Existing portrait shade `#6A3126` is clothing trim in these strips.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Reading start | 31, 32, 33 |
| Reading loop | 26, 34, 26, 34 |
| Reading end | 40, 25, 31 |
| Shocked start | 46 |
| Shocked loop | 43 |
| Shocked end | 46 |

`tests/eiland_spring_reactions.rs` checks 47 literal source-art landmarks, every source-or-target pixel using an independent material inventory, per-frame counts, alpha, geometry, raw metadata, strict hashes, equal selections across four targets and all 1,000 prior files. Both deliberate controls fail at their intended assertions: missing fingertip shadow in reading start frame 2 `[35,47]`, and incorrectly mapped gold trim in shocked loop `[37,43]`. The final corpus test, all three retained Eiland tests and targeted Clippy pass. Retained tests change only expanded corpus paths and total counts.

All 424 final variants strictly validate and match the inspected candidates. Common new-frame selection SHA-256: `4033e4c3ca00c2a210c7b7100cfb86fa77e5eabb0b002fcf147e883873d514c1`. Profile SHA-256: `cf543692652044f103bf76e79c1c73d1c0065298681ee4af5c858ec6d660b6a9`.

The compact gallery covers all 13 cases on three detail pages. Crop `[28,22,24,34]` contains the full artwork bounds `[29,25,50,54]`. Exact checks verify source/palette/frame bindings, hashes, raw sidecars, crop coverage and every displayed pixel: 2,121,600 full-gallery pixels plus 326,400 summary pixels. Chromium checks all four HTML pages, 14 decoded images and every link. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-valen-eiland-spring-reactions.md) for native probes and isolated installation results.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_reactions -- --ignored'
nix-shell --pure --run 'cargo test --test eiland_world --test eiland_spring_actions --test eiland_spring_standard -- --include-ignored'
nix-shell --pure --run 'cargo clippy --test eiland_world --test eiland_spring_actions --test eiland_spring_standard --test eiland_spring_reactions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-reactions-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-reactions-trial
```

Local evidence is under `tmp/eiland-spring-reactions-author-*`: fresh metadata/source inventory, all-target sheets, literal source grids and material decisions, omission/spill red and final green logs, frozen hashes, byte-preservation and gallery/browser checks. Generated game images remain ignored. No runtime behavior changes are introduced.
