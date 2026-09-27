# Juniper's Autumn blink, sit, eat and drink sprites

This slice adds eleven strips: blink South/East and sit, eat and drink
North/South/East. They contain 31 source frames; twelve native West mirrors make
43 offline review cases. Juniper now has 223 sources and 892 variants. All 212
earlier region objects, hashes, groups and palette mappings remain unchanged,
including the `F1E791` midriff alias and earlier accepted bracer exceptions.
Portrait-only recipes are untouched.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete corpus is `extracted/juniper-autumn-actions-study`. All 424 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-autumn-trial` bundle. No pin was refreshed or
bypassed.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Autumn/`, using prefix
`spr_npc_juniper_autumn_`. All retain 80×80 frames, Default atlas and Middle/54
origin. Blinks have three frames at `[0.075,0.125,0.075]`; sitting uses single-frame
defaults. Eat North and all drink strips have three frames with duration `1.0`.
Eat South/East have five frames at `[0.125,0.15,0.175,0.125,0.6]`.
Raw sidecars in `tmp/juniper-autumn-actions-author-metadata.json` match the
independent archive reads. The shared native checks cover seated direction
bindings, West mirrors and the eat/drink final-frame holds `[240,360]`.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
blink skin, neckline, exposed midriff and hands change. Hair, cosmetics, circlet,
gems, cuffs, sleeves, skirt, boots and outlines stay original. The red mouth
interior and dark mouth rim in the open-mouth eating frames remain unchanged.
The Autumn sleeves cover areas exposed in Summer, so masks use the actual
Autumn geometry.

Warm gold corners are separable in this batch; there are no new coupled-material
exceptions. Drink East frames 1/3 deliberately preserve the isolated cuff corner
`[40,44]`, colored `E8B171`, while the same shade on the hand at `[42,43]` changes.
The established `F1E791` alias covers the visible South-facing midriff. North
seated poses retain their small exposed hand shadows where the gold panel is
absent. Literal tests keep these decisions distinct from visible cuff borders.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Blink South | 44, 48, 44 |
| Blink East | 37, 41, 37 |
| Sit North / South / East | 10 / 42 / 28 |
| Eat North | 7, 4, 7 |
| Eat South | 42, 38, 36, 44, 42 |
| Eat East | 23, 27, 18, 23, 24 |
| Drink North | 7, 4, 7 |
| Drink South | 43, 40, 43 |
| Drink East | 26, 25, 26 |

The masks add 414 seeds and recolor 887 pixels per target; 124 matching-color
jewelry pixels remain protected. Evidence:
`tmp/juniper-autumn-actions-author-refined-components.json`,
`tmp/juniper-autumn-actions-author-exclusions.json` and the thirteen
`tmp/juniper-autumn-actions-author-refined-art-*.png` sheets. Profile SHA-256:
`ec5797b20b5c33d6cee4fc80925fb6f0f6907365861953fe181f09736cacd22e`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-actions-preview/summary.png)
shows blink South frame 2, eat South frame 3 and drink East frame 2. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-actions-preview/blue-review/index.html)
shows all 43 cases over eighteen detail pages, with enlarged face views.

The full-body crop `[25,20,32,39]` includes every visible pixel. Exact checks
verified 8,558,880 preview pixels, 86 full-frame bindings, fifteen summary
bindings, metadata, frame counts and West mirrors. Chromium checked all twenty
pages including both indexes: every local link and image loaded, with no
horizontal overflow. Evidence:
`tmp/juniper-autumn-actions-author-preview-check.log` and
`tmp/juniper-autumn-actions-author-preview-browser-check.json`.

The independent candidate test and targeted Clippy passed before promotion;
the final focused test in `tests/juniper_autumn_actions.rs` passed afterward.
It checks every new pixel in four targets, per-frame counts, metadata, 60 literal
skin/material landmarks and all 1,696 prior variant PNG/metadata files. Practical
controls omit a finger-shadow component, omit the midriff highlight component,
and merge groups; the test catches missed skin and spill into a protected cuff.
Evidence: `tmp/juniper-autumn-actions-author-candidate-test.log`,
`tmp/juniper-autumn-actions-author-clippy.log` and
`tmp/juniper-autumn-actions-author-final-test.log`.

All 892 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,120 prior original/variant PNG and metadata files are
byte-identical; all 1,784 final variant files match the inspected candidates.
Evidence: `tmp/juniper-autumn-actions-author-preservation.log`,
`tmp/juniper-autumn-actions-author-stylized-validation.json` and
`tmp/juniper-autumn-actions-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
installation. Static previews do not establish live timing, schedules or outfit
transitions. Game-derived images remain ignored.
