# Valen's Spring action, sleep and kiss sprites

Five strips add general action North/South/East, sleep East and kiss East.
They contain 26 source frames plus 12 native West mirror views: 38 review
cases. The world profile grows from 109 to 114 sources and 456 variants.
All earlier regions, source pins, 11 color roles and six color groups remain
exact; the preset set and canonical Debug Blue recipe are unchanged.
The user approved this artwork on 2026-09-29.

Fresh sources are under `assets/animations/NPCs/Valen/Sprites/Spring/`, named
`spr_npc_valen_spring_action_{north,south,east}.png`,
`spr_npc_valen_spring_sleep_east.png` and
`spr_npc_valen_spring_kiss_east.png`. The corpus is
`extracted/valen-spring-standard-study`. All five raw sidecars in
`tmp/valen-spring-standard-author-metadata.json` match the independent archive
reads and retain 80×80 frames, Default atlas and Middle/54 origin. Action has
seven frames with durations `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`;
kiss has four frames at `[0.15, 0.15, 0.8, 0.15]`; sleep uses single-frame
defaults. The read-only archive remains `tmp/fields-of-mistria/assets.zip`,
freshly confirmed SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 510 new component seeds select 927 skin pixels per target. Faces, closed
eyelids, ears, necks, moving hands and exposed ankles change. Goggles, hair,
eye whites, black eye lines, coat, cuffs, vest, belt, pants and shoes stay
original. North action exposes three hand pixels in its first frame and two
in its sixth; the other North frames have no exposed skin. Sleep includes
the supporting hand beside the face. No new shade, material exclusion or
connected-edge compromise is needed. Every actual frame was inspected in
Vanilla and all four targets using source grids and enlarged sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Action North | 3, 0, 0, 0, 0, 2, 0 |
| Action South | 52, 50, 50, 50, 50, 52, 55 |
| Action East | 43, 43, 42, 43, 42, 43, 48 |
| Sleep East | 50 |
| Kiss East | 45, 51, 56, 57 |

- [Five-choice summary](../../generated/valen-spring-standard-preview/summary.png):
  second-frame action East, sleep East and third-frame kiss East.
- [Complete Vanilla/Blue review](../../generated/valen-spring-standard-preview/blue-review/index.html):
  all 38 source-frame and West cases, with full sprites and enlarged details.
- [Shared batch review](../../generated/balor-valen-eiland-spring-standard-preview/index.html):
  links to all three individual reviews from this same batch.

West views use native NPC mirroring, including the sleep/kiss Single packs;
they do not establish natural directional dispatch. The sleep preview is an
unrotated source view and does not reproduce placement in a bed. The complete
review has 12 sheets across 14 HTML pages. Its directory is 697,388 bytes,
with a largest file of 71,623 bytes. Checks cover 6,917,664 exact preview
pixels, 76 full-case source/palette bindings plus details, and 15 summary
bindings. The full crop `[25,20,32,39]` contains every opaque pixel. Raw
metadata, frame counts and exact West reversal also pass. Chromium loaded
every local link and image without horizontal overflow. Evidence is in
`tmp/valen-spring-standard-author-preview.log`,
`tmp/valen-spring-standard-author-preview-browser-check.json` and the
preview's `coverage.json`.

The new material test, both retained corpus tests and targeted Clippy passed.
The new test checks 68 literal skin/material landmarks, every pixel in four
targets, exact per-frame counts, alpha and metadata. Four omission controls
remove the palm, isolated fingertip, hand outline and ankle shadow. A
separate control deliberately maps cuff cloth and proves it changes two
protected pixels after successful generation. Retained tests changed only
their corpus paths and current source count; earlier assertions remain.
Run the focused checks with:

```sh
nix-shell --pure --run 'cargo test --test valen_spring_standard --test valen_spring_actions --test valen_world -- --include-ignored && cargo clippy --test valen_spring_standard --test valen_spring_actions --test valen_world -- -D warnings'
```

All 456 final variants passed strict recipe validation. All 1,090 earlier
original/variant PNG and metadata files remain byte-identical, and all 912
final variant PNG/metadata files match the inspected candidates. Frozen
hashes and evidence are in
`tmp/valen-spring-standard-author-frozen-inputs.sha256`,
`tmp/valen-spring-standard-author-focused.log` and
`tmp/valen-spring-standard-author-preservation.log`.

See [shared integration](balor-valen-eiland-spring-standard.md) for native
selection, final-frame holds and isolated installation. Static previews do
not establish live timing, scheduling or transitions. Artwork stays ignored;
this character slice changes no runtime code. Other actions and outfits
remain for later batches.
