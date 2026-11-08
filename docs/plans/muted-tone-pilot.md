# Stillwell and Terithia muted-tone pilot

2026-11-08. **Pilot complete: technically verified and accepted as displayed.** The owner
said "go for it" after Sol recommended keeping all current choices and testing
Stillwell and Terithia as muted light/medium donor candidates. This authorizes
the isolated pilot through execution, without a further planning checkpoint.
It does not authorize full-roster additions or runtime changes. The owner later
said "looks good" to the displayed candidates and provisional cosmetics;
[the owning record](../development/muted-tone-pilot.md#owner-feedback) preserves
the exact feedback, artifact binding and unspecified inspection depth.
The [overview](../planning.md) owns priority; the
[native-tone plan](native-tone-coverage.md) owns the coverage goal. The
[development record](../development/muted-tone-pilot.md) owns actual execution,
fingerprints, assessment and verification limits.

The [interactive comparison](../../tmp/muted-tone-pilot/review/index.html) and
[portable download](../../tmp/muted-tone-pilot/review.zip) are complete. The
development record owns final checks and fingerprints, including the corrected
Luc source-shade caption. The ordered steps below preserve the approved plan.

Astra investigated the current source/mapping/control contracts read-only.
Sol checked the role evaluator, Player27 classification rationale and material
rules, and accepts this approach within the approved scope.

## Outcome and boundaries

Prepare one portable five-column comparison in this order:
**Vanilla → current Player04 → Stillwell candidate → current Wiscar → Terithia
candidate**. Show whether the muted donors offer useful differences from the
existing peach-based choices, with cosmetic and material context.

Use existing source-bound profiles, subset only their selected region inventory
and generate four isolated variants per recipient. Keep current production
recipes/profiles, IDs/counts, runtime, installed game, historical/current reviews,
pending coverage decisions and sealed Linux candidate unchanged. Earlier visual
feedback retains its exact scope; a request to execute is not visual acceptance.

No local GPU or local model inference is needed. One Sol implementer can own
the bounded authoring/check helpers and outputs; the parent owns documentation
and final review. Full collection generation, package builds, installation and
game sessions are outside this pilot.

## Sample inventory

| Character | Samples | Material purpose |
| --- | --- | --- |
| Celine | Spring neutral, `blush_closed_eyes`, idle south | Light source, corrected contours, all three cosmetic tiers |
| Ryis | Spring neutral, `closed_eyes_smile_blush`, idle south | Darker source under lighter donor mapping |
| Holt | Spring neutral, `embarrassed`, idle south | Accepted moustache/skin boundary and exposed skin |
| Luc | Spring neutral, `embarrassed`, idle south | Protected glasses and two skin/glasses blends |
| Terithia | Spring neutral, `embarrassed`, idle south | Scar shading, both flush tiers and donor on its own character |
| Seridia | Spring neutral, `closed_eyes_blush`, Spring meditation loop south; Flashback Priestess `closed_eyes` | Gray source, glow offsets and lip/material boundary |
| Caldarus | Dragon Statue `neutral` only | Empty-mask stone no-change boundary check |

Resolve ordinary sample paths through the current worksheet and
`tmp/native-tone-expression-review/selection.json`. Explicit exceptional paths:

- `spr_npc_seridia_specialanimation_spring_meditate_loop_south.png` replaces
  the ordinary Seridia world idle.
- `spr_portrait_seridia_flashback_priestess_closed_eyes.png` is added.
- `spr_portrait_caldarus_dragon_statue_neutral.png` is added.

There are 20 strips and 41 native frames: 40 candidate PNGs, 40 regenerated
current controls and 20 Vanilla PNGs, or 205 native frame/choice views. Caldarus
is a single boundary check, not a full seventh-character review. The meditation
strip's eight frames exercise 15 of the established 25 glow shades. Reina's
corrected mouth and Nora/Maple's separate lip treatments are outside this sample.
Planning verified sample/control availability and common effective profiles;
execution must recheck its actual inputs.

## Mapping contracts and provisional cosmetics

| Donor | Light | Middle | Shadow | Deep |
| --- | --- | --- | --- | --- |
| Stillwell | `#E8C99E` | `#CBA282` | `#BD876C` | `#8F564C` |
| Terithia | `#D29A7D` | `#BA7E66` | `#AD7465` | `#7E4C3B` |

Bind each exact ramp to `npc_stillwell` or `npc_terithia` in the NPC catalog,
including archive/source coordinates. Use temporary IDs such as
`pilot_stillwell` and `pilot_terithia`; no permanent menu ID is selected.

Use the semantic slot classifications in
[Player27 mappings](../development/player27-mappings.json). Noncosmetic `role`,
`role_offset` and `blend` formulas retain positive half-up integer rounding and
channel clamping, as implemented by the existing
[evaluator](../../local/authoring/native-tones/append-presets.py). Before deriving
candidates, reproduce these mappings against all seven current donor families.
Verify every selected classification and both actual control vectors. Unknown
or inconsistent slots fail preparation; do not substitute nearest roles.

The [Player27 preparation](../../local/authoring/player27-data/prepare.py) records
cosmetic classifications, but its numeric offsets are anchored to Player22.
For this pilot, anchor Stillwell to current Player04 and Terithia to current
Wiscar. Main/soft offsets are relative to donor light; deep offsets to donor
middle. Preserve darker flush at `(-18,-13,-11)` from main.

| Temporary treatment | Stillwell | Terithia |
| --- | --- | --- |
| Main blush | `#D79274FF` | `#B87D78FF` |
| Soft blush/lip | `#DE9C84FF` | `#C38786FF` |
| Deep blush/lip | `#AB4D60FF` | `#9A5261FF` |
| Darker flush | `#C58569FF` | `#A6706DFF` |

These are explicit provisional pilot assumptions, not native donor cosmetics or
an approved full-roster policy. Terithia-on-Terithia need not equal Vanilla:
scar adaptations and provisional cosmetics remain part of the mapping.
Preserve the established [Luc](../development/luc-portraits.md),
[Seridia](../development/seridia-world.md#material-and-palette-decisions) and
[Caldarus](../development/caldarus-world.md) material contracts.

## Implementation and acceptance

1. **Pin inputs and preserve old evidence.** Use fresh
   `local/authoring/muted-tone-pilot/` and `tmp/muted-tone-pilot/` destinations.
   Refuse existing final paths. Extend the current review preservation baseline
   with its complete delivered worksheet/ZIP/decisions/checks, the assessment,
   old deep-umber pilot, relevant mappings/helpers and sampled current artwork.
   Preserve historical and sealed-candidate pins. Parent-owned overview and
   planning/status prose may receive authorized updates; review those diffs
   separately. Recheck the rollout integration → artwork → current report
   chain, selected recipes and effective profiles. Parse the 116.5 MB report
   once per process.
2. **Generate source-bound isolated variants.** Copy selected original PNGs and
   metadata. Filter profiles by exact selected region inventory only, retaining
   region order, source colors/groups, seeds, dimensions and source hashes.
   Preserve exact Player04/Wiscar vectors and labels; derive candidate vectors
   separately. One shared subset profile per recipient suffices because both
   controls use the same current effective profile. Run the existing release
   CLI's strict `build-presets` path for four variants. Independently verify
   source pixels for both candidates and both controls against archive members
   and catalog coordinates; pin the original archive before/after.
3. **Verify pixels and materials independently.** Adapt the component decoder
   from [the current proof](../../local/authoring/player27-rollout/verify-artwork.py)
   without running its full-roster writer. Require all 40 control PNG hashes to
   equal current report-bound files, all 80 generated RGBA outputs to equal
   independent reconstruction, and dimensions/alpha/metadata/unselected pixels
   to remain exact. Region entries and independent selected-pixel totals must
   agree with CLI reports. Record actual cosmetic, glasses-blend, scar and glow
   hit counts. Verify protected glasses, Holt hair, true mouth interiors and
   non-skin glow using existing landmarks. Caldarus's mask remains empty and
   all four statue outputs unchanged.
4. **Exercise real rejection paths.** Meaningful negative controls must reject
   ambiguous/unknown mapping, wrong targets, changed protected pixels, source
   checksum mismatch, swapped control/candidate associations and wrong asset
   routing. Use actual validators and assert intended failure reasons;
   comparisons of artificially unequal buffers alone are insufficient.
5. **Build the portable comparison.** Adapt the historical small pilot into the
   new namespace, preserving adjacent control/candidate columns and visibly
   marking provisional cosmetics and the stone-only check. Include four-role
   source swatches, character/sample filters, all native frames, enlarged detail
   and full portraits. Bind images to character/asset/choice/profile/report/hash.
   Retain `data.json`/`data.js` direct-file loading. Independently decode expected
   premultiplied-RGBA for every native frame and portrait crop; this includes
   all eight Seridia meditation frames.
6. **Complete checks and one consolidated review.** CPU Chromium at desktop and
   390×844 mobile must exercise filters, character and frame selections, full
   portrait views, empty combinations, correct labels/swatches and exact canvas
   pixels. Inspect representative screenshots. Verify ZIP inventory/CRC/hashes,
   required local links and interaction checks on an extraction outside the
   checkout by direct file. Run final preservation, documentation-link and
   diff checks. Record actual results/fingerprints in the development record;
   keep all 70 current coverage decisions pending. Present one comparison
   asking whether each candidate improves its adjacent current control and
   whether its provisional cosmetics introduce an objection.

## Alternatives, resources and remaining limits

The old five-character pilot omits glasses blends and darker flush; its ordinary
idle does not test glow. The selected 20-strip inventory covers these boundaries
without full-roster authoring. Do not run historical pilot/preparation entry
points: they overwrite old outputs and assume older catalog/report bindings.
Adapt small helpers or import only safe functions.

The sampled originals and two controls total about 1.22 MB. Expect a modest
review packet and a build/check tree in tens of MiB; measure actual use. Retain
subset profiles/recipes, mapping/pixel proofs, final review/ZIP, logs and a few
screenshots. Remove only verified disposable task-created scratch.

Optional ranking evidence must first reproduce current diagnostic assignments
and retain the existing method; it supplies no threshold, coverage percentage
or visual acceptance. It is not needed to complete the visual comparison.
The bounded implementation, material proof and CPU browser/portable checks are
complete. No technical blocker remains; displayed pilot candidates and their
provisional cosmetics are accepted with inspection depth unspecified.
Production changes or a full-roster rollout require separate
authorization. Later feedback does not by itself close Player13/20 or other
remaining coverage questions.
