# Juniper Beach idle and walk

This slice adds six Beach strips: idle and walk North, South and East. Fifteen
source frames and five native West mirrors make twenty offline review cases.
Juniper now has 278 sources and 1,112 variants. All 272 earlier region objects,
source pins, seventeen source colors, nine groups and target mappings remain
unchanged, along with the portrait-only definitions.

## Sources and material decisions

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-beach-pilot-study`. Its 544 earlier
original PNG/metadata files match `characters-march-winter-injured-trial`.
Source pins remain strict.

Paths use `assets/animations/NPCs/Juniper/Sprites/Beach/` and prefix
`spr_npc_juniper_beach_`. All retain 80×80 frames, Default atlas and origin
`Middle,54.0`. Idle has one frame with default timing; walk has four frames at
`0.15`. Raw sidecars in `tmp/juniper-beach-pilot-author-metadata.json` match the
root's independent archive reads. Shared native checks cover directional
selection and East-to-West mirroring.

Every actual frame was inspected in Vanilla and all four target palettes.
Face, ears, neck, chest, exposed back, arms, hands, waist, legs and feet change.
White/lilac wraps, hair, eyes, gold bangle centers and separable bangle borders
stay original. The Beach hair tie uses `DF8D4B` and `FFD565`; these are jewelry
colors, so neither is added to the skin roles. For example, idle South's hair
tie at `[35,27]` and `[36,27]` stays original while the exposed chest at
`[39,42]` changes. Idle East's `BC8B43` far arm at `[44,44]` changes, while its
near bangle border at `[34,45]` stays original.

Fourteen `BC8B43` bangle-edge occurrences cannot be separated from their
connected hand/arm skin with the current component masks. They follow the
established full-skin compromise, with no schema or runtime changes. All gold
centers and every separable border remain protected. These exact exceptions
are asserted in the focused test and visibly noted in the gallery:

| Strip and frame | Frame-local coordinates that follow skin |
| --- | --- |
| Walk North, frame 2 | `[35,46]`, `[44,46]`, `[45,46]`, `[46,46]` |
| Walk North, frame 4 | `[33,46]`, `[34,46]`, `[35,46]`, `[44,46]` |
| Walk South, frame 2 | `[35,46]`, `[45,46]`, `[46,46]` |
| Walk South, frame 4 | `[33,46]`, `[34,46]`, `[44,46]` |

Frame numbers in this document are one-based. The North frame 2 exceptions
connect to arm `[44,45]` and hand `[45,47]`, or hand `[35,47]`. The analogous
South frame still protects the separable dark bangle corner `[44,46]`, which
uses `763F21`. The opposite swing preserves the corresponding separable corner
at `[35,46]`. Older accepted material exceptions remain unchanged.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Idle North | 62 |
| Idle South | 82 |
| Idle East | 74 |
| Walk North | 62, 58, 62, 57 |
| Walk South | 82, 78, 82, 78 |
| Walk East | 74, 77, 74, 68 |

The masks add 537 seeds and change 1,070 pixels per target, including the
fourteen exceptions. Another 41 matching-color bangle-border pixels remain
protected. Evidence is in `tmp/juniper-beach-pilot-author-refined-components.json`,
`tmp/juniper-beach-pilot-author-exclusions.json` and the six refined art sheets.
Profile SHA-256:
`7b0b67fc8aa62ef15d8fcdec42ea0382cef941ab69b42dd652bda551e60eb66d`.

## Review and verification

The [five-choice summary](../../generated/juniper-beach-pilot-preview/summary.png)
shows idle South, walk East frame 2 and walk North frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-beach-pilot-preview/blue-review/index.html)
includes every source frame and West mirror across eight detail pages, with
additional enlarged face views. The summary remains a sample, while the full
gallery includes all twenty cases and all fourteen jewelry-edge exceptions.

The full crop `[25,20,32,39]` contains every visible pixel in both normal and
mirrored views. Exact checks verify 4,231,200 preview pixels, forty full bindings,
fifteen summary bindings, frame counts and raw metadata. Chromium checks both
indexes and all eight detail pages for working local links, loaded images and
horizontal overflow. Evidence: `tmp/juniper-beach-pilot-author-preview-checks.log`
and `tmp/juniper-beach-pilot-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_beach_pilot.rs` (24.14 seconds). It checks every
new pixel in four targets, per-frame counts, unchanged metadata, 69 literal
skin/material landmarks and all 2,176 prior variant PNG/metadata files. Four
effective controls omit ear, hand and waist components, then remove region
restrictions to expose accidental bangle recoloring. Evidence:
`tmp/juniper-beach-pilot-author-candidate-test.log` and
`tmp/juniper-beach-pilot-author-final-checks.log`.

All 1,112 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,720 prior original/variant PNG and metadata files are
byte-identical; all 2,224 final variant files match the inspected candidates.
Evidence: `tmp/juniper-beach-pilot-author-preservation.log`,
`tmp/juniper-beach-pilot-author-stylized-validation.json` and
`tmp/juniper-beach-pilot-author-final-diff-check.log`.

The shared slice handles full verification, combined packaging, native probes
and isolated installation. Static previews do not establish live gameplay,
schedules or outfit transitions. All game-derived images remain ignored.
