# Deep umber pilot: Player 27 and Player 30

2026-11-07. **CPU pilot prepared and technically verified; Player 27 selected
under owner delegation; candidate art assessment remains separate.** The user replied "sure" to the recommendation
for a small CPU-only comparison. This authorizes the pilot, not a full-roster
addition, preset-ceiling change or installation. Earlier art approvals and their
qualifications remain intact. The [native-tone plan](../plans/native-tone-coverage.md)
owns broader scope.

## Delivered comparison

- [Interactive pilot](../../tmp/deep-umber-pilot/review/index.html)
- [Download with all selected samples](../../tmp/deep-umber-pilot/review.zip)
- [Technical checks and fingerprints](../../tmp/deep-umber-pilot/checks.json)
- [Mapping and cosmetic evidence](../../tmp/deep-umber-pilot/mapping-evidence.json)
- [Literal pixel proof](../../tmp/deep-umber-pilot/literal-pixel-checks.json)
- [Reference diagnostics](../../tmp/deep-umber-pilot/reference-diagnostics.json)

Columns read Vanilla, current Player 22, candidate Player 27, candidate Player 30,
current Player 35. Candidate columns are outlined. Controls expose neutral and
expressive portraits, world sprites, all native frames, full portraits and
Seridia's Flashback Priestess example. Cards wrap in the same order on phones.

The pilot covers Adeline, Ryis, Reina, Seridia and Holt, each with one neutral
portrait, one expressive portrait and one Spring world strip. Seridia adds one
Flashback Priestess portrait for the lip/material boundary: 16 strips and 27
native frames, or 135 frame/choice views across five columns. This is not
exhaustive expression, outfit or live-game evidence.

## Source colors and authoring decisions

The main skin targets are exact creator ramps. The original source LUT member
matches the creator catalog's SHA256.

| Candidate | Light | Middle | Shadow | Deep |
| --- | --- | --- | --- | --- |
| Player 27 | `#813723` | `#582718` | `#431D11` | `#2D130A` |
| Player 30 | `#6A2918` | `#50210F` | `#3A1809` | `#271003` |

The existing shade-role helper and [mapping record](native-tone-shade-mappings.json)
supply skin, transition and special-material targets. They reproduce the current
six donor families on each pilot character before deriving candidate values.
Additional facial slots are classified only when all six families agree on an
existing skin role or the explicit main/deep/soft blush ladder. An ambiguous
slot fails preparation rather than receiving an inferred default.

New cosmetics are a **temporary pilot assumption**: retain Player 22's authored
rose offsets relative to donor light for main/soft blush and donor middle for
deep blush. This gives:

| Candidate | Main blush | Deep blush/lip | Soft blush/lip |
| --- | --- | --- | --- |
| Player 27 | `#964B3E` | `#6F3231` | `#A55B4E` |
| Player 30 | `#7F3D33` | `#672C28` | `#8E4D43` |

These values are not an approved full-roster cosmetic policy. Celine,
Nora/Maple cosmetics and other unsampled characters need their own mappings
and review in an approved rollout.

Profiles are copied into the isolated pilot with only selected asset entries.
Those entries, source colors, groups, seeds, dimensions and source hashes stay
exact. Production profiles, selected recipes and the preset ceiling are unchanged.
Authoring, CLI output and proofs stay under `tmp/deep-umber-pilot/`; local helpers
are `local/authoring/deep-umber-pilot/build.py` and `check.py`.

## Verification and limits

The existing release CLI builds four isolated variants per character: two
current controls and two candidates. It uses strict source hashes and the
existing four-variant path without changing the ten-generated limit.

- All 32 candidate PNGs and 32 controls pass independent source-component flood
  and literal RGBA comparison: 4,816,640 pixel comparisons. Alpha and metadata
  stay exact; unselected pixels retain source values. The 32 controls also match
  current composite-report and generated PNG hashes byte for byte.
- All 373 protected palette/source/prior-review inputs stay unchanged, including
  the prior coverage packet and pending coverage decision record. No new owner
  acceptance is recorded.
- The diagnostic reproduces all 2,592 existing assignments before evaluating
  either candidate. Player 27 improves the reference ranking for creator
  24/27/30; Player 30 improves 27/30/33, with only a small improvement for 33.
  Each applies across 36 characters. These rankings do not establish rendered
  equivalence, an acceptance threshold or complete coverage; earlier warm
  midpoint and muted-tone differences remain.
- CPU Chromium at 1600×1000 and 390×844 passes sample/character filters, all 80
  sample/choice images, last-frame access for all 16 strips, full portraits and
  empty-filter behavior without horizontal overflow. Direct file opening passes.
- All 107 worksheet/ZIP entries agree with fingerprints and archive bytes;
  source bindings and local links pass. The download is 1,055,617 bytes. Exact
  hashes and screenshots are in the check packet.

Pillow supplies read-only independent pixel validation; the existing Rust CLI
generates the images. Commands were
`nix-shell -p python3Packages.pillow --run 'python3 local/authoring/deep-umber-pilot/build.py'`
and `python3 local/authoring/deep-umber-pilot/check.py`. No Rust/game runtime source
changed. Full build/install/remove/live checks were not repeated for this
bounded offline pilot. No local GPU or local model inference was used.

## Assessment and next decision

Sol's sample assessment favors **Player 27**: it supplies the umber undertone
while retaining more visible facial shade separation than Player 30. Player 30
is a viable darker alternative in the same general hue family. This is an
assistant recommendation, not owner acceptance of either candidate or earlier
coverage approximations. Provisional blush also remains open to assessment.

The original pilot authorization covered only this comparison. Subsequently,
the owner asked whether both candidates would be kept. Sol proposed exactly one
Player 27 addition retaining all existing choices, and the owner replied "up to
you". Record this as delegation of the choice; Sol selects Player 27 and proceeds
with the [single-family rollout](../plans/player27-rollout.md). Do not substitute
that delegation for an owner visual assessment of the displayed examples.
Remaining character/material mappings, the bounded preset-limit/test change and
integrated verification belong to that rollout. Further live verification needs
a checked/reserved local GPU.

Retain the approximately 8 MiB authoring/build tree, 1.01 MiB download and small
browser check directory for review/reproduction. Prior packets remain frozen.
