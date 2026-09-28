# Reina Winter standing and seated writing

Six strips add the start, loop and end of standing and seated writing. They
contain 16 South-facing source frames, with no West mirrors. Reina's world
profile grows from 236 to 242 sources and produces 968 variants. Choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork
for commit on 2026-09-28.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_specialanimation_winter_write_{start,loop,end}_south.png` and
`spr_npc_reina_specialanimation_winter_write_sit_{start,loop,end}_south.png`.
The full corpus is `extracted/reina-winter-specials-study`; fresh raw sidecars
are in `tmp/reina-winter-specials-author-metadata.json`. All six retain 80×80
frames and Default atlas. Standing writing keeps numeric horizontal origin
`40.0`; seated writing keeps `Middle`. Both retain vertical `54.0`. Starts
have two frames at 0.125 seconds; loops have four with `[0.1,0.125,0.1,0.3]`;
ends have two with `[0.125,0.1]`. Both native cycles are complex and South-only;
`write_sit` is seated. Source pins remain strict, with no earlier pin refreshed
or relaxed.

The 348 new component seeds select 706 skin pixels per target: faces and hands
around the pencil and clipboard. All 16 actual frames were inspected in Vanilla
and all four targets. Orange pencil colors, pale wood and point, clipboard
backing `#B28159`, dark edge `#57342B`, paper and metal clip remain original.
Winter sleeves, trousers, boots, hair, eyes and outlines also remain original.
All 30 scarf pixels using portrait skin shade `#9E512F` stay excluded. No source
colors, groups or mappings changed.

The first standing-writing loop frame has a three-pixel hand highlight beginning
at `[37,45]`; nearby sleeve `[36,44]` and pencil wood `[39,44]` stay original.
The clipboard-holding hand at `[42,47]` remains distinct from its dark board
edge at `[40,47]`. Literal source landmarks cover these material boundaries and
all scarf pixels that share a portrait mapping.

| Strip | Skin pixels per frame |
| --- | --- |
| Standing-writing start | 48, 39 |
| Standing-writing loop | 45, 44, 45, 45 |
| Standing-writing end | 39, 48 |
| Seated-writing start | 50, 39 |
| Seated-writing loop | 44, 43, 44, 44 |
| Seated-writing end | 39, 50 |

- [Five-choice summary](../../generated/reina-winter-specials-preview/summary.png):
  second-frame standing and seated start/loop samples.
- [Complete Vanilla/Blue review](../../generated/reina-winter-specials-preview/blue-review/index.html):
  all 16 cases and 32 views across two pages of eight cases.

The focused corpus test and targeted Clippy passed. Literal material landmarks
and full-pixel checks cover skin omissions, protected materials, per-frame
counts, alpha, metadata, common masks and all previous outputs.
`FOM_REINA_WINTER_SPECIALS_PRESETS` supports ignored control copies. Removing
the three-pixel hand-highlight seed `[37,45]` from standing-writing loop fails
the material check. Selecting the scarf pixel `[41,42]`, which already has a
portrait mapping, also fails that check. Both controls reach the intended
assertion after successful generation, need no synthetic alias, and leave
production recipes unchanged. Logs are
`tmp/reina-winter-specials-author-focused.log` and
`tmp/reina-winter-specials-author-{omission,spill}.log`.

All 968 final variants passed exact recipe validation. All 236 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,360 earlier
original and variant PNG/metadata files match the accepted Winter reading
baseline byte for byte; all 48 new variant files match the inspected candidate.
Raw metadata matches the independent archive read, and the canonical Debug Blue
recipe matches the preset set. Evidence is in
`tmp/reina-winter-specials-author-compare.json`,
`tmp/reina-winter-specials-author-final-vs-candidate.json` and
`tmp/reina-winter-specials-author-frozen.json`.

The complete preview is 265,374 bytes, about 259 KiB. Crop `[30,27,22,28]`
contains every opaque source pixel, including the pencil and clipboard.
Saved-image checks compared 3,400,320 exact rendered pixels across all-target
evidence, review sheets and 20 summary bindings. Checks include source/palette
identity, raw metadata and zero mirrored cases. Chromium checked all four HTML
pages, all cases, image decodes and local links, with no horizontal overflow.
Reports are `tmp/reina-winter-specials-author-preview.log`,
`tmp/reina-winter-specials-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-winter-specials.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Winter chopping and polishing remain.
