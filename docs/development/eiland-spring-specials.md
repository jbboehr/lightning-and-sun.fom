# Eiland's Spring standing and seated writing

Six strips add standing and seated writing start, loop and end phases. They contain 16 South-only source frames (12 distinct images). Eiland now has 112 pinned sources and 448 recolored variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved the offline artwork on 2026-09-29.

[Five-choice summary](../../generated/eiland-spring-specials-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-specials-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-specials-study`. Sources are under `assets/animations/NPCs/Eiland/Sprites/Spring/`, named `spr_npc_eiland_specialanimation_spring_write_{start,loop,end}_south.png` and `spr_npc_eiland_specialanimation_spring_write_sit_{start,loop,end}_south.png`. Raw sidecars in `tmp/eiland-spring-specials-author-metadata.json` match independent archive reads. Frames are 80×80, atlas `Default`, offset `Middle`/54. Both starts have two frames at 0.125 seconds; loops have four frames at `[0.1,0.125,0.1,0.3]`; ends have two frames at `[0.125,0.1]`. The complex cycles are South-only; the gallery introduces no West views. The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

Existing source colors, groups and target mappings remain unchanged. The five world skin shades are `#E9A980`, `#DE8F5D`, `#BA6A4C`, `#9C5241` and `#7D3B14`. All 106 earlier region objects and strict pins are preserved, and all 1,060 earlier original/variant PNG and metadata files match `generated/characters-balor-valen-eiland-spring-reactions-trial/characters/eiland` byte-for-byte. Portrait-only recipes remain untouched.

All 16 actual frames were inspected in Vanilla and all four targets. The 252 new component seeds select 538 skin pixels per target: forehead, jaw and closed-eye shading, the moving writing hand and the fingers holding the writing surface. Thirty-one shared `#BA6A4C` pixels remain original as cape, belt and uniform trim. Pink hair, gray/white eyes, sleeves and boots stay original. The orange quill, its pale tip, and the brown writing surface use distinct material colors and remain unchanged. In particular, warm `#F9AB6C`, `#FAB680` and `#FFD8D1` belong to the quill, while `#B28159` and `#57342B` shade the writing surface. Existing portrait shade `#6A3126` remains clothing trim here.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Standing writing start | 34, 30 |
| Standing writing loop | 36, 35, 36, 36 |
| Standing writing end | 30, 34 |
| Seated writing start | 34, 30 |
| Seated writing loop | 35, 34, 35, 35 |
| Seated writing end | 30, 34 |

`tests/eiland_spring_specials.rs` checks 69 literal source-art landmarks, every source-or-target pixel using an independent material inventory, per-frame counts, alpha, geometry, raw metadata, strict hashes, equal selections across four targets and all 1,060 prior files. Both deliberate controls fail at their intended assertions: missing fingertip shadow in standing writing loop frame 1 `[36,46]`, and incorrectly mapped gold trim in standing writing start frame 1 `[37,46]`. The final corpus test, all four retained Eiland tests and targeted Clippy pass. Retained tests change only expanded corpus paths and total counts.

All 448 final variants strictly validate and match the inspected candidates. Common new-frame selection SHA-256: `c20e7f3c099bb73fc40943b55e4a5f529f39de3a9c71ed428bcd47afd0a93c9d`. Profile SHA-256: `a599e9aea991287b08c34aed819ce549ffd9913fbf09925f7ab7c3e79c4bac6a`.

The compact gallery covers all 16 cases on three detail pages. Crop `[28,22,24,34]` contains the full artwork bounds `[30,27,51,54]`. Exact checks verify source/palette/frame bindings, hashes, raw sidecars, crop coverage and every displayed pixel: 2,611,200 full-gallery pixels plus 326,400 summary pixels. Chromium checks all four HTML pages, 17 decoded images and every link. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-valen-eiland-spring-specials.md) for native probes and isolated installation results.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_specials -- --ignored'
nix-shell --pure --run 'cargo test --test eiland_world --test eiland_spring_actions --test eiland_spring_standard --test eiland_spring_reactions -- --include-ignored'
nix-shell --pure --run 'cargo clippy --test eiland_world --test eiland_spring_actions --test eiland_spring_standard --test eiland_spring_reactions --test eiland_spring_specials -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-specials-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-specials-trial
```

Local evidence is under `tmp/eiland-spring-specials-author-*`: fresh metadata/source inventory, all-target sheets, literal source grids and material decisions, omission/spill red and final green logs, frozen hashes, byte-preservation and gallery/browser checks. Generated game images remain ignored. No runtime behavior changes are introduced.
