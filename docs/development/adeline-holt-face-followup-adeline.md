# Adeline remaining blush dots, 2026-11-06

The user reported that Adeline still had two uncolored blush dots in the
corrected roster review. They otherwise found that roster review pretty good,
apart from Holt's separately investigated moustache. This record owns only the
bounded Adeline correction; combined installation and art acceptance belong to
the parent batch.

## Finding and correction

The dots are disconnected original skin pixels inside the checkerboard blush,
not a second blush color. In
`assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_cartoon_embarrassed.png`,
their frame-local coordinates are `(148, 73)` and `(160, 75)`. Both native frames
contain them; frame two uses strip coordinates `(444, 73)` and `(456, 75)`.
Their source color is `#E3A17B`. All four seasonal versions have the same defect.
The Beach version has no corresponding unselected skin islands.

The complete 126 selected portrait strips / 252 native portrait frames were
audited for unselected components in the cheek/eye/mouth area using both existing
portrait skin groups. Only the 16 confirmed single-pixel cheek islands receive
new seeds, across Spring, Summer, Autumn and Winter. Other pre-existing source
components in the audit retain their prior treatment; the report does not
authorize a broader skin-edge redesign.

The profile retains every source slot, group, original region field and existing
seed. The recipe is byte-identical. The dots now use the existing first skin
target, preserving the reviewed six cosmetic treatments and all existing target
values. Wiscar's skin target for this source happens to equal the original RGB,
so its images remain exact. No new colors, replacement rules or runtime changes
are needed.

The current source-bound
[correction record](native-tone-face-corrections/adeline.json) contains the extra
components and literal landmarks. The previous record/profile/recipe are frozen
under `tmp/adeline-holt-face-followup/adeline/before/`. Earlier roster review and
verification packets remain unchanged.

## Verification

The literal regression was run before changing the profile: **80 of 96 assertions
failed** on the two islands across four outfits, two frames and all six added
tones. Sixteen Wiscar assertions already passed because its expected RGB equals
the source. After the correction, **all 96 pass**. The test contains literal
coordinates, source RGB and expected target colors, independent of the edited
component record.

The complete fresh candidate at
`generated/adeline-holt-face-followup/adeline` contains 3,150 actual PNGs and
3,150 animation metadata files for all 315 selected strips and ten variants.
The corpus comparison proves:

- Exactly **20 PNGs / 80 RGBA pixels** change against the previous roster
  candidate: four seasonal portraits in five tones.
- All **1,260 old-four PNGs**, **1,134 added-tone world PNGs** and **732 other
  added-tone portrait PNGs** remain byte-identical.
- All 24 affected added-tone portrait PNGs match their precisely constructed
  expected delta over **2,557,440 decoded pixels**. Alpha and every unrelated
  pixel are exact; Wiscar's four affected-portrait images also remain exact.
- All **3,150 metadata files**, **12 prior reviewed facial landmark assertions**
  and **30 protected-material assertions** pass unchanged.
- All 315 source PNG hashes match their source-bound regions. The original
  archive still hashes to
  `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

Proof files are [checks.json](../../tmp/adeline-holt-face-followup/adeline/checks.json),
[before regression](../../tmp/adeline-holt-face-followup/adeline/landmarks-before.json),
[after regression](../../tmp/adeline-holt-face-followup/adeline/landmarks-after.json),
[all-portrait component audit](../../tmp/adeline-holt-face-followup/adeline/unmapped-face-audit.json)
and [added components](../../tmp/adeline-holt-face-followup/adeline/added-components.json).
The retained CPU-only helpers are in
`local/authoring/adeline-holt-face-followup/adeline/`.

Both [frame-one](../../tmp/adeline-holt-face-followup/adeline/all-six-frame-1.png)
and [frame-two](../../tmp/adeline-holt-face-followup/adeline/all-six-frame-2.png)
source/before/after sheets retain every added tone. The author inspected both
sheets. These are offline evidence for integration, not a substitute for the
user's visual assessment. No game rendering, GPU work, live acceptance, staging
or commit is claimed.

## Integration notes

Use the new Adeline candidate when building the combined follow-up report; the
old roster candidate intentionally retains the reproduced defect. The shared
opt-in roster regression consumes the updated source record without changing its
assertions, but needs the new candidate selected for Adeline. The existing
generator reads the unchanged target table from the correction record, so no
shared target-baseline update is necessary. Historical group-A totals and frozen
roster packets remain historical; a fresh combined packet should add the 80
actual corrected pixels and current source hashes instead of rewriting them.
