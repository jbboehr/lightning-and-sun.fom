# Reina Autumn standing and seated writing

Six strips add the start, loop and end of standing and seated writing. They
contain 16 South-facing source frames, with no West mirrors. Reina's world
profile grows from 201 to 207 sources and produces 828 variants. Choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_specialanimation_autumn_write_{start,loop,end}_south.png` and
`spr_npc_reina_specialanimation_autumn_write_sit_{start,loop,end}_south.png`.
The full corpus is `extracted/reina-autumn-specials-study`; fresh raw sidecars
are in `tmp/reina-autumn-specials-author-metadata.json`. All six match the
independent archive inventory and retain 80×80 frames and Default atlas.
Standing writing keeps numeric horizontal origin `40.0`; seated writing keeps
`Middle`. Both retain vertical `54.0`. Starts have two frames at 0.125 seconds;
loops have four with `[0.1,0.125,0.1,0.3]`; ends have two with `[0.125,0.1]`.
Both native cycles are complex and South-only; `write_sit` is seated. Source
pins remain strict, with no earlier pin refreshed or relaxed.

The 366 new component seeds select 762 skin pixels per target: faces, exposed
necklines and hands around the pencil and clipboard. All 16 actual frames were
inspected in Vanilla and all four targets. Orange pencil colors, pale wood and
point, clipboard backing `#B28159`, dark edge `#57342B`, paper and metal clip
remain original. Checkered clothing, trousers, gold trim, boots, hair, eyes and
outlines also remain original. No source colors, groups, mappings or material
exclusions were added. Earlier Autumn walking boot exclusions are unchanged.

| Strip | Skin pixels per frame |
| --- | --- |
| Standing-writing start | 52, 43 |
| Standing-writing loop | 47, 48, 49, 49 |
| Standing-writing end | 43, 52 |
| Seated-writing start | 52, 43 |
| Seated-writing loop | 46, 47, 48, 48 |
| Seated-writing end | 43, 52 |

- [Five-choice summary](../../generated/reina-autumn-specials-preview/summary.png):
  second-frame standing and seated start/loop samples.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-specials-preview/blue-review/index.html):
  all 16 cases and 32 views across two pages of eight cases.

The focused corpus test and targeted Clippy passed. Literal material landmarks
and full-pixel checks cover skin omissions, protected materials, per-frame
counts, alpha, metadata, common masks and all previous outputs.
`FOM_REINA_AUTUMN_SPECIALS_PRESETS` supports ignored control copies. Removing
the hand-highlight seed `[116,46]` from standing-writing loop fails the material
check. An ignored clipboard-brown alias and seed `[46,46]` in standing-writing
start also fails that check. Both controls reach the intended assertion after
successful generation and leave production recipes unchanged. Logs are
`tmp/reina-autumn-specials-author-focused.log` and
`tmp/reina-autumn-specials-author-{omission,spill}.log`.

All 828 final variants passed exact recipe validation. All 201 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,010 earlier
original and variant PNG/metadata files match the accepted Autumn reading
baseline byte for byte; all 48 new variant files match the inspected candidate.
Raw metadata matches the independent archive read, and the canonical Debug Blue
recipe matches the preset set. Evidence is in
`tmp/reina-autumn-specials-author-compare.json`,
`tmp/reina-autumn-specials-author-final-vs-candidate.json` and
`tmp/reina-autumn-specials-author-frozen.json`.

The complete preview is 263,446 bytes, about 257 KiB. Crop `[30,27,22,28]`
contains every opaque source pixel, including the pencil and clipboard.
Saved-image checks compared 3,400,320 exact rendered pixels across all-target
evidence, review sheets and 20 summary bindings. Checks include source/palette
identity, raw metadata and zero mirrored cases. Chromium checked all four HTML
pages, all cases, image decodes and local links, with no horizontal overflow.
Reports are `tmp/reina-autumn-specials-author-preview.log`,
`tmp/reina-autumn-specials-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-autumn-specials.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Autumn chopping and polishing remain.
