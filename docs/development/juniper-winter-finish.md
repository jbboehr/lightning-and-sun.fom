# Completing Juniper's Winter sprites

This slice adds hair flip, snooze and spell-cast start/loop/end: five strips,
22 South-facing source frames. The fresh archive inventory confirms all 33
Winter strips are now covered. Juniper has 272 sources and 1,088 variants.
All 267 earlier regions and pins, seventeen source colors, nine color groups,
target mappings and portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-finish-study`. All 534 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-winter-specials-trial` bundle. Source pins
remain strict. The complete folder inventory is recorded in
`tmp/juniper-winter-finish-author-folder-inventory.txt`.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_specialanimation_winter_`. All retain 80×80 frames, Default
atlas and numeric origin `40.0,54.0`. Hair flip has six frames with durations
`[0.15,0.15,0.125,0.125,0.5,0.125]`; snooze has nine at
`[0.3,0.8,0.3,3.0,0.15,1.5,0.15,0.125,1.5]`. Spell start/loop have two/four
frames at `0.125`; spell end uses single-frame defaults. Raw sidecars at
`tmp/juniper-winter-finish-author-metadata.json` match independent archive
reads. The shared slice probes native animation behavior separately.

Every actual frame was inspected in Vanilla and all four targets. Exposed
face, ears, eyelid skin and forehead change. Gloves, circlet, cosmetics, cyan
gems, shoulder decorations, cuffs, hair and warm clothing details remain
original. No new source roles or coupled-material exceptions are needed;
earlier accepted exceptions remain unchanged. These are body animations;
separate magic-effect assets are outside this slice.

Fresh Winter material landmarks include:

- Hair flip frame 1 retains the glove across the face at `[39,37]` (`DD426C`),
  while ear `[35,37]` (`EFD89A`) and cheek `[37,40]` (`763F21`) change. Frame 2
  preserves the raised cuff `[46,38]` (`BC8B43`), separate from the face.
- Snooze frames 1–5 include the tiny shaded ear `[45,36]` (`BC8B43`) above
  the raised glove. The glove at `[45,38]` (`DD426C`), shoulder `[35,40]`
  and low cuff `[33,45]` (`BC8B43`) remain original. Frame 8 follows that
  shaded ear down to `[45,40]`.
- Spell loop frame 1 reveals forehead `[38,31]`, `[39,31]` and `[37,32]`
  (`763F21`) between hair and circlet. Frame 2 reveals `[37,32]`, `[36,33]`
  (`763F21`) and `[36,34]` (`BC8B43`). All change as skin; adjacent circlet
  `[38,32]` (`E3BF7F`) and `[37,33]` (`BC8B43`) retain their colors.
- All four spell-loop frames preserve cuff corners `[33,44]` and `[46,44]`
  (`BC8B43`) beside the covered hands. Winter gloves allow those corners
  to remain separate from the skin mask.

Frame numbers here are one-based; coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Hair flip South | 8, 21, 23, 23, 23, 22 |
| Snooze South | 24, 20, 24, 20, 24, 14, 14, 11, 22 |
| Spell start South | 26, 26 |
| Spell loop South | 34, 33, 30, 28 |
| Spell end South | 26 |

The masks add 203 seeds and recolor 496 pixels per target. Another 142
matching-color material pixels remain protected. Evidence:
`tmp/juniper-winter-finish-author-refined-components.json`,
`tmp/juniper-winter-finish-author-exclusions.json`,
`tmp/juniper-winter-finish-author-inventory.log` and the eight
`tmp/juniper-winter-finish-author-refined-art-*.png` sheets. Profile SHA-256:
`0c753095d87f8c91ef9146fb577c06762caa7aba3eb81aae54226ee1745b0de9`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-finish-preview/summary.png)
shows hair flip frame 4, snooze frame 8 and spell loop frame 2. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-finish-preview/blue-review/index.html)
shows all 22 South-facing source frames across eight detail pages, with
additional enlarged face views. A visible note identifies the shaded ear and
forehead coverage and the preserved gloves and jewelry.

Crop `[25,20,32,39]` includes every visible pixel. Exact checks verified
4,607,520 preview pixels, 44 full bindings, fifteen summary bindings, frame
counts and raw metadata. Chromium checked all ten pages, including both
indexes: local links and images loaded without horizontal overflow. Evidence:
`tmp/juniper-winter-finish-author-preview-check.log` and
`tmp/juniper-winter-finish-author-preview-browser-check.json`.

The candidate material test passed, followed by formatting, targeted Clippy and
the final focused test in `tests/juniper_winter_finish.rs` (24.98 seconds).
It checks every new pixel in four targets, per-frame counts, unchanged metadata,
76 literal material landmarks and all 2,136 earlier variant PNG/metadata files.
Four effective controls omit a cheek-shadow component, omit an ear component,
omit the exposed forehead and remove region restrictions, exposing missed skin
and accidental circlet, shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-finish-author-candidate-test.log` and
`tmp/juniper-winter-finish-author-focused-checks.log`.

All 1,088 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,670 earlier original/variant PNG and metadata files
remain byte-identical; all 2,176 final variant files match the inspected
candidates. Evidence: `tmp/juniper-winter-finish-author-preservation.log`,
`tmp/juniper-winter-finish-author-stylized-validation.json` and
`tmp/juniper-winter-finish-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
