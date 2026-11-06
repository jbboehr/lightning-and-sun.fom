# Project assessment — 2026-11-05

Assessed commit `bd5980d` on `develop`. The latest runtime/content commit is dated
2026-10-14, adding Elsie Winter standing writing in [slice 200](slice-200.md).
For current priorities and future scope, use [planning](../planning.md).

## Verified inventory

The assessment resolved the profiles selected by
`palettes/sets/characters-trial.json` and compared unique asset paths with the
preserved `local/game/assets.zip`. The recorded archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This identifies the local source corpus, not a newly checked public game release.

| Asset group | Selected strips | Inventory for the registered 36 characters |
| --- | ---: | ---: |
| Portraits | 2,201 | 2,201 |
| World sprites | 2,708 | 4,159 |
| Combined | 4,909 | 6,360 |

The collection generates 19,636 non-vanilla variants. Raw coverage is 77.2%
overall and 65.1% for world sprites. Included assets can deliberately retain their
original appearance. The denominator includes shadows and special forms that may
need no recoloring; it excludes animals/miscellaneous NPC folders outside the
36-character registry. These percentages do not estimate development effort.

Seventeen characters include every normal Spring/Summer/Autumn/Winter world strip:
Adeline, Hayden, Ryis, Reina, Juniper, Celine, March, Balor, Valen, Eiland, Olric,
Landen, Nora, Holt, Josephine, Darcy and Dell. This does not establish every special
outfit's coverage. Elsie includes 122 of 128 seasonal world strips; only Winter
seated-writing start/loop/end in East and South remain.

Eighteen registered characters have no selected world sprites. Their inventories
and the other gaps are enumerated in the [finish plan](../plans/finish.md).
Hayden's excluded shadows have a [separate audit](hayden-shadow-audit.md).

## Verification observed in this assessment

- Formatting and Clippy with warnings denied passed.
- All 412 regular tests passed; 482 opt-in tests were ignored by that command.
- The release build and CLI help smoke check passed.
- The explicit latest Elsie Winter standing-writing pixel/material test passed.
- All three explicit GML runtime tests passed using the retained Fabricator.
- The documented optional archive-review failure was reproduced:
  `batch_reuse_reproduces_every_approved_adeline_mask` at
  `tests/review_batch.rs:623` produced 282 regions while expecting 143.

No assertions were changed. The complete ignored artwork suite, browser suite,
fresh MOMI installation and live gameplay were not rerun. Assessment itself made
no runtime changes and did not modify a game installation. Setup and historical
test evidence remain in [testing](testing.md) and [local storage](local-storage.md).

## Acceptance and release limits

Elsie slices 187–200 record pending visual acceptance. Hayden's embarrassed
expression and possible Reina/Juniper mouth details remain named art follow-ups.
The selected sampled ramps and Debug Blue are still trial palettes.

Historical [portrait-toggle](portrait-toggle.md) evidence demonstrates live
switching in a test-driver conversation. Latest slice checks demonstrate isolated
installation and simulated animator transitions, with engine services simulated
and cycles selected directly. They do not establish natural gameplay behavior of
the complete current collection, all mod interactions, controllers or other
platforms.

Current controls are per-character test hotkeys and start on Vanilla every launch.
There is no preset menu, saved selection or graphical installer. No release tag
or `.github` workflow was present at assessment time. The original
[handoff](../HANDOFF.md) describes an already completed investigation; some
scaffold notes also preserve historical scope. The current implementation and
history are described in [characters](characters.md), the root README and the
central `tests/profile_coverage.rs` check.
