# Stillwell and Terithia muted-tone pilot

2026-11-08. **Prepared and technically verified; displayed pilot candidates
and provisional cosmetics accepted.** After the owner asked for Sol's assessment of the current
twelve-choice review, Sol recommended
retaining all current choices and testing Stillwell and Terithia as muted light
and medium donor candidates. The owner replied "go for it", authorizing this
small CPU comparison through execution. The owner subsequently said "looks good"
after receiving the comparison and a question covering both candidates and their
provisional cosmetics; see [owner feedback](#owner-feedback). Full-roster work
and broader artwork/coverage acceptance remain separate.

The [native-tone plan](../plans/native-tone-coverage.md) owns broader scope;
the [current review](current-tone-coverage-review.md) preserves the complete
twelve-choice worksheet and its still-pending owner coverage decisions.

Open the [interactive comparison](../../tmp/muted-tone-pilot/review/index.html)
or download the [complete portable ZIP](../../tmp/muted-tone-pilot/review.zip).
The final [technical checks](../../tmp/muted-tone-pilot/checks/caption-final/checks.json),
[supplement](../../tmp/muted-tone-pilot/final-supplement.json) and
[parent review](../../tmp/muted-tone-pilot-parent-review/parent-review.json)
own the delivered verdict. Earlier build/release fingerprints are superseded
as explained below; use the final hashes in this record.

## Question and boundaries

Do Stillwell's muted light ramp and Terithia's muted medium ramp improve the
peach/orange bias of current substitutes when applied to representative
characters? The existing common proposals use Player04 for Stillwell and Wiscar
for Terithia. The pilot compares those actual current controls with Vanilla
and two isolated donor candidates, without changing selected production sets.

Sol's [priority swatch comparison](../../tmp/current-tone-coverage-assessment/priority.png)
and [assessment record](../../tmp/current-tone-coverage-assessment/assessment.json)
record the assistant's concern. These are assistant judgments, not owner visual
acceptance or proof of equivalent rendered shading. Player13/20 warm-midpoint
depth/undertone differences remain separate approximations; the pilot does not
promise complete native-tone coverage.

Use existing source-bound masks, role mapping and current output bindings.
Include enough neutral/expressive portrait, world-sprite and material-boundary
context to assess the two candidates. Any new cosmetic mapping is an explicit
temporary pilot assumption, not a full-roster policy. Ambiguous material/skin
mapping must be resolved or reported before generating candidates.

Current recipes, palette IDs/counts, runtime, installed game and sealed Linux
candidate remain unchanged. Historical and current review packets, pending
decisions, approvals and technical receipts remain frozen. This work needs no
local GPU or local model inference; a full-roster rollout is outside approval.

## Approved execution

Astra returned the [implementation plan](../plans/muted-tone-pilot.md), and Sol
checked its evaluator/classification and material contracts. The completed
20-strip comparison covers six principal characters and one Caldarus stone
boundary check. It has 41 native frames and five choices. Source colors,
provisional mappings, checks and final artifact fingerprints follow.

## Comparison and temporary assumptions

The five columns are **Vanilla, current Player04, Stillwell candidate, current
Wiscar, Terithia candidate**. Each candidate sits beside its current control.
Current control recipes/vectors are copied exactly; candidate recipes use
temporary pilot IDs and existing source-bound region entries.

The 20 strips cover Celine, Ryis, Holt, Luc and Terithia with one neutral
portrait, one expressive portrait and one Spring world strip each. Seridia
adds neutral and expressive portraits, the eight-frame Spring meditation loop
and a Flashback Priestess portrait. Caldarus adds only one Dragon Statue
portrait as an empty-mask stone no-change check. The
[plan](../plans/muted-tone-pilot.md#sample-inventory) records exact sample names
and the material reasons. These are six principal characters plus a boundary
check, not complete seventh-character or roster coverage.

Both four-color ramps come from sampled original NPC source pixels, ordered
light, middle, shadow, deep:

| Candidate | Light | Middle | Shadow | Deep |
| --- | --- | --- | --- | --- |
| Stillwell | `#E8C99E` | `#CBA282` | `#BD876C` | `#8F564C` |
| Terithia | `#D29A7D` | `#BA7E66` | `#AD7465` | `#7E4C3B` |

Noncosmetic targets retain existing role, offset and blend formulas, including
glow and glasses transitions. New cosmetics are **temporary pilot assumptions**:
Stillwell retains Player04's cosmetic offsets and Terithia retains Wiscar's.
Main/soft treatments anchor to donor light, deep treatments to donor middle;
darker flush stays `(-18,-13,-11)` from main.

| Treatment | Stillwell | Terithia |
| --- | --- | --- |
| Main blush | `#D79274` | `#B87D78` |
| Soft blush/lip | `#DE9C84` | `#C38786` |
| Deep blush/lip | `#AB4D60` | `#9A5261` |
| Darker flush | `#C58569` | `#A6706D` |

These are neither donor-native cosmetics nor an approved full-roster policy.
Applying the Terithia ramp to Terithia need not reproduce Vanilla because scar
adaptations and provisional cosmetics remain active. Actual treatment/material
hit counts are recorded below.

## Verification and preservation

The final `checks/caption-final/checks.json` and supplement establish:

- All 40 regenerated controls equal their current report-bound PNGs byte for
  byte. All 80 generated outputs pass independent literal reconstruction across
  6,300,160 RGBA pixels. Dimensions, alpha, metadata and unselected pixels are
  exact; each variant selects 60,523 pixels across the 20 source strips.
- Seven source-bound profile subsets preserve the exact selected region
  entries and all other profile fields. All 135 source slots are classified:
  86 roles, 34 noncosmetic offsets, two glasses blends and 13 cosmetic slots.
  Existing targets reproduce 854 noncosmetic and verify 91 cosmetic mappings
  across all seven current native-tone families.
- The samples exercise main/soft/deep/darker-flush cosmetics with respective
  source hit counts of 213/9/6/6, 48 glasses-blend pixels, 232 scar pixels and
  544 glow pixels. There are 152 protected landmark output checks. The separate
  Seridia test oracle confirms 68 selected pixels in each of eight meditation
  frames, 15 glow shades and 504 protected shared-color output checks. All four
  Caldarus statue outputs remain byte-identical to Vanilla with empty masks.
- Nine rejection controls cover unknown/ambiguous mappings, unknown cosmetic
  classification, existing destinations, wrong selected targets, changed
  protected hair, swapped associations, wrong assets and actual CLI checksum
  refusal. Each uses its real validator and checks the expected reason.
- Desktop 1440×1000, mobile 390×844 and an extracted direct-file mobile run each
  exercise 41 frame selections, 28 portrait crop selections and 345 exact
  canvas comparisons. Filters, empty combinations, labels and swatches pass.
  The packet has 130 members and nine required HTML links, with matching ZIP
  inventory, CRC and member hashes.
- All 3,773 frozen files remain exact, including the complete delivered current
  review, its pending decisions and the historical evidence. The original
  archive retains SHA256
  `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
  All 70 owner coverage decisions remain pending.

The retained Nix package is unavailable in this sandbox. The pilot instead
uses the existing release CLI, SHA256
`f59b46e4ab1e009f7405a984ee07d5a3303b02b4a8d07a4e339d4f82415efda8`,
after checking 372 source/palette pins against the sealed candidate, its
version, exact control regeneration and strict checksum refusal. The command
receipt records this bounded validation; no new package build or general
package-parity claim is made.

Screenshot review corrected reference/canvas alignment before final checking.
Parent review then identified an inaccurate four-role caption on Luc's
three-shade recipient sample. The final page now uses count-neutral recipient
captions and numbered sampled-shade tooltips, retaining donor four-role labels.
The [caption contract](../../tmp/muted-tone-pilot/caption-contract-checks.json)
checks all three browser runs. All 100 PNGs, 20 metadata files and 454
authoring/build/mapping pins remain exact through this presentation correction.
The [presentation](../../tmp/muted-tone-pilot/presentation-refinement.json) and
[caption](../../tmp/muted-tone-pilot/caption-refinement.json) receipts explain
the superseded initial and release packet hashes.

Parent review freshly confirms all 3,773 frozen files, 48 current input pins,
126 source/copy bindings, every ZIP member and all 651 worker-inventory file
hashes. The parent also reviewed the helper/validator code and material oracles;
no unresolved implementation defect remains in this bounded pilot.

The local helpers are under `local/authoring/muted-tone-pilot/`. The source
manifest and final receipt bind their exact bytes. Commands used for generation,
capture and the authoritative final check:

```sh
nix-shell -p python3Packages.pillow --run 'python3 local/authoring/muted-tone-pilot/build.py'
nix-shell -p python3Packages.pillow --run 'python3 local/authoring/muted-tone-pilot/capture.py'
nix-shell -p python3Packages.pillow --run 'python3 local/authoring/muted-tone-pilot/check.py --checks tmp/muted-tone-pilot/checks/caption-final'
```

These document the completed run. Builders/capture/check destinations refuse
existing paths; explicit fresh revisions must preserve the delivered packet.
Generation commands and logs are retained in the
[command receipt](../../tmp/muted-tone-pilot/commands.json). Full collection,
package, install/remove and live-game checks were not repeated for this isolated
CPU authoring slice. No local GPU or local model inference was used.

## Final artifacts and retention

| Artifact | SHA256 |
| --- | --- |
| ZIP, 1,359,831 bytes | `cb3122c4539058c331ea4464c209fa91d8246b6880642a640dc3f36eea6f1a94` |
| Caption-final checks | `4150dd585056ceb0fd79f1f979eba843645c4eef40853d5d5812584ea0ef1663` |
| [Source manifest](../../tmp/muted-tone-pilot/review/source-manifest.json) | `762d51a4a423128663d20d04fae5055e79e370cf423e0eb88c67db0d7d4753f1` |
| [Portable manifest](../../tmp/muted-tone-pilot/review/portable-manifest.json) | `e0a73b99e353441444f5e3c084a1e0273a33f3e7ff08ce1b988559972b1dcb9a` |
| [Worker delivery inventory](../../tmp/muted-tone-pilot/delivery-inventory.json) | `487d23136fc362f5645eb37171888b45bf03a52fde9a52168d32448bbd67b792` |

Retain the isolated recipes/profiles, original/control/candidate build files,
mapping and literal proofs, final packet/ZIP, logs and captures. The worker tree
uses about 11.5 MB logically and about 14 MiB allocated, plus small helpers and
separate parent expression captures. Verified superseded task packets/checks and
bytecode were cleaned; historical artifacts remain retained. The inventory
covers the worker handoff; parent checks/captures live separately under
`tmp/muted-tone-pilot-parent-review/`.

## Assistant assessment and owner decision

Sol inspected Luc's glasses, Terithia's expression/full portrait, Seridia's
eighth meditation frame, the statue boundary and mobile framing, plus final
[Celine](../../tmp/muted-tone-pilot-parent-review/celine-expression.png),
[Ryis](../../tmp/muted-tone-pilot-parent-review/ryis-expression.png) and
[Holt](../../tmp/muted-tone-pilot-parent-review/holt-expression.png) expressions.
The limited assistant assessment favors retaining both as useful candidates:
Stillwell supplies a muted beige alternative to Player04, while Terithia gives
a quieter medium alternative to Wiscar. No obvious cosmetic objection was
identified in these samples. This is not owner approval or an exhaustive
visual assessment.

### Owner feedback

On 2026-11-08, after receiving the interactive comparison and ZIP links, the
owner replied **"looks good"** to the question: "do both candidates look like
worthwhile additions, including their provisional blush and lip colors?"
Record acceptance of the displayed Stillwell and Terithia pilot candidates,
including the provisional cosmetics. The owner did not identify the format,
characters, samples or frames inspected; inspection depth remains unspecified.

The [feedback receipt](muted-tone-pilot-feedback.json) binds the exact question,
feedback, delivered files and inspection limits. A separate
[presentation snapshot](../../local/fixtures/muted-tone-pilot-presentation/review.zip)
preserves the complete reviewed portable ZIP, byte-identical to the final
artifact above. This freezes the delivered presentation; it does not turn every
contained strip/frame into an exhaustively inspected artwork baseline. The
original packet, technical receipts and candidate mappings remain unchanged.

The pilot review is complete. No full-roster rollout, permanent cosmetic policy
or broader coverage acceptance follows automatically. All 70 existing coverage
decisions and the sealed candidate remain unchanged; Player13/20 and other
unsampled differences remain separate. Commit and publication are not authorized
by this feedback.
