# Eiland's remaining Spring pose and tools

The user approved the offline artwork on 2026-09-29.

Seven strips complete all 47 Spring animations: princely-pose start/loop/end South, plus axe, pickaxe, brush and trowel East. They contain 41 source frames (29 distinct) and 29 native West mirrors, for 70 review cases. Eiland now has 125 pinned sources and 500 recolored variants, with Vanilla, Debug Blue, Hayden, Ryis and Seridia choices.

[Five-choice summary](../../generated/eiland-spring-finish-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-finish-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-finish-study`, with sources under `assets/animations/NPCs/Eiland/Sprites/Spring/spr_npc_eiland_specialanimation_spring_*.png`. Raw sidecars in `tmp/eiland-spring-finish-author-metadata.json` match independent archive reads. All strips use 80×80 frames, atlas `Default`, and offset `Middle`/54. The princely pose is a South-only complex cycle with 8/1/3 frames; all four tools use East linear Single packs with native West mirroring. The archive SHA-256 remains `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

All 41 actual frames were inspected in Vanilla and all four targets. The 703 new component seeds select 1,476 skin pixels per target, including fingers around shafts, the rose-holding fingertip, forehead/jaw shading and trowel knuckles. The existing five world skin shades and their roles remain unchanged. Gold belt and moving cape-hem components account for 166 protected pixels sharing those source colors. Pink hair, eyes, rose, uniform, sleeves and boots remain original. The axe's warm decorative bands, pickaxe's gold head, brush bristles, gray trowel and blue/white tool trails use distinct material shades and remain unchanged.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Axe East | 21, 46, 44, 36, 34, 21 |
| Pickaxe East | 22, 45, 43, 35, 34, 22 |
| Brush East | 31, 32, 31, 32, 31, 31, 32 |
| Princely pose start South | 42, 37, 42, 44, 42, 42, 42, 42 |
| Princely pose loop South | 42 |
| Princely pose end South | 42, 37, 42 |
| Trowel East | 40, 35, 34, 35, 34, 35, 34, 35, 35, 40 |

`tests/eiland_spring_finish.rs` checks 85 literal source-art landmarks, every source-or-target pixel against a source-grid material inventory, per-frame counts, alpha, geometry, metadata, strict pins and equal selections across targets. Effective controls fail on an omitted fingertip in axe frame 2 `[53,44]` and recolored cape hem in pose-start frame 5 `[33,47]` (frame numbers here are one-based). The final test, all six retained opt-in tests and targeted Clippy pass. Retained test edits only update corpus paths and total counts.

All 118 prior region objects, pins, colors, groups and target maps remain unchanged; all 1,180 prior original/variant PNG and metadata files match `generated/characters-balor-summer-valen-heal-eiland-magnify-trial/characters/eiland` exactly. The 500 final variants strictly validate and match the inspected candidates. Selection SHA-256: `7d8d46b4781b6d1d81b466978b5642d8d6000b8eeae0dc03ce5820433d97723d`. Profile SHA-256: `757eac5a3e2e96a23fc97ecab1f637a0b6ea517c73fe18b486a1f2641b3ad65c`.

The gallery covers all 70 cases across 12 detail pages. Full-width crop `[0,16,80,44]` includes artwork bounds `[31,19,78,56]`, preserving long tool trails and mirrored West views. Exact checks verify bindings, source hashes, raw sidecars, crops, mirrors and 12,320,000 displayed gallery pixels plus 352,000 summary pixels. Chromium checks all 13 HTML pages, 71 decoded images and every link. Static previews do not establish native timing or natural scheduling; see [shared integration](balor-valen-summer-eiland-spring-finish.md) for those checks and isolated installation.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_finish -- --ignored'
nix-shell --pure --run 'cargo clippy --test eiland_spring_finish -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-finish-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-finish-trial
```

Local evidence is under `tmp/eiland-spring-finish-author-*`: fresh metadata, source grids, all-target sheets, material decisions, red/green logs, frozen hashes, preservation and gallery/browser checks. Generated game images remain ignored; portrait-only recipes and runtime behavior remain unchanged.
