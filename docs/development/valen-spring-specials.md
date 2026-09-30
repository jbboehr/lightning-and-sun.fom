# Valen's Spring standing and seated writing sprites

Six strips add the start, loop and end phases of standing and seated writing.
They contain 16 South-facing source frames: 2/4/2 for each cycle. The world
profile grows from 120 to 126 sources and 504 variants. All prior region
objects, source pins, 11 color roles and six color groups remain exact;
the preset set and canonical Debug Blue recipe are unchanged.
The user approved the offline artwork on 2026-09-29.

Fresh sources are under `assets/animations/NPCs/Valen/Sprites/Spring/`, named
`spr_npc_valen_specialanimation_spring_write_{start,loop,end}_south.png` and
`spr_npc_valen_specialanimation_spring_write_sit_{start,loop,end}_south.png`.
The corpus is `extracted/valen-spring-specials-study`. All six raw sidecars
in `tmp/valen-spring-specials-author-metadata.json` match the independent
archive reads and retain 80×80 frames, Default atlas and numeric 40/54 origin.
The read-only archive remains `tmp/fields-of-mistria/assets.zip`, freshly
confirmed SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 414 new component seeds select 779 skin pixels per target. Faces, necks,
writing fingers, supporting hands, wrists and exposed ankles change. The
blue quill, pale writing surface, blue/red prop details, eyes, goggles, hair,
clothing and shoes stay original. The quill's `#B6CBF7` highlight, `#799ADD`
blue and `#3F3F74` shadow differ from skin. Its blue color remains visible
in every palette, including Vanilla. The writing surface uses `#F5F5F5`
and `#C3D1DD`; adjacent prop details use `#6E8FBA` and `#933D52`. No new
palette role, material exclusion or connected edge compromise is needed.
Every actual frame was inspected in Vanilla and all four targets using
source grids and enlarged sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Standing writing start | 58, 45 |
| Standing writing loop | 47, 51, 48, 48 |
| Standing writing end | 45, 58 |
| Seated writing start | 57, 43 |
| Seated writing loop | 44, 45, 45, 45 |
| Seated writing end | 43, 57 |

- [Five-choice summary](../../generated/valen-spring-specials-preview/summary.png):
  first-frame standing start, second-frame standing loop and third-frame seated loop.
- [Complete Vanilla/Blue review](../../generated/valen-spring-specials-preview/blue-review/index.html):
  every frame of both South-facing cycles, with full sprites and enlarged details.
- [Shared batch review](../../generated/balor-valen-eiland-spring-specials-preview/index.html):
  links to all three individual reviews from this same batch.

The complete review contains six sheets across eight HTML pages. Its
directory is 348,571 bytes, with a largest file of 59,168 bytes. Checks compare
3,183,648 exact preview pixels, 32 full-case source/palette bindings plus
details, and 15 summary bindings. The full crop `[25,20,32,39]` contains every
opaque pixel; source metadata and frame counts also match. Chromium loaded
every local link and image without horizontal overflow. Evidence is in
`tmp/valen-spring-specials-author-preview.log`,
`tmp/valen-spring-specials-author-preview-browser-check.json` and the
preview's `coverage.json`.

The new material test, all four retained corpus tests and targeted Clippy
passed. The new test checks 63 literal skin/material landmarks, every pixel
in four targets, exact per-frame counts, alpha and metadata. Four omission
controls remove writing-finger, hand-shadow, supporting-hand and jaw
components. A separate control deliberately maps the writing-surface edge
shade and proves it changes two protected pixels after successful generation.
Retained tests changed only corpus paths and the current source count;
earlier assertions remain. Run the focused checks with:

```sh
nix-shell --pure --run 'cargo test --test valen_spring_specials --test valen_spring_reactions --test valen_spring_standard --test valen_spring_actions --test valen_world -- --include-ignored && cargo clippy --test valen_spring_specials --test valen_spring_reactions --test valen_spring_standard --test valen_spring_actions --test valen_world -- -D warnings'
```

All 504 final variants passed strict recipe validation. All 1,200 earlier
original/variant PNG and metadata files remain byte-identical, and all 1,008
final variant PNG/metadata files match the inspected candidates. Frozen
hashes and evidence are in
`tmp/valen-spring-specials-author-frozen-inputs.sha256`,
`tmp/valen-spring-specials-author-focused.log`,
`tmp/valen-spring-specials-author-retained.log` and
`tmp/valen-spring-specials-author-preservation.log`.

See [shared integration](balor-valen-eiland-spring-specials.md) for native
phase selection, seated flags and isolated installation. Static previews
do not establish live timing, scheduling or transitions. Artwork stays
ignored; this character slice changes no runtime code. Healing, charm
and other outfits remain for later batches.
