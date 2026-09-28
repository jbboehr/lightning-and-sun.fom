# Juniper's Winter laugh sprites

This slice adds the Winter laugh start, loop and end strips, with 1, 2 and 2
South-facing frames. Juniper now has 267 sources and 1,068 variants. All 264
earlier regions and pins, seventeen source colors, nine color groups, target
mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-specials-study`. All 528 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-winter-reading-trial` bundle. Pins remain strict.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_specialanimation_winter_laugh_`. All retain 80×80 frames,
Default atlas and numeric origin `40.0,54.0`. Start uses single-frame defaults;
loop durations are `[0.15,0.125]`, and end retains `0.125`. Raw sidecars are
recorded in `tmp/juniper-winter-specials-author-metadata.json` and match
independent archive reads. The shared slice checks native cycle behavior.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
closed-eye skin and their shadows change. The Winter gloves remain pink,
including the raised hand in front of the mouth. Circlet, cosmetics, shoulder
decorations, cuffs, hair and warm clothing details stay original. No new source
roles or coupled-material exceptions are needed; earlier accepted exceptions
remain unchanged.

Fresh Winter landmarks include:

- Start frame 1 changes cheek shadow `[37,40]` (`763F21`), ear `[35,37]`
  (`EFD89A`) and closed-eye skin `[37,36]` (`E3BF7F`). Circlet `[38,34]`
  (`E3BF7F`), shoulder `[35,41]` and cuff `[35,45]` (`BC8B43`) remain original.
- Loop frame 1 changes cheek `[37,36]` (`EFD89A`) above the glove, jaw
  `[42,40]` (`763F21`) and chin `[40,41]` (`BC8B43`). The mouth outline,
  glove `[38,39]` (`DD426C`), glove shadow `[37,39]` (`60285E`) and wrist
  gemstone `[36,41]` (`3CB9D8`) remain original.
- Loop frame 2 follows the lowered face and hand, changing jaw `[42,39]`
  (`763F21`) while preserving glove `[38,38]` (`DD426C`), shoulder `[44,40]`
  and opposite cuff `[46,43]` (`BC8B43`).
- End frame 2 restores the original raised-arm pose, retaining cosmetics,
  both gloves and the same separable cuff boundaries.

Frame numbers here are one-based; coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Start South | 26 |
| Loop South | 21, 17 |
| End South | 21, 26 |

The masks add 51 seeds and recolor 111 pixels per target. Another 34
matching-color material pixels remain protected. Evidence:
`tmp/juniper-winter-specials-author-refined-components.json`,
`tmp/juniper-winter-specials-author-exclusions.json`,
`tmp/juniper-winter-specials-author-inventory.log` and the three
`tmp/juniper-winter-specials-author-refined-art-*.png` sheets. Profile SHA-256:
`891a92064aee383da910816db0f3a2a2c8ae1b2051f899dc9c970805df75dcaa`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-specials-preview/summary.png)
shows frame 1 of each phase. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-specials-preview/blue-review/index.html)
shows all five South-facing source frames across three detail pages, with
additional enlarged face views. A visible note identifies the preserved gloves,
cosmetics and jewelry.

Crop `[25,20,32,39]` includes every visible pixel. Exact checks verified
1,408,800 preview pixels, ten full bindings, fifteen summary bindings, frame
counts and raw metadata. Chromium checked all five pages, including both
indexes: local links and images loaded without horizontal overflow. Evidence:
`tmp/juniper-winter-specials-author-preview-check.log` and
`tmp/juniper-winter-specials-author-preview-browser-check.json`.

The candidate material test passed, followed by formatting, targeted Clippy and
the final focused test in `tests/juniper_winter_specials.rs` (25.16 seconds).
It checks every new pixel in four targets, per-frame counts, unchanged metadata,
42 literal material landmarks and all 2,112 earlier variant PNG/metadata files.
Three effective controls omit a cheek-shadow component, omit an ear component
and remove region restrictions, exposing missed skin and accidental circlet,
shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-specials-author-candidate-test.log` and
`tmp/juniper-winter-specials-author-focused-checks.log`.

All 1,068 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,640 earlier original/variant PNG and metadata files
remain byte-identical; all 2,136 final variant files match the inspected
candidates. Evidence: `tmp/juniper-winter-specials-author-preservation.log`,
`tmp/juniper-winter-specials-author-stylized-validation.json` and
`tmp/juniper-winter-specials-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
