# Valen's Spring shocked and seated-reading sprites

Six strips add the start, loop and end phases of shocked and seated reading.
They contain 13 South-facing source frames: shocked 1/1/1 and reading 3/4/3.
The world profile grows from 114 to 120 sources and 480 variants. All prior
region objects, source pins, 11 color roles and six color groups remain
exact; the preset set and canonical Debug Blue recipe are unchanged.
The user approved this artwork on 2026-09-29.

Fresh sources are under `assets/animations/NPCs/Valen/Sprites/Spring/`, named
`spr_npc_valen_spring_shocked_{start,loop,end}_south.png` and
`spr_npc_valen_specialanimation_spring_read_sit_{start,loop,end}_south.png`.
The corpus is `extracted/valen-spring-reactions-study`. All six raw sidecars
in `tmp/valen-spring-reactions-author-metadata.json` match the independent
archive reads and retain 80×80 frames, Default atlas and Middle/54 origin.
The read-only archive remains `tmp/fields-of-mistria/assets.zip`, freshly
confirmed SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 331 new component seeds select 648 skin pixels per target. Faces, ears,
raised fingers, wrists, small jaw corners and exposed ankles change. Mouth
reds, eye whites and pupils, goggles, hair, clothing, book covers and pages
stay original. The book's cream `#F6E4D7` pages and shaded `#C9AF9C` page edges
are distinct from skin. Its darkest binding `#76404E` also differs from the
skin outline `#762E21`. No new palette role, material exclusion or connected
edge compromise is needed. Every actual frame was inspected in Vanilla and
all four targets using source grids and enlarged sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Shocked start / loop / end | 67 / 78 / 67 |
| Reading start | 48, 43, 42 |
| Reading loop | 36, 49, 36, 49 |
| Reading end | 51, 34, 48 |

- [Five-choice summary](../../generated/valen-spring-reactions-preview/summary.png):
  shocked loop, second-frame reading start and first-frame reading loop.
- [Complete Vanilla/Blue review](../../generated/valen-spring-reactions-preview/blue-review/index.html):
  every frame of both South-facing cycles, with full sprites and enlarged details.
- [Shared batch review](../../generated/balor-valen-eiland-spring-reactions-preview/index.html):
  links to all three individual reviews from this same batch.

The complete review contains six sheets across eight HTML pages. Its
directory is 298,384 bytes, with a largest file of 57,361 bytes. Checks compare
2,674,464 exact preview pixels, 26 full-case source/palette bindings plus
details, and 15 summary bindings. The full crop `[25,20,32,39]` contains every
opaque pixel; source metadata and frame counts also match. Chromium loaded
every local link and image without horizontal overflow. Evidence is in
`tmp/valen-spring-reactions-author-preview.log`,
`tmp/valen-spring-reactions-author-preview-browser-check.json` and the
preview's `coverage.json`.

The new material test, all three retained corpus tests and targeted Clippy
passed. The new test checks 62 literal skin/material landmarks, every pixel
in four targets, exact per-frame counts, alpha and metadata. Four omission
controls remove forehead, isolated forehead corner, hand and hand outline
components. A separate control deliberately maps the book-page shade and
proves it changes two protected page pixels after successful generation.
Retained tests changed only corpus paths and the current source count;
earlier assertions remain. Run the focused checks with:

```sh
nix-shell --pure --run 'cargo test --test valen_spring_reactions --test valen_spring_standard --test valen_spring_actions --test valen_world -- --include-ignored && cargo clippy --test valen_spring_reactions --test valen_spring_standard --test valen_spring_actions --test valen_world -- -D warnings'
```

All 480 final variants passed strict recipe validation. All 1,140 earlier
original/variant PNG and metadata files remain byte-identical, and all 960
final variant PNG/metadata files match the inspected candidates. Frozen
hashes and evidence are in
`tmp/valen-spring-reactions-author-frozen-inputs.sha256`,
`tmp/valen-spring-reactions-author-focused.log`,
`tmp/valen-spring-reactions-author-retained.log` and
`tmp/valen-spring-reactions-author-preservation.log`.

See [shared integration](balor-valen-eiland-spring-reactions.md) for native
phase selection, the seated reading flag and isolated installation. Static
previews do not establish live timing, scheduling or transitions. Artwork
stays ignored; this character slice changes no runtime code. Other actions
and outfits remain for later batches.
