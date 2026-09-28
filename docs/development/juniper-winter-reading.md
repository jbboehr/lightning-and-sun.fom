# Juniper's Winter seated reading sprites

This slice adds the three Winter `read_sit` phases: start, loop and end, with
3, 4 and 3 South-facing frames. Juniper now has 264 sources and 1,056 variants.
All 261 earlier regions and pins, seventeen source colors, nine color groups,
target mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-reading-study`. All 522 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-winter-standard-trial` bundle. Pins remain
strict.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_specialanimation_winter_read_sit_`. All retain 80×80 frames,
Default atlas and origin `Middle,54.0`. Start/end use `0.1` frame durations;
loop retains `[3.0,0.1,3.0,0.1]`. Raw sidecars at
`tmp/juniper-winter-reading-author-metadata.json` match independent archive
reads. The shared slice checks the native seated complex cycle separately.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
closed-eye skin and their shadows change. The cream pages, page shadows, pink
book cover, gloves, cuffs, shoulder decorations, circlet, hair, cosmetics and
warm clothing details remain original. No new source roles or coupled-material
exceptions are needed. Earlier accepted exceptions remain unchanged.

Fresh Winter material landmarks include:

- Start frame 1 changes cheek shadow `[37,39]` (`763F21`) and ear `[35,36]`
  (`EFD89A`), preserving cuff `[34,45]` (`BC8B43`) and glove `[34,46]`
  (`8A2C5F`). The dark Winter outfit fully covers the torso.
- Start frame 2 changes closed-eye skin `[37,36]` (`E3BF7F`) and cheek
  `[38,39]` above the raised book. Page highlight `[39,41]` (`F6E4D7`), page
  shadow `[38,41]` (`C9AF9C`) and cosmetics `[37,37]` (`FC639B`) stay original.
- Loop frames 1 and 3 follow the turned face. Jaw `[38,39]` in frame 1 and
  `[41,39]` in frame 3 (`763F21`) change, while shoulder `[35,40]` (`BC8B43`)
  and the wide pages below retain their colors.
- End frame 1 preserves the page beside the face `[35,40]` (`F6E4D7`) and
  the small glove beyond the book `[45,46]` (`8A2C5F`). End frame 3 restores
  the same protected cuffs and gloves as the starting seated pose.

Frame numbers here are one-based; coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Start South | 22, 16, 22 |
| Loop South | 18, 26, 18, 26 |
| End South | 26, 12, 22 |

The masks add 86 seeds and recolor 208 pixels per target. Another 64
matching-color jewelry pixels remain protected. Evidence:
`tmp/juniper-winter-reading-author-refined-components.json`,
`tmp/juniper-winter-reading-author-exclusions.json`,
`tmp/juniper-winter-reading-author-inventory.log` and the three
`tmp/juniper-winter-reading-author-refined-art-*.png` sheets. Profile SHA-256:
`47287945896c02e51331e0e083280e8a99359e49d3aa1b01645d10932accd298`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-reading-preview/summary.png)
shows frame 1 of each phase. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-reading-preview/blue-review/index.html)
shows all ten South-facing source frames across three detail pages, with
additional enlarged face views. A visible note identifies the preserved book,
gloves and jewelry.

Crop `[25,20,32,39]` includes every visible pixel. Exact checks verified
2,349,600 preview pixels, twenty full bindings, fifteen summary bindings,
frame counts and raw metadata. Chromium checked all five pages, including both
indexes: local links and images loaded without horizontal overflow. Evidence:
`tmp/juniper-winter-reading-author-preview-check.log` and
`tmp/juniper-winter-reading-author-preview-browser-check.json`.

The candidate material test passed, followed by formatting, targeted Clippy and
the final focused test in `tests/juniper_winter_reading.rs` (25.95 seconds).
It checks every new pixel in four targets, per-frame counts, unchanged metadata,
49 literal material landmarks and all 2,088 earlier variant PNG/metadata files.
Three effective controls omit a cheek-shadow component, omit an ear component
and remove region restrictions, exposing missed skin and accidental circlet,
shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-reading-author-candidate-test.log` and
`tmp/juniper-winter-reading-author-focused-checks.log`.

All 1,056 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,610 earlier original/variant PNG and metadata files
remain byte-identical; all 2,112 final variant files match the inspected
candidates. Evidence: `tmp/juniper-winter-reading-author-preservation.log`,
`tmp/juniper-winter-reading-author-stylized-validation.json` and
`tmp/juniper-winter-reading-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
