# Player 27: one deep umber addition

2026-11-07. **Committed locally; technically verified; representative portraits accepted with qualifications.**
The owner delegated
the choice with "up to you" after Sol proposed one Player 27 addition while
retaining every existing palette. Sol selected Player 27. This authorizes the
bounded [rollout](../plans/player27-rollout.md); it records no new owner visual,
approximate-coverage or live-game acceptance. Subsequent qualified representative
feedback is recorded in the [sheet record](player27-review-sheet.md#owner-feedback).
Player 30 remains a pilot alternative.

## Source and preservation contracts

Append `player_27`, label `Deep umber (Player 27)`, with the exact creator ramp:
`#813723FF`, `#582718FF`, `#431D11FF`, `#2D130AFF`. All 36 characters retain
their existing ten generated entries in exact order and gain one entry. Vanilla
remains index zero; the result is twelve overall choices. Each new entry uses
the same effective profile as the character's current Player 22 entry. Masks,
selected assets, old targets, defaults and version-1 settings remain unchanged.

Before implementation, the [baseline receipt](../../tmp/player27-rollout/baseline.json)
verified all 63,120 actual current PNGs and metadata against the final Holt
composite, plus all 6,312 original PNGs and metadata. It pins 1,171 production,
palette and test inputs and snapshots current recipes/catalog/target records.
The canonical PNG-binding digest is
`8dcab47656f38c879adc9a48e7ffa17caba56985aa010ebaf09d586bf6259296`;
it hashes binding tuples, rather than the report file. Original archive SHA256:
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The actual current baseline includes the final Holt classification and its
documented historical exceptions. Earlier accepted snapshots, correction
records and frozen review packets remain intact. The six-family catalog pin
`1cb71c3f49b88acb9540d0eb4a6658efb03aadddd555069a341cbc6e313b12be`
resolves to the exact [historical catalog fixture](../../tests/fixtures/player27-six-family-catalog.json)
when reproducing the historical matrix. The [separate diagnostic supplement](../../tmp/player27-rollout/reference-diagnostics.json)
reproduces all 2,592 old assignments before comparing Player27. It improves the
reference ranking for Player24/27/30 across all 36 characters (108 assignments).
This remains a reference diagnostic, not a measured coverage percentage. Existing worksheet
decisions remain pending; reference distances establish no visual threshold.

## Material authoring

Astra's read-only planning resolved all 524 current source slots: 456 baseline
slots and 68 facial additions. Existing targets must reproduce before new
values are derived. Direct roles, offsets, blend formulas, half-up rounding and
clamping follow the established authoring rules. Preserve Reina's corrected
mouth role, Holt's hair/skin distinction and Seridia's 25 glow offsets.

The following are necessary authoring choices within the delegated palette
selection, subject to the complete artwork review:

| Treatment | New target | Derivation |
| --- | --- | --- |
| Main blush | `#964B3EFF` | Pilot's Player22-relative light offset |
| Soft blush/lip | `#A55B4EFF` | Pilot's Player22-relative light offset |
| Deep blush/lip | `#6F3231FF` | Pilot's Player22-relative middle offset |
| Caldarus/Terithia darker flush | `#843E33FF` | Main blush plus existing `(-18,-13,-11)` |
| Nora lip highlight | `#814538FF` | Existing Player22 lip offset from light |
| Nora lip middle/extra alias | `#582C2EFF` | Existing Player22 lip offset from middle |
| Maple lip | `#523632FF` | Existing Player22 lip offset from middle |

Celine retains her 23-slot facial profile, including its mouth, eyes and blush
roles. Nora's additional lip slot aliases her lip-middle target. New derivations
belong to a separate Player27 record; historical six-family records retain their
original evidence and scope. The [Player27 mapping record](player27-mappings.json)
contains every slot's source, derivation, effective-profile hash and new target.

## Completed CPU checks

The [data receipt](../../tmp/player27-rollout/data-checks.json) records 36
eleven-entry sets, 360 exact prior preset entries and 68 unchanged effective
profile hashes. The authoring helper reconstructs the old six-family prefix
before verifying all eleven current choices. Ten focused normal tests and five
affected opted-in facial tests pass; the historical corpus assertions and counts
remain bound to their original ten IDs.

The full collection build took 95.458 seconds. The [artwork proof](../../tmp/player27-rollout/artwork-checks.json)
verifies all 69,432 generated PNGs and metadata against the build report, all
63,120 prior PNGs against the frozen final Holt composite, and every new PNG
against independent component reconstruction. Its 323,581,760 literal RGBA pixel
comparisons pass. All source profiles, selected inventories, dimensions, alpha,
metadata and unselected pixels are exact. Wrong-target and changed-protected-pixel
negative controls are detected for every character. The same 6,312 strips contain
18,315 native frames.

The [standard check receipt](../../tmp/player27-rollout/standard-check-summary.json)
records formatting, locked all-target Clippy with denied warnings, 466 regular
tests (520 ignored) and the locked release build passing. All seventeen focused
CLI/runtime/control cases pass, including the observed eleven-preset failure
before the validator change. The [actual full-table check](../../tmp/player27-rollout/emitter-checks.log)
also passes all three emitter cases with the current twelve-choice definitions.
During verification, new fixtures had intent-to-add entries so the root Git
flake included them, while the staged content list remained empty. Commit
authorization followed later; see the [local checkpoint](#local-checkpoint).

The [single combined correctness review](../../tmp/player27-rollout/correctness-review.json)
returned PASS with no concrete findings. A fresh read-only Breaker checked all
53 frozen task hashes, preservation fixtures, target derivations and affected
production paths. No additional review pass or production fix was needed.

## Complete comparison packet

Open the [complete gallery](../../tmp/player27-rollout/review/index.html),
[expressive cosmetic focus](../../tmp/player27-rollout/review/cosmetics.html), or
[portable ZIP](../../tmp/player27-rollout/review.zip) (183.1 MiB). Columns read
Vanilla, current Player22, Player27 and current Player35. All other game choices
remain available; this packet focuses on the addition and its dark comparisons.
The [historical context index](../../tmp/player27-rollout/review-context.html)
links earlier frozen packets without copying them into the new ZIP.

The [review checks](../../tmp/player27-rollout/review-checks.json) verify all
6,312 strips, 18,315 native frames and 4,853 applicable West mirrors: 92,672
four-choice frame/view cases. All 25,248 PNGs and 6,312 metadata files bind to
their actual generated reports and source inputs. The 31,743 portable entries,
31,665 source bindings and 297 internal HTML links pass. Seventy-seven CPU
Chromium loads cover every page at desktop and phone widths plus a direct-file
deep link, with controls, filtering, pagination, frame/wrap/West/detail actions
and 2,528 exact premultiplied-RGBA comparisons. Premultiplication accounts for
native partial alpha; no pixels are skipped. Missing-choice, wrong-binding and
stale-RGBA negative cases are rejected.

ZIP SHA256:
`93cccd5f4c70aae08344d7327996083d7f0f8dcf3174a8181e9a4b001f03397d`.

Sol inspected the two expressive contact sheets (Celine/Nora/Maple/Caldarus and
Terithia/Holt/Seridia/Priestess), desktop and phone screens, and both native
frames of the Flashback Priestess closed-eyes strip. No concrete defect was
observed in those samples. Player27 reads as a warmer umber neighbor to Player22
with visible facial shading; this is the assistant's bounded assessment, not
owner art acceptance. All sixteen Player27 PNGs from the earlier pilot reproduce
exactly in the [full-build comparison](../../tmp/player27-rollout/pilot-reproduction.json).

## Package and native verification

Root and source-only standalone Nix builds resolve to the same package,
`/nix/store/il1nhcfkdfsvlg2dp9d7a8acxxnhl151-mistria-palette-0.1.0`.
All 355 packaged palette JSON files match current source; see the
[package binding](../../tmp/player27-rollout/package-binding.json). This uses the
same machine/store, not a fresh operating-system installation. The first offline
attempt lacked cached dependencies and failed in an upstream bootstrap-source
hash check; the normal online build passed without changing pins or checks.

The [source-only candidate](../../tmp/player27-rollout/lightning-and-sun-linux-candidate.tar.gz)
contains 1,832 text entries, 155,728,258 uncompressed bytes and 8,790,218 archive
bytes, with no game images, archives or local state. Its source snapshot precedes
later verification-document updates; production, palette and test inputs match.
The [full strict installation](../../tmp/player27-rollout/install-binding.json)
passes with unmodified pinned MOMI 0.16.4 and required compilation, in 505.388
seconds. All 69,432 generated report bindings match the literal-verified build;
all 6,312 original PNGs and metadata remain byte-exact. Production atlas checks
verify 219,780 original/generated native frames. The installed table matches
the current build: 36 helpers, 3,815,193 bytes, SHA256
`5f43b73f33f3803442cc1da3c0918bd39806cf6867c1b6863fb279fd74264cc4`.
No pipe intervention, relaxed compilation or alternative compiler counts as
this result.

Separate retained production-code audit harnesses verify all 18 spectral strips
(48 frames) and 30 shadow strips (149 frames): original loose PNGs, parsed native
metadata and visible atlas pixels are unchanged. Hidden RGB below zero alpha
may differ after normal atlas trimming; alpha and visible RGBA must be exact.
The installed archive grows from 629,768,271 to 877,797,753 bytes; atlas pages
grow from 11 to 81. These are measured package costs, not visual acceptance.

The first supervisor preflight found a
dangling cached test-tool link and stopped before launching a game. Rebuilding
the existing pinned tool expression restored the exact same tool-store path;
executables, libraries and AMD driver paths were checked before retrying.
The failed preflight contains no game state. Actual sessions run with separate
state and a foreground GPU reservation; other host workloads remain untouched.

The [native check packet](../../tmp/player27-rollout/live-checks.json) passes all
three sessions on the RX 9070 XT with radeonsi/Mesa 26.1.8. Actual OS
keyboard/mouse and SDL virtual-gamepad events exercise the menu; no palette
callback/API is dispatched directly. Forty-four raw captures and input logs
retain the bounded evidence:

- The old all-36 version-1 configuration restores byte-exactly without startup
  writes. Every one of the twelve choices is reachable and wraps; the other
  35 selections remain unchanged.
- A real read-only directory triggers save-failure feedback and failed opt-out
  without changing the primary file. Reset saves all Vanilla; Player27 saves
  independently and restores on restart without initialization writes.
- Opt-out retains Player27 in the current session. The subsequent restart
  starts all 36 characters on Vanilla. Cycling with remembering Off leaves
  saved bytes exact.
- Normal prologue dialogue renders Adeline's Player27 portrait and world sprite,
  while Eiland remains Vanilla; see the [native capture](../../tmp/live-release/player27-remembered/normal-prologue-adeline-player27.png).

Game exits are 0, intentional supervisor stop -15 after the remembered-session
prologue capture, and 0. All three compositor supervisors return 0. Owned game
and supervisor processes are absent, reservations have returned, and the final
GPU snapshot matches the checked desktop process baseline. Isolated Steam
initialization diagnostics match the previous baseline; no unexpected runtime
error or palette-disabled warning is present. Physical controllers, Steam Cloud,
broader seasonal/action playthroughs and other platforms/mod combinations remain
outside this result.

Packaged [removal](../../tmp/player27-rollout/uninstall-binding.json) restores
the exact original archive SHA256 and removes the recovery directory. The
preserved source archive is unchanged. Real user settings and saves were never
mounted. No commit or publication had been performed during that verification pass.

## Retention and handoff

The [resource record](../../tmp/player27-rollout/resource-costs.json) measures
2.12 GB logical / 3.00 GB allocated for the single complete build, 0.97 GB logical
/ 1.06 GB allocated for this batch's review/source/proof directory, and about
22.5 MB logical for the three private live sessions. The complete review is
423.5 MB logical, including 293.7 MB media and self-contained recipe/profile
evidence; its ZIP is 192.0 MB. These remain for review and reproduction. Installer
transaction scratch is automatically removed; the reused lab is restored.
Only the review worker's disposable experiments were cleaned. Earlier assets,+caches, accepted snapshots and frozen packets remain intact.

The mutable current-review routing index now links Player27 and labels earlier
ten-choice portrait pages as unchanged historical context. Its preceding bytes
are retained in the batch snapshot; old frozen packets were not rewritten.
The consolidated receipt is `tmp/player27-rollout/integration-checks.json`.

The implementation scope is complete. The owner requested a compact sheet of
all 36 characters and responded "looks okay to me". The
[sheet record](player27-review-sheet.md#owner-feedback) owns qualified acceptance
of the displayed neutral Spring/frame2 crops, the pinned artifacts and separate
presentation snapshot. Inspection depth is unspecified; unsampled artwork,
broader coverage and the remaining release/publication gates retain
their separate scope. Historical technical receipts retain their original
pre-feedback limits.

## Local checkpoint

On 2026-11-07, after the readiness assessment, the owner requested "commit please".
This local checkpoint includes the Player27 catalog and 36 recipes, bounded
eleven-preset validator and tests, historical preservation fixtures, coverage and
pilot records, and the representative-sheet feedback record. The overview and
linked plans record the completed checkpoint and remaining coverage/release work.

Before committing, the current technical files were checked against all 53
reviewed input hashes and all 33 frozen integration-evidence files. The completed
466-test run, package/install/native/removal checks and single correctness review
remain applicable; later changes concern documentation and acceptance records.
The original receipts and local artwork snapshots remain frozen. No additional
GPU work or publication is part of this checkpoint.

## Verification scope

Require 69,432 generated PNGs over the same 2,201 portrait and 4,111 world strips.
All 63,120 prior PNGs must remain exact; every new PNG must match independent
literal RGBA reconstruction from source components. Check dimensions, alpha,
metadata, source hashes, selected inventories and protected materials. Include
negative altered-target and protected-pixel checks and expressive inspection of
the lip/flush cases omitted from the five-character pilot.

The bounded validator change accepts eleven generated presets and rejects
twelve without publishing partial output. Native interpreter tests cover the
last asset, twelve-choice wrapping, independent selection and old/new settings.
The existing per-character table partition remains necessary for the pinned
compiler's constant limit. No production GML or settings schema change is planned.

Full standard checks, root/standalone pinned Nix builds, strict MOMI 0.16.4
installation, production atlas reconstruction, original/spectral/shadow
preservation and exact removal are required. Actual native menu/storage and
representative rendering checks use isolated state, checked GPU availability
and a foreground reservation. They do not imply physical-controller, Steam
Cloud or broader seasonal/action acceptance.

One complete comparison will show columns Vanilla, current Player22, Player27
and current Player35, with every native frame and applicable West mirror
reachable. It will bind actual generated PNGs and metadata and receive CPU
desktop/mobile/direct-file checks. One combined correctness review follows
normal validation. Installation, package and live results will be recorded here
when complete; automated checks do not establish that the new artwork looks right.
