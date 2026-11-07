# Broaden native skin-tone coverage

2026-11-06. **Implementation and reported portrait corrections verified;
checkpoint committed. Qualified displayed-art feedback is recorded; broader
approximate-coverage assessment and release gates remain separate.**
The user asked "can we plan to do that next?" after the
[coverage assessment](../development/palette-tone-coverage.md). The user then approved execution with "let's do it". This authorizes the complete
implementation batch; new colors and approximate coverage still need the final
consolidated review. The [overview](../planning.md) owns priority; the
[original finish plan](finish.md) retains prior scope and acceptance.

## Outcome and scope

Give all 36 supported characters a practical selection spanning the depth and
meaningful undertone differences in the existing NPC and creator catalogs.
Approximate coverage means reviewed representative tones, rather than a separate
option for every catalog entry or an assertion that one donor reproduces another
character's shading exactly. Record each source ramp's representative or an
explicit remaining gap. Do not declare the expanded coverage complete while
meaningful gaps remain unaddressed or unaccepted.

Keep Vanilla, Debug Blue, existing donor IDs/labels/order and remembered-choice
compatibility. Append new choices and reuse the accepted source-bound masks for
all portraits, seasons, world strips and native mirrors. Preserve lips/cosmetics,
scars, glasses, glow/stone effects and intentional no-change assets under their
existing material rules. Any concrete defect becomes a bounded correction with
its own evidence. Earlier art approvals remain valid for the old choices.

No new characters, player customization changes, arbitrary color editor, shader
system, runtime image loader, installer GUI, new operating systems or public
publication are included. Creator colors are sources for NPC palettes; this
batch does not recolor the player.

## Initial palette budget

Start by comparing four additional representative families:

| Gap to fill | Initial source candidates, not selected colors |
| --- | --- |
| Light cream/peach | Celine, Elsie, or early creator ramps |
| Light golden undertone | Juniper or March |
| Deeper brown, with undertone chosen against existing Ryis | Creator 22–27 and existing darker NPC ramps |
| Very deep brown | Creator 31–36 |

Use the full NPC/creator catalogs to choose these representatives; the table is
a starting hypothesis. Check intermediate depths and undertones too, and avoid
spending multiple slots on visually redundant neighbors. Keep source identity
and provenance explicit when a creator tone becomes an NPC donor.

At the baseline checkpoint, the [validator](../../src/toggle.rs) accepted at most
eight generated variants; Vanilla is additional. Each character then had four generated variants, leaving room for four additions
and nine total menu choices. The
[boundary test](../../tests/presets.rs) and
[runtime navigation case](../../tests/gml/toggle_cases.gml) exercise this ceiling.
This is a technical budget, not proof that four additions cover the goal.

The first slice must identify residual gaps before full-roster work. If a
sufficient representative set needs more slots, revise the proposed count and
document the smallest necessary validator/test/UI/package change before
implementing it. Do not remove accepted choices, create mutually exclusive packs,
or mark missing tones covered merely to stay within the old ceiling.

## Selected implementation size, 2026-11-06

The first selection pass uses six additional source families: Player 04 (light
peach), March (light golden), Wiscar (warm peach), Player 16 (warm brown), Player 22
(deep brown), and Player 35 (very deep). The exact sourced colors and stable IDs
are in [the native-tone catalog](../../palettes/catalog/native-tones.json).
The coverage record owns diagnostic comparisons and remaining visual limits.
Four additions leave larger middle-tone/golden gaps. Six retain separate light,
warm-peach and deeper representatives. Wiscar also supplies the near-Adeline
family to the 32 characters that lack an Adeline donor; this deliberately
overlaps the four existing Adeline donor choices without duplicating their IDs.
These are candidate colors pending consolidated visual acceptance.

Raise the generated-preset limit from eight to ten, retaining Vanilla as the
eleventh menu choice. Update the boundary and navigation cases to exercise the
new ceiling while preserving prior one/eight behavior and rejecting eleven
generated variants. No new runtime selection mechanism or config schema is needed.
Use expanded copies of the selected sets referencing unchanged accepted profiles;
historical four-variant sets and their tests retain their original contracts.
The collection switched after the four-character pilot passed; all original
recipe files and profiles remain unchanged.

Six additions yield 63,120 generated variants on the current 6,312-strip roster.
Measure actual generation/install/atlas cost and retain the accepted 25,248 old
PNG bindings as an exact subset.

## Execution sequence

1. **Choose and document representatives.** Build a source-to-representative
   matrix for all 34 distinct sampled NPC ramps and all 36 creator ramps. Assess
   light/mid/deep shade roles, depth, warmth and undertone; color-distance ranking
   may identify gaps but cannot establish visual acceptance. Check the choices
   actually available on each character, including its own Vanilla. Record
   merged near-neighbors and remaining gaps, final candidate count, stable IDs,
   labels and source hashes. Update the owning coverage document and this plan.
2. **Pilot difficult mappings.** Adapt the candidate set on Adeline, Ryis, Reina
   and Seridia first: warm and darker sources, the accepted mouth correction,
   and glow/special-form boundaries. Reuse source masks and explicit shade roles;
   document any interpolation needed for secondary colors. Inspect portraits and
   world sprites, blush/lips, palms, fine transition shades and glow. Resolve
   candidate problems internally before full rollout. Measure generation time,
   archive/atlas growth and review size to size the remaining work.
3. **Roll out across the roster.** Append the same new donor families to all 36
   selected sets, adapting each character's existing shade-role mapping. Keep
   prior recipe entries and accepted output bytes exact. Audit duplicate or
   near-Vanilla choices explicitly; do not silently drop IDs per character. Build
   complete per-character comparisons and a small overview of the new families.
4. **Integrate and verify the complete batch.** Run the existing standard checks,
   affected material/recipe and runtime cases, and full strict generation and
   disposable installation/removal. Verify every new variant's pixels/alpha/
   metadata and compare every prior accepted variant's actual PNG hash. Preserve
   unchanged spectral/shadow assets and exact source-archive restoration. Exercise
   all choices in the native menu, independent character selection, persistence
   from an old config, reset, opt-out/restart and failure feedback. Update the
   supported Linux candidate and documentation only with matching evidence.
5. **Present one consolidated review.** Finish the active batch and checks before
   asking for feedback. Supply a source/reference row, old accepted choices and
   new tones in a clearly labeled portable gallery, with full per-character
   pages linked from the overview. Record acceptance of the new colors and
   approximate coverage separately from automated verification. Freeze accepted
   new outputs without rewriting prior snapshots or historical packets.

The first full run at four additions would double generated variant count from
25,248 to 50,496 with the existing strip inventory. Treat build/install/atlas size
as a measured pilot concern; do not promise unchanged resource use or invent an
elapsed-time estimate before measuring it.

## Native compilation boundary discovered during integration

The first full ten-variant strict install exceeds the pinned compiler's
per-function constant pool in the single generated `lns_palette_definitions`
literal. Its verbose panic also stalls the pinned MOMI stdout/stderr pipes.
No modified archive is published. The owning coverage record retains failure
and diagnostic evidence.

The bounded correction partitions a large table into per-character literal
helper functions while retaining the public function's row order and exact data
shape. Safe small-table output retains its prior literal form to preserve
historical byte/test contracts. No compiler check is bypassed and the pinned
MOMI version stays unchanged. The real emitted-large-table compile/evaluation regression and fresh exact-pinned
strict installation now pass. Final generated-image hashes match the frozen
PNG/metadata gallery, and installed native-frame reconstruction passes. The
[coverage record](../development/palette-tone-coverage.md) owns timings and exact
bindings. Expanded live input/restart/rendering checks and exact removal now
also pass; human acceptance remains separate.

## Ownership and resources

Catalog selection, shade mapping, generation, static comparisons, tests and
package checks use **CPU; no local GPU or local model inference is needed**.
Actual game rendering and input checks require **a local GPU**, fresh
`gpu-status` and a foreground `gpu-run` reservation. Keep real installations,
saves/settings and other GPU workloads untouched.

Two authors may own disjoint character-set
files and their affected tests. One integrator owns the common donor definition,
collection selection, counts, runtime changes if justified, combined checks and
acceptance records. Do not run overlapping full builds against one output tree.
Use the existing task-level review policy if substantial runtime behavior changes;
data-only target additions do not restart an already completed runtime review.

## Completion and handoff

- Every sampled native ramp has a documented representative or explicitly
  accepted limitation; coverage claims identify the sampled catalog's limits.
- The chosen new families work across all 36 NPCs and both portrait/world art.
  Old accepted outputs, stable IDs, defaults and saved choices remain valid.
- New art has recorded user acceptance after the complete batch review; passing
  checks alone do not supply it. Freeze new accepted baselines separately.
- The expanded candidate passes its own install/remove, metadata, native menu
  and persistence checks, with exact original-archive restoration.
- Remaining physical-controller and broader seasonal/action release checks,
  final compatibility notes and licensing choice stay visible in the original
  finish plan. Original alpha and displayed-live approvals are preserved;
  publication still needs separate authorization.


## Current handoff, 2026-11-06

Implementation and all reported portrait defects are resolved and technically
verified. The user calls the final Holt edge revision "much better", with no
additional defect reported. Record qualified positive feedback on the displayed
artwork and unspecified inspection depth; do not infer inspection of every
expression, tone, frame or season, complete approximate-coverage approval, or new
live-game acceptance. The user subsequently authorized the checkpoint commit
with "do it".

| Completed work | Evidence and recorded limits |
| --- | --- |
| Six source families across all 36 characters; full strict installation and expanded native controls | [Coverage and integration record](../development/palette-tone-coverage.md); initial "not bad" feedback, with later defects handled below |
| Celine blush, mouth/eye contours and visible nose blush | [Celine facial record](../development/celine-native-tone-face.md); qualified "much better" feedback |
| Expressive roster audit, confirmed blush and sporadic mouth corrections | [Roster record](../development/native-tone-roster-face-correction.md); 31 corrected, four audited unchanged, Celine preserved; "otherwise looks pretty good" feedback |
| Adeline's two missed cheek islands and Holt's pre-existing moustache gap | [Focused follow-up](../development/adeline-holt-face-followup.md); Adeline preserved in subsequent work; qualified positive feedback with Holt edge issue carried forward |
| Holt's skin/hair boundary | [Material record](../development/holt-moustache-edge.md) and [final integration](../development/holt-moustache-edge-integration.md); final "much better" feedback, with no new issue reported |

Detailed source selections, counts, timings, failure/after proofs and historical
review packets stay in these owning records. Earlier approvals and their
qualifications remain intact. This checkpoint includes the verified expansion,
reported corrections and planning updates; public distribution remains separate.

## Standing correction decisions

- The validated offline [preset profile override](../development/palette-presets.md)
  allows the six additions to select source-bound facial masks with the same
  asset inventory, preserving previous skin targets and earlier choices.
- Holt's current-only base keeps historical recipes/profiles unchanged. The sole
  original-choice exception is 64 portrait PNGs / 2,304 true moustache-hair pixels;
  all other originally accepted outputs remain exact. The final integration
  record owns the exact proof and qualified assessment.
- Frozen prior reviews, source candidates, accepted snapshots and verification
  packets are historical evidence. Add current assessment records rather than
  rewriting those packets or treating automated checks as visual approval.

## Remaining work

2026-11-07: the owner approved a small CPU comparison of Player 27 and Player 30
after Sol identified a residual deep umber undertone gap. The
[five-character pilot](../development/deep-umber-pilot.md) is prepared and
technically verified. Following Sol's proposal of one Player 27 addition, the
owner delegated the decision with "up to you". Sol selects Player 27 and proceeds
with the authorized [single-family rollout](player27-rollout.md), now implemented
and technically verified. All 63,120 previous PNGs remain exact; 6,312 new PNGs,
strict full installation/removal and twelve-choice native persistence checks
pass. The complete comparison remains available. The requested
[representative sheet](../development/player27-review-sheet.md#owner-feedback)
has qualified acceptance of the displayed 36-character neutral portrait sample;
broader artwork and coverage acceptance are not inferred.
The pilot's isolated recipes remain unchanged. Earlier qualified
feedback is preserved; delegated selection supplies no new coverage acceptance.

The owner approved the [compact coverage review](native-tone-coverage-review.md)
after its planning pass. The worksheet and pending decision record are prepared
and technically verified using existing diagnostics and current artwork. Owner
coverage assessment remains pending; no local GPU was required. The
[coverage record](../development/palette-tone-coverage.md#coverage-worksheet-prepared)
owns verification and inspection limits.

The six-family checkpoint and verified Player27 follow-up are committed locally.
The commits required no local GPU or model inference. Broader
approximate-coverage assessment and any newly accepted baselines remain separate
from qualified feedback on the displayed corrections. Remaining physical-controller,
seasonal/action, compatibility and licensing/release work is owned by the
[original finish plan](finish.md). Further rendered gameplay/input checks require
checked local GPU availability and a reservation. Public publication still needs
separate authorization.
