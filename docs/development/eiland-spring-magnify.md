# Eiland's Spring magnifying-glass animations

Six strips add magnify start, loop and end phases facing South and East. They contain 12 distinct source frames and six native West mirrors, for 18 review cases. Eiland now has 118 pinned sources and 472 recolored variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved the offline artwork on 2026-09-29.

[Five-choice summary](../../generated/eiland-spring-magnify-preview/summary.png) · [Every-frame Vanilla/Debug Blue review](../../generated/eiland-spring-magnify-preview/blue-review/index.html)

The fresh corpus is `extracted/eiland-spring-magnify-study`. Sources are under `assets/animations/NPCs/Eiland/Sprites/Spring/`, named `spr_npc_eiland_specialanimation_spring_magnify_{start,loop,end}_{south,east}.png`. Raw sidecars in `tmp/eiland-spring-magnify-author-metadata.json` match independent archive reads. Frames are 80×80, atlas `Default`, offset `Middle`/54. Both starts have three frames at `[0.15,0.125,0.125]`; loops have one frame with default timing; ends have two frames at `[0.125,0.15]`. The native complex Multi pack provides South and East, with West rendered by mirroring East. North falls back to South. The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

Existing source colors, groups and target mappings remain unchanged. All 112 earlier region objects and strict pins are preserved, and all 1,120 earlier original/variant PNG and metadata files match `generated/characters-balor-valen-eiland-spring-specials-trial/characters/eiland` byte-for-byte. Portrait-only recipes remain untouched.

All 12 actual source frames were inspected in Vanilla and all four targets. The 193 new component seeds select 378 skin pixels per target, including moving fingers around the handle, forehead and jaw shading, and the lowered hand. Fifty-four shared-color pixels remain original as clothing trim. Pink hair, eyes, sleeves, cape, uniform and boots stay original. The magnifier uses distinct `#BE6D44` rim shadows, yellow `#FFCF36`/`#FFF672`, blue `#488DE1`/`#74D2FF` and white lens highlights; these remain unchanged.

One isolated `#DE8F5D` pixel at `[43,44]` in East start frame 1 is chest trim despite sharing a skin color. Comparing Summer's same pose places pink trim there, while Spring's next frame uses gold at the shifted sash endpoint. The separately visible far hand at `[45,47]` remains skin across Spring, Summer and Autumn. This material inference informs only the new exclusion; no earlier recipes change. The protected point and adjacent skin have literal regression coverage.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Start East | 37, 23, 24 |
| Loop East | 28 |
| End East | 17, 29 |
| Start South | 42, 45, 37 |
| Loop South | 25 |
| End South | 38, 33 |

`tests/eiland_spring_magnify.rs` checks 77 literal source-art landmarks, every source-or-target pixel using an independent material inventory, per-frame counts, alpha, geometry, raw metadata, strict hashes, equal selections across four targets and all 1,120 prior files. Deliberate controls fail at the intended assertions: missing fingertip shadow in South start frame 3 `[42,45]`, and incorrectly mapped chest trim in East start frame 1 `[43,44]`. The final corpus test, all five retained Eiland tests and targeted Clippy pass. Retained tests change only expanded corpus paths and total counts.

All 472 final variants strictly validate and match the inspected candidates. Common new-frame selection SHA-256: `2ec804f1b2d03c9840a3acbffd664ac5bb80f34c806742623df2fda5eb94dd3c`. Profile SHA-256: `de87cf97bbb31e906bd539d2cecfde4cfae5d9f07bdd6104ee1a62a4c220dffe`.

The compact gallery covers all 18 cases on three direction pages. Crop `[28,22,24,34]` contains the full artwork bounds `[31,27,49,54]` and is symmetric for West mirroring. Exact checks verify source/palette/frame bindings, hashes, raw sidecars, crop coverage, mirrors and every displayed pixel: 2,937,600 full-gallery pixels plus 326,400 summary pixels. Chromium checks all four HTML pages, 19 decoded images and every link. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-summer-valen-heal-eiland-magnify.md) for native probes and isolated installation results.

```sh
nix-shell --pure --run 'cargo test --test eiland_spring_magnify -- --ignored'
nix-shell --pure --run 'cargo test --test eiland_world --test eiland_spring_actions --test eiland_spring_standard --test eiland_spring_reactions --test eiland_spring_specials -- --include-ignored'
nix-shell --pure --run 'cargo clippy --test eiland_world --test eiland_spring_actions --test eiland_spring_standard --test eiland_spring_reactions --test eiland_spring_specials --test eiland_spring_magnify -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-spring-magnify-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-spring-magnify-trial
```

Local evidence is under `tmp/eiland-spring-magnify-author-*`: fresh metadata/source inventory, all-target sheets, source grids and material decisions, omission/spill red and final green logs, frozen hashes, byte-preservation and gallery/browser checks. The cross-outfit comparison is `tmp/bve-summer-heal-magnify-trim-compare.png`. Generated game images remain ignored. No runtime behavior changes are introduced.
