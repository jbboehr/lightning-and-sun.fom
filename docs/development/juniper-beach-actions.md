# Juniper Beach blinks, actions and kiss

This slice adds blink East/South, action North/South/East and kiss East: six
strips with 31 source frames. Fourteen native West mirrors bring the offline
review to 45 cases. Juniper now has 284 sources and 1,136 variants. All 278
previous region objects, strict pins, seventeen source colors, nine groups,
target mappings and portrait-only definitions remain unchanged, including the
fourteen accepted Beach-pilot bangle-edge exceptions.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-beach-actions-study`. All 556 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-beach-pilot-trial` bundle.

Paths use `assets/animations/NPCs/Juniper/Sprites/Beach/` and prefix
`spr_npc_juniper_beach_`. All retain 80×80 frames, Default atlas and origin
`Middle,54.0`. Each blink has three frames with durations
`[0.075,0.125,0.075]`; each action has seven with
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four with
`[0.15,0.15,0.8,0.15]`. Raw sidecars in
`tmp/juniper-beach-actions-author-metadata.json` match independent archive reads.
The shared slice checks native direction, mirroring and the action's final-frame
hold; static previews do not establish gameplay timing.

Every actual frame was inspected in Vanilla and all four target palettes.
Exposed face, ears, mouth edges, chest, back, moving arms/hands, waist, legs and
feet change. The purple eyelid cosmetics, hair, gold hair tie, white/lilac wraps,
gold bangle centers and separable bangle borders remain original. No new source
roles or mappings are needed. For example, blink South frame 2 changes skin
`[37,35]` (`E3BF7F`) above the closed eye while retaining its `B789D5` eyelid at
`[37,36]`. Kiss frame 3 changes the cheek beside the mouth at `[46,38]` while
preserving the eyelid at `[41,35]` and bangle border at `[37,44]`.

Five new `BC8B43` bangle-edge occurrences share a connected color component with
hand skin. They follow the established full-skin compromise. The following
exact exceptions are asserted in the focused test and visibly noted throughout
the gallery; all other identifiable borders and all gold centers stay original:

| Strip and frame | Bangle edge that follows skin | Attached hand skin |
| --- | --- | --- |
| Action South, frame 3 | `[36,45]` | `[36,46]` |
| Action South, frame 5 | `[36,45]` | `[36,46]` |
| Action South, frame 6 | `[45,45]` | `[45,46]`, `[45,47]` |
| Action East, frame 1 | `[38,46]` | `[38,47]` |
| Action East, frame 6 | `[38,46]` | `[38,47]` |

Frame numbers here are one-based and coordinates are frame-local. South action
frame 3's opposite edge `[38,45]` is separable and stays original; East action
frame 1 similarly protects `[40,46]`. Blink and kiss require no new coupled-edge
exceptions. No schema or runtime behavior changes are introduced.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Blink East | 74, 78, 74 |
| Blink South | 82, 86, 82 |
| Action North | 59, 58, 58, 58, 58, 60, 62 |
| Action South | 74, 75, 74, 75, 74, 76, 82 |
| Action East | 64, 75, 68, 75, 68, 64, 74 |
| Kiss East | 66, 67, 73, 71 |

The masks add 1,052 seeds and change 2,184 pixels per target, including the five
exceptions. Another 74 matching-color bangle pixels stay protected. Evidence:
`tmp/juniper-beach-actions-author-refined-components.json`,
`tmp/juniper-beach-actions-author-exclusions.json` and nine refined art sheets.
Profile SHA-256:
`c253d1428a48e45da3406dbf083bec6b878334144f08c095b662abefb3393800`.

## Review and verification

The [five-choice summary](../../generated/juniper-beach-actions-preview/summary.png)
shows blink South frame 2, action East frame 2 and kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-beach-actions-preview/blue-review/index.html)
includes all 31 source frames and fourteen West mirrors across thirteen detail
pages, with additional enlarged face views. The full gallery includes every
new bangle-edge exception; the summary remains a sample.

The full crop `[25,20,32,39]` contains every visible pixel in normal and mirrored
views. Exact checks verify 8,935,200 preview pixels, ninety full bindings,
fifteen summary bindings, source frames and raw metadata. Chromium checks both
indexes and all thirteen detail pages for local links, loaded images and
horizontal overflow. The preview directory is 827,837 bytes; its largest PNG
is 59,320 bytes. Evidence: `tmp/juniper-beach-actions-author-preview-checks.log`
and `tmp/juniper-beach-actions-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_beach_actions.rs` (24.20 seconds). It verifies every
new pixel in four targets, per-frame counts, unchanged metadata, 79 literal
skin/material landmarks and all 2,224 previous variant PNG/metadata files. Four
effective controls omit ear, hand and waist components, then remove region
restrictions to expose accidental bangle recoloring. Evidence:
`tmp/juniper-beach-actions-author-candidate-test.log` and
`tmp/juniper-beach-actions-author-final-checks.log`.

All 1,136 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,780 previous original/variant PNG and metadata files
are byte-identical; all 2,272 final variant files match the inspected candidates.
Evidence: `tmp/juniper-beach-actions-author-preservation.log`,
`tmp/juniper-beach-actions-author-stylized-validation.json` and
`tmp/juniper-beach-actions-author-final-diff-check.log`.

The shared slice handles broader verification, combined packaging, native
probes and isolated installation. Live gameplay, schedules and transitions
were not exercised here. All game-derived images remain ignored.
