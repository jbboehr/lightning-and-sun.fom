# Native-tone coverage review

2026-11-06. **Worksheet prepared and technically verified; owner coverage
assessment pending.** The owner approved Astra planning, then separately
approved worksheet preparation with "go for it" after reviewing this proposal.
Sol implemented the bounded slice without palette changes. The
[worksheet](../../tmp/native-tone-coverage-review/index.html),
[download](../../tmp/native-tone-coverage-review.zip) and
[verification record](../development/palette-tone-coverage.md#coverage-worksheet-prepared)
own the delivered result and its limits.
The [overview](../planning.md) owns priority, the
[native-tone plan](native-tone-coverage.md) owns expansion scope, and the
[coverage record](../development/palette-tone-coverage.md#coverage-review-planning)
owns investigation evidence and limits.

## Intended outcome and boundaries

Make it practical for the owner to judge whether the current choices adequately
represent the sampled NPC and creator tone range. Each source ramp needs a
reviewed representative or an explicit remaining gap or accepted limitation.
Assess lightness, undertone and shade contrast alongside numerical similarity.

Reuse the current catalog, diagnostics and generated artwork. Add one compact
worksheet and a decision record. No new palettes, facial corrections, runtime
changes, full-gallery rebuild, dependencies or color metric are included.
Physical-controller, seasonal/action, compatibility, licensing and publication
work remain separate. This slice needs no local GPU or local model inference.

## Review approach

The matrix covers 72 catalog entries against 36 characters' actual choices;
those entries contain 70 distinct sampled ramps. Reference-ramp distance does
not measure a recolored character's appearance. Ordered source colors are not
interchangeable artistic roles; see the [sampling limits](../development/npc-palettes.md)
and [diagnostic method](../development/palette-tone-coverage.md#approved-expansion-and-candidate-selection-2026-11-06).

Every character can access eight common natural reference families and special
gray through a donor or its own Vanilla. Propose one common representative per
distinct source ramp to reduce repeated decisions. Preserve the existing matrix
unchanged and expose nearer character-specific alternatives, including Vanilla,
plus explicit review exceptions. A common family does not imply identical
rendered shade roles across characters.

Group source/representative swatches by family, with priority comparisons first
and all 70 ramps accessible. Preserve all 72 identities, including duplicates.
Identify special gray separately; Debug Blue does not establish natural-tone
coverage. Resolve reference families to actual available menu choices and labels.

| Sources | Candidate comparison | Review concern, not an established defect |
| --- | --- | --- |
| Player 13 and 20 | Player 16 | Largest current diagnostic gaps |
| Stillwell; Holt/Olric/Wheedle; Terithia | Player 04; Hayden; Wiscar | Contrast and muted intermediate tones |
| Player 02/03 and 29/30 | Player 04 and Player 22 | Light and deep undertones |
| Player 31/34/36; irregular NPC ramps | Player 35; existing assignments | Endpoints and differing shade roles |

Provide one neutral portrait and one suitable world frame per character, with
a selector for relevant current choices and links to the complete galleries.
These samples give context for tone judgments without establishing acceptance
of every frame or outfit.

## Preparation tasks

1. **Bind inputs and propose representatives.** Verify catalog pins and selected
   recipe/profile bindings. Create a small decision record, provisionally
   `docs/development/native-tone-coverage-review.json`, referencing the unchanged
   matrix. Record source IDs, common representatives, actual per-character
   choices, alternatives, exceptions, pending review status and evidence links.
   Expand defaults and exceptions to all 2,592 combinations; avoid another
   handwritten matrix or an invented numerical acceptance threshold.
2. **Prepare the worksheet.** Write `tmp/native-tone-coverage-review/index.html`.
   Reuse the existing swatch and exact-frame approach with a small local helper
   if needed. Copy only selected existing images and pin their hashes. Resolve
   final Holt, Adeline, Celine and other current artwork through the
   [composite report](../../tmp/holt-moustache-edge/current-roster-report.json)
   and [candidate-source binding](../../tmp/holt-moustache-edge/roster-binding.json).
   Resolve Vanilla samples from source-bound originals with provenance. Link the
   [current portrait index](../../tmp/native-tones/current-review.html) and
   [complete gallery](../../tmp/native-tones/review/index.html); its world art is
   unchanged. Historical portrait "Before" columns must not stand for current
   outputs. Do not build a review framework.
3. **Validate and present.** Complete the checks below, then ask one consolidated
   question about the displayed range and named approximations. Identify the
   artifact, layout and references; the owner should not need to reconcile raw
   JSON or interpret distance scores.
4. **Record the assessment.** Preserve exact feedback, reviewed grouping,
   artifact fingerprint and inspection limits in the decision record and owning
   coverage document. Update the plan and overview when status changes. A
   rejected approximation becomes a named gap and separate proposal. Freeze
   only specifically accepted evidence; retain historical packets and limits.

Each task depends on the preceding task. Keep the decision record and worksheet
under one owner; this modest slice does not need parallel workers.

## Acceptance and validation

- Every source/character combination resolves exactly once to an available
  choice or explicit gap. Duplicate ramps retain provenance. Natural-tone
  coverage cannot depend on Debug Blue.
- Catalog, recipe/profile and selected PNG/metadata bindings match current
  inputs. Generated image hashes agree with the composite report; Vanilla
  samples agree with source evidence. Labels, native frame boundaries and
  selected palettes agree with those bindings.
- Common defaults and exceptions retain diagnostic alternatives without
  silently relabeling them as reviewed assignments.
- Links, selectors and image loading pass a focused CPU browser check at
  desktop and phone sizes. Focused negative checks reject an unavailable
  representative and a stale image binding.

Inspect the final diff and local links. Existing full build, install and runtime
checks need no repetition for this report-only slice. Investigate a discovered
binding discrepancy before broadening validation or changing artwork.

Owner feedback establishes adequacy of the sampled range with the recorded
approximations. It does not establish uninspected expression, seasonal/outfit or
live-game acceptance. A documented missing family is a valid assessment result;
coverage completion requires addressing the gap or explicit owner acceptance
of the limitation. Do not create an unrestricted art baseline from coverage
feedback.

## Alternatives and remaining decisions

The existing swatches and raw matrix permit manual review with less preparation,
but require the owner to reconcile sources, choices and current images. Reviewing
all 216 distinct diagnostic pairs individually adds repetition. The proposed
worksheet retains those alternatives while making the main decisions readable.
Rebuilding the full gallery or optimizing new colors is unsupported by current
findings.

Preparation and technical validation are complete; no technical blocker remains.
The owner now judges whether the displayed range is adequate, with limitations
stated explicitly. All entries in the
[decision record](../development/native-tone-coverage-review.json) remain pending
until that feedback. New defects or missing families require a bounded proposal
of their own.

## Approved follow-up pilot, 2026-11-07

After reviewing all source/reference swatches and selected current NPC examples,
Sol identified deep umber as the clearest remaining undertone gap and recommended
a small comparison. The owner replied "sure". The
[Player 27/30 pilot](../development/deep-umber-pilot.md) is prepared and technically
verified. Its original approval authorized only the offline comparison. After
Sol proposed one Player 27 addition retaining all existing choices, the owner
delegated the decision with "up to you". Sol selects Player 27; the
[single-family rollout](player27-rollout.md) is authorized and in planning.
The coverage decision record remains pending; no owner visual approval is inferred.
