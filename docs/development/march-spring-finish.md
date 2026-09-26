# March Spring gestures and poses

Adds grumpy South, sigh South, pose North/South/East and side_look South: six strips, thirteen distinct source frames, plus one native West pose mirror. March now has 220 sources and 880 recolored variants. This finishes the normal Spring gestures and poses; the fourteen hurt animations remain outside this slice.

## Source and material boundaries

The archive remains read-only. Each PNG is pinned to its exact source SHA-256. All six strips use 80×80 frames, the `Default` atlas and numeric offset `[40,54]`. Grumpy has four durations `[0.1,0.1,0.125,1.0]`; side_look has three `[0.1,0.125,1.0]`; sigh has three `[0.15,0.1,1.0]`. Each pose is one frame with default timing. All four native cycles are linear and return to idle when speaking or background-paused. Pose supplies North/South/East packs; the native West handler mirrors the East pack.

The exposed face, neck and hands use existing `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. The six masks add 357 explicit component seeds selecting 586 pixels per target. No colors, groups or mappings change. The old 214 region objects, source pins and seeds remain exact.

Every actual frame was inspected in Vanilla and all four targets. Crossed fingers and forearms stay distinct from the green sleeves, and the small North-facing neck opening maps below the hair. Goggles, hair, apron, clothing and footwear stay original. Pink `#F27E7B` expression accents remain intact, as do the sigh mouth's dark `#410808` interior and red `#9E2626` tongue. These expression colors are distinct from the surrounding skin ramp.

## Verification

`tests/march_spring_finish.rs` contains thirty literal source-art landmarks across the folded arms, exposed hands, neckline, sleeves, apron, goggles, hair and expression details. The opt-in test checks all four target outputs against the independent skin/material inventory, including alpha, dimensions, raw metadata and strict source hashes. All 2,140 earlier original/variant PNG and metadata files are compared against `generated/characters-reina-juniper-march-specials-trial/characters/march`.

Temporary negative controls omit the crossed-arm shadow at grumpy frame 3, `[37,45]`, and recolor the green shirt at pose South `[39,43]`. Both failed at the corresponding literal landmark before the accepted candidate passed the full focused test. Targeted Clippy, formatting and the normal data test also passed. All 880 final variants strictly validate and match the reviewed candidate bytes. The common selection SHA-256 is `1a340cd00b21dcb093914b579fd21bf2a509c4b3613f7eb8b5e1763fce0c67c7`.

```sh
nix-shell --pure --run 'cargo test --test march_spring_finish'
nix-shell --pure --run 'cargo test --test march_spring_finish -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_finish -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-finish-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-finish-trial
```

Local previews:

- [Five-choice summary](../../generated/march-spring-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/march-spring-finish-preview/blue-review/index.html): thirteen source frames and the West pose mirror, across four detail pages.

The full crop `[28,24,24,32]` contains the complete source silhouette and is symmetric around the native mirror axis. Preview checks cover exact source/palette identity, hashes, raw metadata, each displayed pixel and the West mirror. The checker passed 2,150,400 full-review pixels and 307,200 summary pixels. Chromium passed all five pages, fifteen images and their links. Parent integration handles the combined package, native probes and installation separately. Game images and generated artifacts remain ignored and uncommitted.
