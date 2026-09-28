# Juniper's Winter action, sleep and kiss sprites

This slice adds action North/South/East, sleep East and kiss East: five strips,
26 source frames and twelve native West mirror views, totaling 38 review cases.
Juniper now has 261 sources and 1,044 variants. All 256 earlier regions and
source pins, seventeen source colors, nine color groups, target mappings and
portrait-only recipes remain unchanged.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-winter-standard-study`. All 512 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-winter-actions-trial` bundle. Source pins remain
strict.

Sources use `assets/animations/NPCs/Juniper/Sprites/Winter/` and prefix
`spr_npc_juniper_winter_`. All retain 80×80 frames, Default atlas and origin
`Middle,54.0`. Each action has seven frames with durations
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; sleep uses single-frame defaults; kiss has
four frames at `[0.15,0.15,0.8,0.15]`. Raw sidecars are recorded in
`tmp/juniper-winter-standard-author-metadata.json` and match the root's
independent archive reads. The shared slice checks native action holds,
speaking fallback and direction handling separately. Sleep/kiss West previews
exercise the native Single pack and NPC mirror without claiming natural
gameplay dispatch.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
closed-eye skin and their shadows change. Winter gloves cover the hands; bright
cuffs retain jewelry highlights even where they share skin colors. North action
has an explicit empty mask and remains byte-identical in every preset. Hair,
circlet, cyan gems, shoulder decorations, cosmetics, eyes, gloves and warm
clothing details remain original. No new source roles or coupled-material
exceptions are needed. Previous accepted exceptions remain unchanged.

The moving cuffs and face use separate components:

- Action South frame 3 preserves cuff `[37,42]` (`EFD89A`) and `[36,43]`
  (`E3BF7F`), while the ear `[35,36]` (`EFD89A`) changes.
- Action East frame 2 preserves cuff highlights `[43,42]` and `[47,43]`
  (`EFD89A`), midtone `[44,43]` (`E3BF7F`) and border `[45,44]` (`BC8B43`).
  The moving ear `[39,36]` and jaw `[41,39]` change independently.
- Sleep preserves the pink glove beside the mouth at `[43,39]` (`DD426C`)
  and cuffs below at `[40,42]` (`E3BF7F`) and `[41,42]` (`EFD89A`). Face
  `[41,39]` and closed-eye skin `[38,35]` change; cosmetics remain pink.
- Kiss frame 3 changes closed-eye skin `[41,37]` (`E3BF7F`) and `[46,37]`
  (`EFD89A`), preserving cosmetics `[41,35]` (`FC639B`), shoulder ornament
  `[38,40]` (`BC8B43`) and cuff `[38,44]` (`EFD89A`).

Frame numbers here are one-based; coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Action North | 0, 0, 0, 0, 0, 0, 0 |
| Action South | 22, 22, 22, 22, 22, 22, 22 |
| Action East | 19, 19, 19, 19, 19, 19, 19 |
| Sleep East | 18 |
| Kiss East | 19, 19, 23, 23 |

The masks add 142 seeds and recolor 389 pixels per target. Another 211
matching-color material pixels remain protected. Evidence:
`tmp/juniper-winter-standard-author-refined-components.json`,
`tmp/juniper-winter-standard-author-exclusions.json`,
`tmp/juniper-winter-standard-author-inventory.log` and the eight
`tmp/juniper-winter-standard-author-refined-art-*.png` sheets. Profile SHA-256:
`e5d499d03b3533977dbd9d8d097c2d7bccc08079649aa1c2ad9f7b79664c115e`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-winter-standard-preview/summary.png)
shows action South frame 3, sleep East and kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-winter-standard-preview/blue-review/index.html)
contains all 26 source frames and twelve West mirrors across twelve detail
pages, with enlarged face views. A visible note explains the unchanged North
art and the bright cuff jewelry.

Crop `[25,20,32,39]` includes every visible pixel in native and mirrored views.
Exact checks verified 7,618,080 preview pixels, 76 full bindings, fifteen summary
bindings, frame counts and raw metadata. Chromium checked all fourteen pages,
including both indexes: local links and images loaded without horizontal
overflow. Evidence: `tmp/juniper-winter-standard-author-preview-check.log` and
`tmp/juniper-winter-standard-author-preview-browser-check.json`.

The candidate material test passed, followed by formatting, targeted Clippy and
the final focused test in `tests/juniper_winter_standard.rs` (23.99 seconds).
It checks every new pixel in four targets, per-frame counts, unchanged metadata,
61 literal material landmarks, the explicit empty North mask and all 2,048
prior variant PNG/metadata files. Three effective controls omit a cheek-shadow
component, omit an ear component and remove region restrictions; they expose
missed skin and accidental circlet, shoulder and cuff recoloring. Evidence:
`tmp/juniper-winter-standard-author-candidate-test.log` and
`tmp/juniper-winter-standard-author-focused-checks.log`.

All 1,044 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,560 prior original/variant PNG and metadata files are
byte-identical; all 2,088 final variant files match the inspected candidates.
Evidence: `tmp/juniper-winter-standard-author-preservation.log`,
`tmp/juniper-winter-standard-author-stylized-validation.json` and
`tmp/juniper-winter-standard-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
