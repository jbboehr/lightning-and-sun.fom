# Valen's first overworld batch

Six Spring idle/walk strips add North, South and East, with 15 source frames
and five native West mirrors. Separate world definitions preserve all 92
accepted portrait regions and extend them to 98 sources and 392 variants.
The portrait-only profile, preset set and Debug Blue recipe remain unchanged.
The user approved this artwork on 2026-09-29.

Sources are under `assets/animations/NPCs/Valen/Sprites/Spring/`, named
`spr_npc_valen_spring_{idle,walk}_{north,south,east}.png`. Fresh exports are in
`extracted/valen-world-study`; raw sidecars are recorded in
`tmp/valen-world-author-metadata.json`. All six match the independent archive
read and retain 80×80 frames, Default atlas and Middle/54 origin. Idle uses
single-frame defaults; walk has four frames at 0.15 seconds each. The read-only
archive is `tmp/fields-of-mistria/assets.zip`, freshly confirmed SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All earlier source pins remain strict and unchanged.

Three additional world shades, `#FBD3A7`, `#EFA67A` and `#C37555`, map to the
existing highlight, middle and shadow targets (indices 0, 1 and 2). Each gets
a separate appended color group. The existing darkest `#762E21` keeps its
portrait role at index 4. All eight prior source roles and three original
groups remain exact; the world definitions have 11 roles and six groups.
Every portrait output remains identical in all four target palettes.

The 284 new component seeds select 514 skin pixels per target. Face, ears,
neck, hands and exposed ankles change. Hair, eye whites and pupils, goggles,
coat, vest, cuffs, belt, pants and shoes retain their original colors. North
views contain no exposed skin, so their empty seed lists are intentional.
The warm goggles use distinct gold shades; no connected-material compromise
or material exclusion is needed. Every actual frame was inspected in Vanilla
and all four targets using source grids and enlarged all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle North / South / East | 0 / 55 / 48 |
| Walk North | 0, 0, 0, 0 |
| Walk South | 55, 55, 55, 55 |
| Walk East | 48, 49, 48, 46 |

- [Five-choice summary](../../generated/valen-world-preview/summary.png): idle
  South, second-frame walk East and fourth-frame walk North.
- [Complete Vanilla/Blue review](../../generated/valen-world-preview/blue-review/index.html):
  all 20 cases, with full sprites and enlarged details, across eight small sheets.

The full preview directory is 407,754 bytes; the largest file is 58,089 bytes.
All sheets use the final standalone bundle. The full crop `[25,20,32,39]`
contains every opaque pixel, including mirrored views. Checks compare
3,862,560 exact preview pixels, 40 full-case source/palette bindings, enlarged
details and 15 summary bindings. They also check source metadata, frame
counts and exact West reversal. Chromium loaded all local links and images
across ten HTML pages without horizontal overflow. Evidence is in
`tmp/valen-world-author-preview.log`,
`tmp/valen-world-author-preview-browser-check.json` and the preview's
`coverage.json`.

Both focused tests and targeted Clippy passed. The normal test asserts the
exact preserved portrait definitions and append-only world roles. The local
corpus test checks 63 literal skin/material landmarks, every pixel in all
four targets, exact per-frame counts, alpha and metadata. Four controls omit
forehead, ear, wrist and ankle components and expose the intended unchanged
skin pixel; another deliberately adds gold to the palette and demonstrates
goggle recoloring. Each control applies successfully before its pixel
assertions. Run the focused checks with:

```sh
nix-shell --pure --run 'cargo test --test valen_world -- --include-ignored && cargo clippy --test valen_world -- -D warnings'
```

All 392 final variants passed strict recipe validation. All 920 accepted
portrait original/variant PNG and metadata files remain byte-identical,
and all 784 final variant PNG/metadata files match the inspected candidates.
The canonical Debug Blue recipe matches every role in the preset set.
Frozen recipe/test hashes are recorded in
`tmp/valen-world-author-frozen-inputs.sha256`; focused evidence is in
`tmp/valen-world-author-focused.log`.

See [shared integration](balor-valen-eiland-world.md) for native animation
selection and isolated installation. Static previews do not establish live
animation timing, scheduling or transitions. Artwork stays ignored, and this
character slice changes no runtime code. Other actions and outfits remain
for later batches.
