# Reina Beach bathing and swimming

Two strips add Beach bathing/swimming East and South. They contain eight source
frames and four native West mirrors, giving 12 review cases. Reina's world
profile grows from 258 to 260 sources and produces 1,040 variants. Choices
remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork on 2026-09-29.

Sources are under `assets/animations/NPCs/Reina/Sprites/Beach/`, named
`spr_npc_reina_beach_bath_swim_{east,south}.png`. The fresh corpus is
`extracted/reina-beach-swim-study`; raw sidecars are recorded in
`tmp/reina-beach-swim-author-metadata.json`. Both retain four 80×80 frames at
0.15 seconds, Default atlas and `Middle`/54 offsets. The archive hash remains
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Source hashes are strict; no earlier pin was refreshed or relaxed.

The 116 new component seeds select 234 skin pixels per target. Only the head is
visible; body and swimwear are submerged. All eight actual frames were inspected
in Vanilla and all four targets. Faces and the lower jaw recolor while hair,
eyes, outlines, blue water `#328BC9`, pale ripples/splashes `#9DEBFC` and the
original alpha remain untouched. All four existing world shades belong to skin
in these strips. No source colors, groups, mappings or material exceptions
were added.

Literal landmarks cover the two-pixel jaw shadow beginning at `[39,54]` in
South frame one, directly above the ripple at `[40,55]`. The third and fourth
frames lower the face by one pixel, hiding more of the jaw behind the water.
Separate checks retain detached splash droplets on both sides and below the
head, as well as the dark water edge.

| Strip | Skin pixels per frame |
| --- | --- |
| Bath/swim East | 30, 30, 25, 25 |
| Bath/swim South | 34, 34, 28, 28 |

- [Five-choice summary](../../generated/reina-beach-swim-preview/summary.png):
  South frames one/three and East frames two/four.
- [Complete Vanilla/Blue review](../../generated/reina-beach-swim-preview/blue-review/index.html):
  all 12 cases and 24 views across two pages.

The focused corpus test uses literal skin/material landmarks and full-pixel
checks for omissions, protected materials, per-frame counts, alpha, metadata,
common masks and previous outputs. `FOM_REINA_BEACH_SWIM_PRESETS` supports
ignored control copies. One removes the jaw-shadow seed `[39,54]` from South.
Another adds an ignored water alias and selects ripple `[40,55]`. Production
palette definitions retain their existing colors and mappings.

Both controls failed at the intended material assertion after successful
generation: omitted jaw shading at `[39,54]`, and recolored water beginning
at `[36,55]`. The production focused test and targeted Clippy passed. Logs are
`tmp/reina-beach-swim-author-focused.log` and
`tmp/reina-beach-swim-author-{omission,spill}.log`.

All 1,040 final variants passed exact recipe validation. All 258 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,580 earlier
original and variant PNG/metadata files match the accepted Beach actions
baseline byte for byte; all 16 new variant files match the visually inspected
candidate. Both raw sidecars match the independent archive read, and the
canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-beach-swim-author-compare.json`,
`tmp/reina-beach-swim-author-final-vs-candidate.json` and
`tmp/reina-beach-swim-author-frozen.json`.

The complete preview is 176,961 bytes, about 173 KiB. Crop `[30,42,20,21]`
contains every opaque source pixel, including the detached splash droplets.
Saved-image checks compared 1,562,400 exact rendered pixels across all-target
evidence, complete review sheets and 20 summary bindings, including
source/palette identity and exact West reversal. Chromium checked all four
HTML pages, all 12 cases, image decodes and local links, with no horizontal
overflow. Reports are `tmp/reina-beach-swim-author-preview.log`,
`tmp/reina-beach-swim-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-beach-swim.md) for native selection
and isolated installation. Artwork remains ignored; this slice changes no
runtime behavior. Static images do not establish animation timing, scheduling
or transitions.
