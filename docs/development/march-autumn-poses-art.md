# March Autumn standing pose artwork

This slice adds only the three Autumn standing poses: North, South and East. Each source strip contains one 80×80 frame on `Default`, with numeric horizontal origin `40.0`, vertical origin `54.0` and native default timing. The native horizontal pack can also render East mirrored as West. Fourteen Autumn injured strips remain outside this slice.

[Five-choice summary](../../generated/march-autumn-poses-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-autumn-poses-preview/blue-review/index.html)

All three actual frames were inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. Raised hands retain their disconnected dark finger edges; face, ear and neck shading follows the selected palette. The brown jacket and cuffs, pale shirt, scarf/headband, goggles, hair, eyes, trousers and footwear remain original. No new color role or special boundary exception is needed.

The source inventory uses `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14` as skin in these three strips. Sixty explicit component seeds select 98 pixels per target: East 38, North 14 and South 46. The profile now contains 311 pinned sources. All 308 earlier region objects, source pins, colors, groups, palette mappings and 3,080 previous PNG/metadata files remain exact.

The ignored corpus test `march_autumn_poses_cover_moving_skin_and_preserve_materials` checks 30 literal source landmarks, every new source-or-target pixel, alpha, dimensions, raw metadata, equal selections across four targets, every source hash and earlier output bytes. Two temporary controls were observed failing at the intended assertions: removing the deep hand component at East `[36,44]`, and selecting jacket color `#55423B` at East `[38,42]`. The corrected candidate then passed, along with targeted Clippy and formatting.

The final standalone contains 1,244 variants. Each strictly validates and matches the inspected candidate bytes. The gallery binds every source frame and native West mirror to the actual final bundle; a symmetric crop `[28,24,24,34]` contains all nontransparent artwork (source bounds `[31,27,48,54]`). Exact preview checks cover every displayed pixel and source hash, metadata, frame/palette binding and mirror. Chromium checks both HTML pages, five decoded images and all links. These static checks do not claim an interactive game playtest or natural scheduler dispatch of a West pose.

Local evidence is under `tmp/march-autumn-poses-author-*`, with raw exported sidecars in `metadata.json`, selection inventory in `audit.json`, focused test/control logs, frozen input hashes and preview/browser results. Original game assets, generated outputs and helpers stay ignored. The read-only archive SHA-256 is `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.

To rerun the focused corpus check after exporting the local assets:

```sh
nix-shell --pure --run 'cargo test --test march_autumn_poses -- --ignored'
```

The user approved this artwork for commit on 2026-09-27.
