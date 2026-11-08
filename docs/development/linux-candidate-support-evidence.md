# Linux candidate support evidence

2026-11-07. **Candidate documentation prepared and verified.** This record owns
the documentation conclusions and focused checks; the linked development
records retain their detailed technical results and exact owner feedback.
The [candidate notes](../releases/linux-candidate.md) remain unpublished. The
[README](../../README.md) is the canonical installation, compatibility and
recovery guide. Documentation completion does not close the usable-release gates.

## Authorization and scope

The owner first approved Astra planning with "go for it", then separately
approved the proposed documentation batch with "go for it". Sol implemented
the [plan](../plans/linux-release-documentation.md) as one sequential batch:
candidate notes, this support record, focused README/tool-guide corrections
and planning consolidation. No runtime, artwork, recipe, dependency, package,
helper or license changed. No build, game launch, GPU work, tagging or publication
was performed for this batch.

Installation and compatibility instructions stay together in the README.
A separate compatibility page would duplicate a small support section;
maintainer receipts belong here rather than in the end-user instructions.
Historical release records and frozen packets remain unchanged.

## Distribution boundary

The owner clarified on 2026-11-07 that the release must exclude "recolored,
game-derived assets", as well as original game assets. Release contents are
tooling, text recipes/masks and mod scripts. Character and strip counts describe
recipe coverage for local generation from each user's game copy. Exported
assets, generated replacements, image galleries, screenshots and clips are
excluded from release files and attachments; retained local review evidence is
preserved separately.

The [read-only distribution check](../../tmp/linux-candidate-documentation/distribution-boundary.json)
confirms the retained package contains one tool binary and 355 text palette JSON
files, all matching its receipt. The frozen source archive contains 1,832 UTF-8
text files, with no image/media/archive file extensions or detected embedded
raster signatures. Source inspection confirms its embedded resource declarations
reference text configuration, mod scripts and gallery code. Existing artifacts
were neither regenerated nor removed; final release packaging must preserve
this boundary.

## Candidate identity and provenance

The documentation baseline is commit `57e85f3`. The current production candidate
is the Player27 package:

`/nix/store/il1nhcfkdfsvlg2dp9d7a8acxxnhl151-mistria-palette-0.1.0`

The [package receipt](../../tmp/player27-rollout/package-binding.json) records
identical root/standalone package outputs and 355 palette JSON files.
The [new input binding](../../tmp/linux-candidate-documentation/input-binding.json)
freshly verifies all 356 retained package files, all 355 source recipe files,
1,182 inputs from the latest live baseline, 31 historical receipts and eight
seed/config files. All hashes agree. These are preservation checks, not new
runtime or gameplay tests.

The tested preserved `assets.zip` SHA256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Live runs paired that archive with the supplied Linux executable. The supplied
launch archive itself differs; the
[latest live record](remaining-live-verification-2026-11-07.md#setup-failures-and-controls)
owns the corrected baseline comparison. The evidence does not establish a public
game-version number or version range.

The frozen [current source candidate](../../tmp/player27-rollout/lightning-and-sun-linux-candidate.tar.gz)
has SHA256
`e7d172e0980c98febeec5c3fb86251d750dec8b674a4ab3f6e8e31605491a9f7`.
Its [summary](../../tmp/player27-rollout/source-candidate-summary.json) records
1,832 text entries, 155,728,258 uncompressed bytes and 8,790,218 archive bytes,
with no game images, archives or local state. Its source snapshot predates later
verification-document updates. The checksum is freshly confirmed; the archive
and its manifest were not rewritten. These notes are companion documentation,
not content already inside that archive or proof that it snapshots current HEAD.
Refreshing a distributable remains separate package-finalization work.

## Support claims and owning evidence

| Claim | Evidence and qualification |
| --- | --- |
| x86_64 Linux through pinned Nix | [Flake](../../flake.nix), [package metadata](../../nix/package.nix) and [installer platform guard](../../src/installer.rs). An aarch64 developer shell is not an ARM installation package. The [Player27 package evidence](player27-rollout.md#package-and-native-verification) uses the same machine/store, not a fresh OS. |
| Packaged MOMI 0.16.4 | The [MOMI recipe](../../nix/momi.nix) supplies its binary, loader, libraries and Bubblewrap runner. The host must permit that sandbox. There is no distribution-wide or portable non-Nix verification claim. |
| Reviewed sources and installed output | [Palette validation](../../src/palette.rs), [installer](../../src/installer.rs) and [installed-output verifier](../../src/installed.rs) enforce source PNG pins, inventory/mask/dimension rules and installed pixels/metadata/scripts. The tested whole-archive fingerprint identifies provenance; it is not an installation compatibility allowlist. Whole-archive hashes also protect publication and recovery. Passing source checks on another archive does not establish support for that build. |
| Twelve choices over the complete roster | The [selected collection](../../palettes/sets/characters-trial.json), [Player27 data checks](../../tmp/player27-rollout/data-checks.json) and [rollout](player27-rollout.md) cover 36 characters, 2,201 portrait strips, 4,111 recolorable world strips and 69,432 generated non-Vanilla variants. Thirty shadows and eighteen spectral strips are intentionally unchanged. Existing choices, Vanilla defaults and opt-in remembering remain. |
| Image preservation and strict installation | The [Player27 artwork proof](../../tmp/player27-rollout/artwork-checks.json), [installation binding](../../tmp/player27-rollout/install-binding.json) and [latest live record](remaining-live-verification-2026-11-07.md#candidate-and-isolation) verify current generated bindings, original assets and installed native frames. Earlier artwork acceptance retains its exact scope. |
| Bounded existing-mod handling | The [2026-11-06 CPU record](linux-release-candidate-2026-11-06.md#installer-regressions-and-probes) and [probe receipt](../../tmp/release-checks/packaged-install-checks.json) exercise independent folder mods, precedence/refusal paths, source mismatch and exact restoration. This is dated five-choice checkpoint evidence, not a fresh twelve-choice mod-combination run or compatibility test with arbitrary named mods. |
| Explicit source hash override | The [override contract](source-hash-override.md) and current validator show that `--allow-source-hash-mismatch` relaxes only PNG hash equality. Other checks remain. A valid mask can still select an unintended material in changed art; passing checks establishes neither visual acceptance nor a new supported game build. |
| Controls, remembering and basic play | The [basic live record](live-release-verification-2026-11-06.md), [current native checks](../../tmp/player27-rollout/live-checks.json) and [rollout](player27-rollout.md#package-and-native-verification) cover title/journal keyboard/mouse and SDL virtual-gamepad events, current choice wrapping, persistence/failures and basic movement/dialogue/save/load. Physical devices, Steam/Cloud integration and other hardware configurations retain their limits. |
| Representative seasonal/action/scene rendering | The [seven-case matrix](remaining-live-verification-2026-11-07.md#results-and-limits) distinguishes normal play after recorded preparation from built-in debug activity/scene access. It does not establish ordinary story progression. The Priestess case is intentionally unchanged spectral art, not recolorable Flashback Priestess. |
| Qualified displayed acceptance | Exact feedback and inspection limits belong to the [basic live record](live-release-verification-2026-11-06.md#qualified-displayed-live-acceptance-2026-11-06), [Player27 portrait sheet](player27-review-sheet.md#owner-feedback) and [seven-case record](remaining-live-verification-2026-11-07.md#owner-feedback). Their displayed samples are accepted with qualifications; exhaustive artwork, approximate coverage and omitted behavior are not inferred. |

The new input binding compares six current enforcement/packaging files against
the preserved CPU source candidate: `src/installer.rs`, `src/installed.rs`,
`src/palette.rs`, `nix/package.nix`, `nix/momi.nix` and `flake.nix`. All are
byte-identical. This supports carrying forward the historical probe conclusions
within their original scope; it does not change their date or tested palette set.

The recorded native configuration includes an RX 9070 XT and radeonsi/Mesa
26.1.8. These identify the tested setup, not minimum hardware requirements.
Historical approximate disk-space figures in the tool guide are not measured
requirements for the current complete collection.

## Practical compatibility and recovery conclusions

Existing MOMI handling requires complete unpacked source mods, the matching
pristine `assets.bak.zip` and MOMI's installed-mod list for that archive. Each
source mod has its own manifest directly inside its folder; that manifest is
separate from MOMI's list in the game configuration directory. The list has no
archive checksum, so matching names/versions cannot prove that it belongs to
the archive. Preserve its original ordering; recover a known setup through MOMI
if the correct list is unavailable. See [tool-guide details](../TOOLS.md#cli-installation).

Unsupported layouts, missing/skipped selections and unpreservable precedence
are refused. Conflicting sprite/script replacements remain outside a blanket
compatibility claim. Remove the palette installation before changing recipes,
updating the game or managing mods separately. Preserve recovery files after
refusal; selecting Vanilla changes appearance without uninstalling the package.

The README's single-character example now selects `caldarus-native-tones.json`,
the same twelve-choice set used by the complete collection. The older
`caldarus-world-trial.json` remains a five-choice historical set; no recipe changed.
The tool guide retains historical study scopes and points current users to the
README and candidate notes.

## Focused documentation validation

The [validation receipt](../../tmp/linux-candidate-documentation/checks.json)
records local Markdown file/anchor checks, shell-example syntax, selected recipe
paths, palette labels/counts, final preservation checks and the document allowlist.
Shell syntax checks do not execute installation examples. Source/package binding
and final diff checks establish that this batch changes documentation only.

The retained [standard-check receipt](../../tmp/player27-rollout/standard-check-summary.json)
records the earlier formatting, Clippy, 466 regular tests and release build.
Those results are reused within their verified input scope; they were not rerun
or described as new results here. No full Rust/Nix suite or helper that writes
historical receipts was needed for documentation edits.

## Remaining release gates

Physical-controller acceptance, the distinct complex-animation end-stage live
criterion, broader approximate-tone assessment, unsampled artwork/forms/actions
and retained visual qualifications remain open. The
[CPU route investigation](animation-end-route-investigation-2026-11-07.md)
found no suitable bounded native caller. The subsequently approved
[first instrumented attempt](animation-end-live-verification-2026-11-07.md) was
incomplete. Its approved [prepared retry](animation-end-prepared-retry-2026-11-07.md)
passes paired native stop rendering, with owner acceptance of the displayed
comparison and inspection depth unspecified. Ordinary gameplay reachability and
the original live criterion remain separate. The owner's
licensing choice, final distribution decisions and
publication authorization remain separate. Documentation preparation neither
declares the usable Linux milestone complete nor closes those gates.
