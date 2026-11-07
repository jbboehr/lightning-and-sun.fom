# Adeline blush dots and Holt moustache

2026-11-06. Both reported defects are corrected and the complete focused batch
passes its technical checks. The user judged the preceding displayed roster
"otherwise looks pretty good", except for two Adeline blush dots and Holt's
pre-existing moustache damage. Preserve that qualified assessment and unspecified
inspection depth. These two revised appearances still need visual assessment;
complete new-tone coverage and live acceptance are separate. No commit is requested.

## Corrections

The [Adeline record](adeline-holt-face-followup-adeline.md) identifies two isolated
skin pixels inside checkerboard blush, at local `(148,73)` and `(160,75)`, in both
frames of four seasonal cartoon-embarrassed portraits. Adding sixteen component
seeds makes them follow the existing skin tone. The recipe, source slots, groups
and all targets remain exact. Exactly 20 PNGs / 80 pixels change; the Wiscar target
already equals the source RGB and its corresponding images remain exact. All old
choices and world outputs remain exact. The full 126-portrait inventory was audited.

The [Holt record](adeline-holt-face-followup-holt.md) traces the moustache defect to
368 component seeds already present in the original portrait mask. Moustache hair
shares skin RGBs, so these components were mistakenly recolored in all ten current
choices. Removing only those source-bound selections restores 896 source pixels
across 32 portraits / 64 native frames. Exactly 320 PNGs / 8,960 pixels change.
This includes 128 old-choice portrait PNGs / 3,584 pixels: a deliberately bounded
moustache-only exception to earlier output preservation.

A new current-only `holt-native-tone-base.json` preserves the historical world
profile, portrait profile and recipes unchanged. The current Holt set selects
that base, while its six added choices retain the facial override with the same
seed removals. All ten preset objects, source colors, groups and targets remain
exact. Blush, actual skin and unrelated materials remain exact. Existing historical
Holt tests and accepted snapshots keep their original contracts and inputs.

## Source-bound regressions and full preservation

Adeline's literal regression observes 80 failures among 96 assertions before the
fix, with sixteen same-RGB Wiscar cases already passing; all 96 pass after. Holt's
3,680 component landmarks fail before and pass after. Independent author corpus
checks compare all 4,580 fresh PNGs and metadata to precisely reconstructed expected
images, including every pixel in the affected portraits. Alpha, world artwork and
all unrelated pixels stay exact.

The shared current roster regression now selects the fresh two-character builds.
It retains exact old-four preservation everywhere except Holt's explicitly
enumerated, source-hashed moustache components. These must equal original source
RGBA in all ten choices; all other pixels retain their previous requirements.
The test asserts the exact 128 old-choice PNG and 8,960 restored-pixel counts.
A separate frozen-first-roster test still proves the original 23,688 older-choice
PNGs byte-exact. No historical pixel assertion is discarded to hide the defect.
The regular seed-removal fixture reconstructs Holt's new base from the unchanged
historical profile and permits only the listed portrait seed removals.

Observed shared retained-before regression: 2,288 failures, consisting of the
eighty Adeline cases and 2,208 new-six moustache landmarks. All three current
opt-in tests pass in 163.559 seconds: 360 correction and 2,772 protected assertions,
complete 35-character preservation, and the frozen prior-four proof. The complete
corpus checks 59,220 PNGs and metadata; 23,560 old-four and 23,424 added-tone world
PNGs stay byte-exact, with the 128 Holt exception files checked pixel by pixel.
Across initial-to-current facial changes, 5,774 PNGs / 183,985 pixels change;
alpha and unrelated materials are exact over 615,277,440 decoded portrait pixels.

Formatting, denied-warning all-target Clippy, 465 regular tests and release build
pass; 519 fixture-dependent tests remain ignored by default. The three opt-in
tests above pass separately. Authoring replay reproduces all 36 sets and all
1,812 historical noncosmetic targets. No Rust production, GML, menu or persistence
behavior changes in this batch.

The [full-roster binding](../../tmp/adeline-holt-face-followup/roster-binding.json)
verifies all 63,120 actual current PNG hashes. Relative to the preceding reviewed
roster, exactly 340 PNGs change on 36 portrait strips: 20 Adeline and 320 Holt.
All 58,540 outputs for the other 34 characters, all 41,110 world PNGs and all 3,900
latest Celine PNGs stay exact. Of the originally accepted 25,248 outputs, 25,120
stay exact and only the 128 enumerated Holt moustache files differ. Composite
binding SHA256 is `1ca764233357e9d2169d791b50838de18890d427f7e359762298ed0b455848e1`.

## Package and native installation

Root and standalone-source Nix builds produce the identical package
`/nix/store/4qxn0543av3p127g0j6g6szmyp8chqp1-mistria-palette-0.1.0`.
All 355 recipes match source, with 356 packaged files total and 1,179 standalone
text input pins. Only the Adeline/Holt facial profiles and current Holt set change
relative to the prior package; the corrected Holt base is added. The production
binary remains byte-identical. This uses the same machine/store, not a fresh OS.

A fresh focused packaged installation verifies both changed characters: 458
strips / 4,580 variants, in 38.281 seconds using the unmodified pinned strict MOMI
runner with no pipe intervention or skipped checks. All generated hashes match
the reviewed candidates. All 458 original PNG and metadata pairs stay byte-exact.
Production atlas reconstruction verifies 14,267 original/variant native frames
with exact alpha, visible RGBA, dimensions, placements and metadata. Hidden RGB
under zero alpha may differ after atlas trimming. Installed archive SHA256 is
`d51ba323c051dd6ba80465a130cbe009bd4406cca6956c7b6625a7346cde23ca`.

Runtime and control scripts remain byte-exact. The actual installed two-character
table matches the current runtime inputs, SHA256
`5e0dc1a49b3ed92a034b40adb2c3bbda068edd4fb6e64178db9c0e021508cbf0`.
Reconstructing the full 36-character table from unchanged IDs, labels, order and
animation groups produces the previous actual installed/live-tested table exactly:
3,491,802 bytes, 36 helpers, SHA256
`131d08081c2f08d1878d940e0582ead12c9f198264a9b79d971e18f366b21d27`.
This combines a fresh focused installation with a full-roster output/runtime-input
proof; it does not claim a new full36 installation or live playthrough.

The [integration packet](../../tmp/adeline-holt-face-followup/integration-checks.json)
owns independent installed spectral/shadow preservation, exact archive restoration,
source-only candidate bindings and final artifact hashes. Earlier full36/native/
live packets remain frozen and retain their original scope.

## Focused review and retention

The [review](../../tmp/adeline-holt-face-followup/review/index.html) shows
**Vanilla → Before → After**, defaulting to Very deep and native frame 2. Before
is the exact previous roster candidate. Both character pages expose all 158
selected portrait strips, both frames and all ten choices: 6,636 comparison frame
cases. Twenty static contact sheets cover both frames and all choices.

Source and browser checks verify 2,236 copied bindings, 2,272 portable entries,
44 internal links and six desktop/phone page loads with their filter/frame/tone/
crop controls. The parent inspected the reported default cases and source/after
mouth patterns; the authors inspected the complete six-tone Adeline treatment and
all ten Holt choices over the unique expression/frame patterns. Automated checks
and internal inspection do not provide user visual acceptance.

The [portable ZIP](../../tmp/adeline-holt-face-followup/review.zip) is self-contained,
36,496,652 bytes (34.8 MiB), SHA256
`5d1dba140f2f7b076d0cd8897c9d186d903ad5f46be29fd6e4f608dec30b1e59`.
The mutable current roster index points to these two revised character pages;
earlier galleries and ZIPs remain unchanged. The source-only Linux candidate
excludes game images, archives, private local state and credentials.

Scoped retention review keeps about 185 MiB of final candidates and 449 MiB of
review, proof and standalone-source inputs, before final small source/summary
artifacts. This includes Adeline's 135 MiB pre-author candidate: it caught an
authoring precondition and is retained as evidence, not an integration input.
The existing owned installation lab is reused and restored; no extra game archive
is copied. All authoring/browser checks use CPU; no GPU, game or local inference
session is needed or claimed.

Recommend the bounded corrections based on the exact component proof and source
appearance. One focused visual assessment is due for Adeline's cheek dots and
Holt's moustache. Overall tone-coverage acceptance, physical controllers, broader
seasonal/action play, compatibility/licensing and publication remain separate gates.

## Subsequent Holt edge assessment

The user finds the revision "otherwise much better", but reports remaining
pixels around the moustache. The [material follow-up](holt-moustache-edge.md)
identifies surrounding skin restored too broadly; its
[integration record](holt-moustache-edge-integration.md) owns the bounded
correction and focused review. The 368-component / 896-pixel figures and
128-old-choice-file exception above describe this frozen first restoration.
The latest exception is narrowed to 64 true-hair components / 576 source pixels
and 64 old-choice files. Adeline's improved output and all historical packets
remain exact. No complete art/coverage or live acceptance is inferred.
