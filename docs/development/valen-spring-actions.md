# Valen's Spring everyday actions

Eleven strips add blink East/South and sit, eat and drink North/South/East.
They contain 31 source frames plus 12 native West mirrors: 43 review cases.
The world profile grows from 98 to 109 sources and 436 variants. All prior
regions, source pins, 11 color roles and six color groups remain exact.
The preset set is unchanged; the Debug Blue description now refers to
reviewed Spring overworld shading. Its mappings remain unchanged.
The user approved this artwork on 2026-09-29.

Fresh sources are under `assets/animations/NPCs/Valen/Sprites/Spring/`, named
`spr_npc_valen_spring_{blink,sit,eat,drink}_{north,south,east}.png`, with no
North blink. The corpus is `extracted/valen-spring-actions-study`. All eleven
raw sidecars in `tmp/valen-spring-actions-author-metadata.json` match the
independent archive reads and retain 80×80 frames, Default atlas and
Middle/54 origin. The read-only archive remains
`tmp/fields-of-mistria/assets.zip`, freshly confirmed SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 675 new component seeds select 1,242 skin pixels per target. Face, closed
eyelids, ears, neck, moving hands and exposed ankles change. Goggles, hair,
eye whites, black eye lines, open-mouth red/dark red, near-black outlines,
coat, cuffs, vest, belt, pants and shoes stay original. No new shade, material
exclusion or connected-edge compromise is needed. North sitting exposes no
skin; North eating/drinking expose two hand pixels per frame and have
byte-identical source artwork. Every actual frame was inspected in Vanilla
and all four target palettes using source grids and enlarged sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 51, 61, 51 |
| Blink South | 58, 67, 58 |
| Sit North / South / East | 0 / 51 / 41 |
| Eat North | 2, 2, 2 |
| Eat South | 50, 55, 55, 70, 51 |
| Eat East | 38, 43, 45, 49, 40 |
| Drink North | 2, 2, 2 |
| Drink South | 52, 62, 52 |
| Drink East | 41, 48, 41 |

- [Five-choice summary](../../generated/valen-spring-actions-preview/summary.png):
  second-frame blink South, third-frame eat South and second-frame drink East.
- [Complete Vanilla/Blue review](../../generated/valen-spring-actions-preview/blue-review/index.html):
  every source frame and West view, with full sprites and enlarged details.
- [Shared batch review](../../generated/balor-valen-eiland-spring-actions-preview/index.html):
  links to the individual Balor, Valen and Eiland reviews from this same batch.

The gallery shows NPC source strips; separate held food and cups are not
rendered in these images. The full review has 18 small sheets across 20 HTML
pages. Its directory is 787,964 bytes, with a largest file of 80,292 bytes.
Checks cover 7,766,304 exact preview pixels, 86 full-case source/palette
bindings plus enlarged details, and 15 summary bindings. The full crop
`[25,20,32,39]` contains every opaque pixel. Raw metadata, frame counts and
exact West reversal also pass. Chromium loaded every local link and image
without horizontal overflow. Evidence is recorded in
`tmp/valen-spring-actions-author-preview.log`,
`tmp/valen-spring-actions-author-preview-browser-check.json` and the gallery's
`coverage.json`.

The new local corpus test, retained pilot tests and targeted Clippy passed.
The new test checks 67 literal material landmarks, every pixel in four
targets, exact per-frame counts, alpha and metadata. Four omission controls
remove forehead, raised finger, isolated eyelid-corner and ankle components;
a separate control deliberately maps mouth red and proves it changes the
protected mouth pixels after successful generation. The retained test changed
only its corpus path and current source count; all earlier assertions remain.
Run the focused checks with:

```sh
nix-shell --pure --run 'cargo test --test valen_spring_actions --test valen_world -- --include-ignored && cargo clippy --test valen_spring_actions --test valen_world -- -D warnings'
```

All 436 final variants passed strict recipe validation. All 980 previous
original/variant PNG and metadata files remain byte-identical, and all 872
final variant PNG/metadata files match the inspected candidates. Frozen
input hashes and evidence are in
`tmp/valen-spring-actions-author-frozen-inputs.sha256`,
`tmp/valen-spring-actions-author-focused.log` and
`tmp/valen-spring-actions-author-preservation.log`.

See [shared integration](balor-valen-eiland-spring-actions.md) for the native
seated flags, timing/holds and isolated installation. Static previews do not
establish live timing, scheduling or transitions. Artwork stays ignored;
this character slice changes no runtime code. Other actions and outfits
remain for later batches.
