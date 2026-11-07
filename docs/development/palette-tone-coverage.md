# Native-tone coverage assessment and expansion

2026-11-06. The baseline assessment at checkpoint `1f40105` identifies gaps in
the original selectable tones. The approved expansion below adds six sourced
families across all 36 characters; its new art and approximate coverage await
consolidated human acceptance. Automated checks do not establish visual coverage.

## Baseline evidence at checkpoint `1f40105`

The baseline collection selects four explicit variants per character: Debug Blue
and three donor palettes. Vanilla is supplied separately by the runtime.

| Donor family across the project | Approximate appearance |
| --- | --- |
| Adeline | Warm peach/tan |
| Hayden | Golden tan/brown |
| Ryis | Deeper warm brown |
| Seridia | Muted gray; same sampled ramp as Caldarus |

Most characters offer Hayden/Ryis/Seridia. Hayden offers Adeline/Ryis/Seridia;
Ryis offers Adeline/Hayden/Seridia; Caldarus and Seridia offer
Adeline/Hayden/Ryis. A character's original tone remains available through its
own Vanilla choice; it is not automatically a donor choice on other characters.

The [NPC catalog](../../palettes/catalog/npc-spring.json) has 36 entries and 34
distinct sampled ramps; the [creator catalog](../../palettes/catalog/player.json)
has 36 distinct main skin ramps. All 36 selected collection recipes were
inspected. All 36 neutral-portrait source PNG hashes and the creator LUT PNG hash
match the preserved `local/game/assets.zip`; the older catalog archive hash
differs, but these exact source images remain unchanged. The
[NPC study](npc-palettes.md) and [creator study](palette-presets.md) own the
original sampling and art-role limits.

Clear gaps include light/cream NPC tones such as Celine/Elsie, additional
golden/brown undertones, and the deepest creator tones. These are comparisons of
sampled colors, not a measured percentage of visually equivalent results.
Earlier Adeline-only trials contain Player 01/18/33; the current full collection
does not select those creator palettes.

The [swatch comparison](../../tmp/palette-coverage-2026-11-06/index.html) and
[audit data](../../tmp/palette-coverage-2026-11-06/audit.json) retain all source
pins, selected-recipe hashes and full swatches. No palettes, masks, runtime code
or acceptance records changed during this assessment.

## Planning consequence

The initial request authorized planning broader coverage. The
[native-tone expansion plan](../plans/native-tone-coverage.md) records scope,
representative selection, preset budget and acceptance gates. That planning
request did not authorize implementation or provide visual acceptance; the
subsequent approval below authorizes implementation. The original
content-complete alpha remains reached.

## Approved expansion and candidate selection, 2026-11-06

The user approved execution with "let's do it". The implementation adds six
native donor families, recorded in [the sourced catalog](../../palettes/catalog/native-tones.json):
Player 04, March, Wiscar, Player 16, Player 22 and Player 35. Labels identify
light peach, light golden, warm peach, warm brown, deep brown and very deep choices.
The initial four-addition budget was revised to six to retain separate light,
golden, middle and deeper families. Eight additions gave less further diagnostic
improvement than the change from four to six. These are candidate art choices;
new-art and approximate-coverage acceptance remain pending.

[The per-character coverage matrix](native-tone-coverage-matrix.json) records
2,592 nearest-reference assignments: all 72 NPC/creator catalog entries against
the actual choices available to each of the 36 NPCs. Each comparison includes
that NPC's own Vanilla reference and its actual old donor subset; it does not
pretend every old family is available on every character. The three/five-shade
catalog entries are interpolated by position in Lab for diagnostic ranking only.
No such interpolation changes the sourced four-color new donor ramps.

The local deterministic diagnostic uses D65 CIELAB deltaE76 with
light/middle/shadow/deep weights 0.35/0.35/0.20/0.10. Minimizing the worst actual
per-character reference gap gives scores about 17.0, 12.8 and 12.3 for four, six
and eight additions respectively. These scores rank choices; they are not
perceptual acceptance thresholds or a proof of optimal selection. The final
six-family set uses Wiscar's near-Adeline ramp in place of the Adeline candidate
so every character can receive the same additions without conflicting IDs.
This makes the warm-peach family available to the 32 characters that previously
lacked an Adeline donor. Overlap on the other four characters and Wiscar's
Vanilla is deliberate and recorded, rather than silently dropping an ID.

Every assignment remains a candidate for visual review. Differences in shade
contrast, saturation, lighting and secondary details remain possible even at a
low numerical distance. No coverage percentage or exhaustive color-equivalence
claim is made.

The implementation uses expanded copies of the selected sets with unchanged
profile references and four-entry historical prefixes. Ten generated variants
plus Vanilla require the bounded eight-to-ten validator ceiling change; source
masks, selection mechanics and config schema remain unchanged. The first boundary
regression rejected nine as expected before this change and passes for one,
eight, nine and ten afterward, while zero and eleven remain rejected.

## Mapping and integrated generation

The [shade mapping record](native-tone-shade-mappings.json) pins each accepted
profile and historical recipe and records all 456 source slots. The append-only
mapping reuses 394 direct shade roles, 49 unique clamped RGB offsets and ten
existing blend/scale formulas. All 1,812 historical non-cosmetic targets reproduce
exactly before any additions are emitted. Positive half-up rounding preserves
existing blends; Reina's corrected lower-mouth transition stays on the light
role, and Seridia's 25 literal glow offsets are retained. No masks change.

Nora's two lip shades and Maple's lip shade have six explicit muted donor-specific
color treatments in that record. Their protected shape/interior selections remain
unchanged; the new cosmetic colors need consolidated visual acceptance.

The four-character pilot covers 1,218 strips and 12,180 generated PNGs. All 4,872
old accepted pilot PNG hashes agree. Its complete native-frame/West-mirror
review and desktop/mobile checks pass; the parent inspected extreme-tone
portraits and Reina/Seridia enlarged comparisons before roster integration.
The full 36-character build then generates 63,120 variants on 6,312 strips,
including 37,872 new PNGs. The retained 25,248 accepted PNGs remain an exact subset.
The local build/source bindings are under `tmp/native-tones/`.

Before the generated-table correction, formatting, all-target Clippy with denied
warnings and 457 regular tests passed; 511 opt-in cases were ignored. Four runtime and four
controls interpreter cases execute separately and pass, including all ten choices
and wrapping plus the missing-final-asset refusal. The focused installer and
package cases pass; the expanded Reina/Juniper portrait check covers 235 strips,
470 native frames and ten target palettes with protected mouth/material checks
retained. Packaged installation/removal and expanded live evidence are recorded
below; no release or new-art acceptance is inferred from these CPU
checks.

## Complete review and packaged candidate

The complete [gallery](../../tmp/native-tones/review/index.html) and
[portable ZIP](../../tmp/native-tones/review.zip) expose all 6,312 strips,
18,315 native frames and 4,853 West mirrors (23,168 cases). Vanilla is the first
baseline; the six new families follow in catalog order, with previous choices
available through All choices. Frame and native-mirror selectors, season/outfit
filters and per-character pages expose complete coverage. Nora/Maple lip examples
and the diagnostic source-reference assignments are linked from the overview.

All 138,864 copied source PNG/metadata bindings, 138,946 portable entries and
306 internal links pass independent checks. All 76 CPU desktop/mobile browser
loads and navigation/filter/frame/mirror checks pass. The parent inspected the
Nora/Maple deep-tone mouth screenshots as well as the pilot views. This remains
internal spot inspection, with new artwork and approximate coverage awaiting
user acceptance. The ZIP is 567,208,034 bytes, SHA256
`428787e4d2562a93bd2f6b58f83be4d30d06929a2e407b711da52a1cf9c1695c`.
The frozen pilot packet remains unchanged.

The initial standalone text-source Nix build and root `nix build .#` both
produced `/nix/store/spcrkw1492v3nllfmwmx71h1c6rf7k17-mistria-palette-0.1.0`.
This historical package fails the native compilation gate described below and
is superseded by the corrected candidate.
All 319 installed files agree, including every recipe. The source fixture excludes
game images/archives and local state. Root builds discover 38 explicit new text
paths through intent-to-add entries; there is no staged content or new commit.
Package and artifact evidence lives under `tmp/native-tones/`; prior alpha/live
packets remain unchanged.

## Initial native-check failure and bounded correction

The first package above builds, but its full strict install fails before
publication: the generated 3,488,906-byte single `lns_palette_definitions` function
exceeds the pinned Fabricator per-function constant pool. Pinned commit
`ec52baf` uses a `u16` constant index with 65,536 distinct entries per function.
The native checker panics with **Internal Codegen Error: too many constants used**
and emits a 12,444,270-byte IR dump to stderr. Pinned MOMI reads stdout before
stderr, causing the observed pipe stall. Local pinned-source inspection confirms
both boundaries; this is not an art-validation failure.

The parent captured the blocked stderr solely to diagnose the failure. That
intervened attempt is not counted as a strict success. It returns failure, leaves
the exact original `8354ff03…` archive intact and creates no recovery directory.
The dump SHA256 is
`5a96a8de44efe89e80c3af726c7958e1f3ad0f1b16a1ecec70db136ef6eb87c1`;
`tmp/native-tones/pre-split-failure.json` records this checkpoint. Initial package
links are preserved as `pre-split-nix-package` / `pre-split-flake-package`.

The correction partitions large generated definitions by character, preserving
the same returned table, palette IDs and runtime API. Safe small tables retain
their old literal bytes. Pinned strict checking remains required; a fresh
unmodified install after the fix must pass before live verification or completion.
The existing complete art gallery is retained and must be rebound to final
installed PNG bytes, rather than regenerated or treated as installation proof.

## Generated-table correction verified

The emitter now uses a conservative constant bound: string occurrences plus the
largest literal-array index span, against the pinned 65,536-entry limit. The full
candidate's bound is 70,684. Oversized collections emit ordered per-character
literal helpers and a small dispatcher; each current character stays well below
the limit (maximum bound 4,249). Small tables keep their prior flat JSON bytes,
preserving historical package fixtures and literal-table contracts.

The new real 36-character/6,312-group/ten-variant regression fails before this
change and passes afterward, comparing every emitted row and its call order.
A separate emitted-code test compiles and evaluates all row scalars, labels,
IDs and group strings through the local Fabricator interpreter. That interpreter
is checkout `f2483a61`; it is distinct from MOMI's pinned `ec52baf`. The fresh
strict integration check below exercises the exact pinned compiler. No GML runtime API or source masks
change. Both regular generator checks and the executed emitted-code test pass.

After the correction, formatting, all-target Clippy with denied warnings and
459 regular tests pass; 512 opt-in cases are ignored by the standard run. The
corrected root Nix package is
`/nix/store/7rss4apfibv770jjy9kk302svbbyvz0l-mistria-palette-0.1.0`.
The first package above is historical failed-integration evidence. Fresh strict
installation now uses this corrected output, the same pinned MOMI/checker and
unmodified runner, with no pipe intervention.

The single combined runtime correctness review reports no concrete findings.
It reviewed the emitter partition/bound, palette/data preservation and tests
without rerunning the integration lane. The subsequent exact pinned install passes; the expanded native input checks below also pass. New visual acceptance remains
a separate gate. No additional review pass is used for individual
files or follow-up checks.


## Final packaged installation and preservation

The corrected root build and the isolated 1,137-file text-source build produce
exactly the same `7rss4apfibv770jjy9kk302svbbyvz0l` package. All 319 installed
files agree; all 318 packaged recipe files match the working source. This tests
standalone-source rebuilding in the current Nix store, rather than a fresh OS.

Fresh full installation with pinned MOMI 0.16.4, strict lints, fail-on-skip and
required compilation succeeds in **485.792 seconds**, without modifying the
runner or intervening in its pipes. The installed archive SHA256 is
`d9a6c425849d69f41b9145e8aa3c0cec90cbbc7efe9fd39586815b510b13eb2f`.
The final installed `palette_assets.gml` is 3,491,802 bytes and contains 36
literal helpers plus the ordered public dispatcher. Its SHA256 is
`131d08081c2f08d1878d940e0582ead12c9f198264a9b79d971e18f366b21d27`.
The production controls and toggle source scripts remain byte-identical.

All **63,120 generated PNG report hashes** match the frozen full art build,
including the exact subset of **25,248 previously accepted PNG hashes**. The
strict production verifier independently reconstructs every installed original
and variant native atlas frame: **201,465 frames**, with exact dimensions,
placements, alpha and visible RGBA. Transparent hidden RGB may change through
atlas trimming. Generated variant PNGs are atlas inputs; they are not installed
as loose PNG members. All 6,312 selected original PNGs and their 6,312 metadata
members remain byte-identical to the preserved archive. The independent
spectral check also passes for all 18 intentionally unchanged strips / 48 frames
across the ten current Default atlas pages.

The [final binding](../../tmp/native-tones/final-install-binding.json),
[package binding](../../tmp/native-tones/final-package-binding.json),
[installation timing](../../tmp/native-tones/final-install-timing.json) and
[spectral proof](../../tmp/native-tones/spectral-installed.json) retain the
machine-readable evidence. The final corrected installation is therefore bound
to the frozen review artwork; the earlier oversized generated script is not
used as installation evidence.


The measured archive grows from 629,768,271 to 854,868,141 bytes, with all atlas
PNG pages increasing from 11 to 75 (Default contributes ten pages). The full
build has 1,934,203,964 logical bytes; the review directory has 972,243,012 logical
bytes and its portable ZIP has 567,208,034 bytes. Small-file allocation makes
on-disk usage larger. These are measured batch costs, not a delivery-time or
runtime-memory guarantee; [resource costs](../../tmp/native-tones/resource-costs.json)
retains exact totals. No extra game-archive copy is included in the source-only
candidate.


The 30 intentionally unchanged ground-shadow strips are also checked on the
final installed archive: **149 native frames**, exact loose PNGs and native
metadata, and matching atlas alpha/visible pixels. The retained spectral harness
is copied locally with only the audited shadow selection/counts/path guard and
negative input changed; atlas reconstruction and pixel comparison stay exact.
Pristine and installed cases pass. An injected opaque pixel in the first
fully transparent shadow is rejected with `Installed pixels differ`, proving
that alpha changes cannot pass as hidden-RGB differences. The initial attempt
to mutate a visible RGB pixel on that empty shadow could not execute and is not
counted as a successful negative control. The
[shadow binding](../../tmp/native-tones/shadow-harness-binding.json),
[installed proof](../../tmp/native-tones/shadows-installed.json) and
[controls](../../tmp/native-tones/shadow-controls.json) retain the evidence.
Together with spectral verification, all 48 audited no-change strips are checked.


## Expanded native controls, restarts and rendering

All three fresh isolated hardware-rendered sessions pass on Fields of Mistria
1.0.8, RX 9070 XT/radeonsi Mesa 26.1.8, under checked foreground `gpu-run`
reservations. Inputs are actual OS keyboard/mouse and SDL virtual-gamepad
events. No palette callback or API is used to select choices. Unrelated iGPU
workloads and real game/user data remain untouched; no local inference runs.

- `native-tones-controls`: the prior version-1 all-36 Debug Blue configuration
  restores before actions without changing its bytes. All eleven Adeline choices
  and wrapping work; the other 35 choices remain stable. Long new labels wrap
  visibly. Real read-only storage failures show native warnings for cycling and
  opting out while keeping the primary file exact. Reset saves all 36 Vanilla;
  a new Player 35 selection saves independently. Normal native Exit returns 0.
- `native-tones-remembered`: a fresh session restores Player 35 for Adeline and
  Vanilla for the other 35 without a startup write. Turning remembering off
  saves false while retaining the current in-session selection. Normal title
  New Game enters the prologue and displays the new very-deep Adeline portrait
  and world actor, alongside Vanilla Eiland. The supervisor intentionally stops
  this session after evidence capture: game exit -15, supervisor success. This
  is not recorded as a normal native game exit.
- `native-tones-off`: a fresh session starts all 36 rows on Vanilla, despite
  the saved primary retaining Player 35. Native cycle changes, including the new
  Player 04 choice, leave the opted-out primary exact. Normal native Exit returns 0.

The [live proof](../../tmp/native-tones/live-checks.json) binds all 48 raw captures,
input event counts, configurations, session results and archive hashes. The
[parent-inspected Player 35 dialogue example](../../tmp/live-release/native-tones-remembered/normal-prologue-adeline-player35.png)
and [native wrapped label](../../tmp/live-release/native-tones-controls/cycle-08-player_35.png)
show representative rendered results. This is bounded technical verification;
new-art and approximate-coverage acceptance still require human review.
Physical controllers/sticks/rumble, Steam/Cloud integration, broader seasonal/
outfit/action live play, other operating systems and other-mod combinations
remain unverified by this batch.

All game/supervisor jobs finish and GPU reservations are released before removal.
Packaged uninstall succeeds; the independently hashed lab archive and preserved
source both equal
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The recovery directory is absent. The
[removal binding](../../tmp/native-tones/final-uninstall-binding.json) retains the
result. Installation scratch trees are removed by the transaction; the measured
full/pilot build and frozen gallery/ZIP artifacts, two source fixtures and initial
compiler diagnostic are retained for review/reproduction. Prior accepted
snapshots and review/release packets are untouched.

## Handoff and acceptance

Implementation and integrated technical checks are complete. New colors,
Nora/Maple cosmetic adaptations and approximate native-tone coverage await one
consolidated review of the linked full gallery. Vanilla appears first, followed
by the six new catalog families; All choices exposes the retained old variants.
No new art acceptance, commit or publication is inferred from implementation
approval or passing checks. After actual visual acceptance, freeze new accepted
outputs separately; retain every prior approval and its qualifications.

The source-only candidate, final check packet and artifact hashes are under
`tmp/native-tones/`. They exclude game images/archives and private state from the
source archive; the art-review ZIP necessarily contains generated art for the
local review. Remaining physical-controller/broader gameplay and owner licensing
release gates stay in the original finish plan.


## Celine review follow-up

After the initial complete review, the user said "not bad" and reported Celine
problems on darker tones, clarified portraits, named the Autumn/Garden `blush`
and `angry_blush` strips, then noted many expression portraits. The original
blush color `#F4AEAE` remains pale in the reviewed deep-tone outputs. The special
blush expression also shares pink colors with real mouth details, requiring
explicit cheek component selections rather than a global pink replacement.

The bounded Celine correction is now implemented and technically verified.
The [owning art record](celine-native-tone-blush.md) and
[component/target record](celine-native-tone-blush.json) retain exact selection,
color arithmetic and red/green corpus evidence. It appends isolated blush source
slots and identity tails preserving every previous four-choice PNG, adapting only
the six new families. The original 456-slot mapping evidence remains intact;
three new Celine cosmetic slots bring the selected total to 459. Initial
review/install/live/source packets remain frozen historical bindings; revised
bytes are checked separately below.


## Celine correction integration

The corrected current roster is bound from the frozen outputs for the other 35
characters and the fresh Celine build. Every actual PNG hash is checked:
63,120 current generated PNGs, with all 25,248 previous accepted hashes exact.
Only 378 PNGs change from the initial native-tone review: 63 Celine expression
strips × six additions. All 2,070 Celine world PNGs and all 1,560 Celine previous
four-choice PNGs remain byte-exact. Of the 115 initial review input pins, only
the Celine expanded recipe changes; the additional profile is separately pinned.
The [current roster binding](../../tmp/celine-tone-followup/current-roster-binding.json)
is a composite artwork proof, rather than a regenerated full-roster build.

Formatting, all-target Clippy with denied warnings, release build and 460 regular
tests pass; 514 opt-in tests are ignored in that standard run. The two dedicated
source-backed Celine cases run separately and pass, including the observed
failing-before/passing-after cheek regression and all-pixel correction boundaries.
The append authoring helper reproduces all 36 sets with the documented Celine
cosmetic overlay; the baseline shade-mapping evidence remains unchanged.

Root and standalone 1,139-file text-source Nix builds both produce
`/nix/store/lg61y3bx50f5vp0zh9sns7awpxfy6wc0-mistria-palette-0.1.0`.
Of 320 installed package files, the production binary and every old file other
than Celine's expanded recipe remain byte-identical to the prior full live-tested
package. The new blush profile is the only added package file. All 319 packaged
recipes match current source. Standalone rebuilding uses the existing Nix store;
a fresh operating system is not tested.

Focused packaged installation covers all 390 Celine strips and 3,900 variants
with the same unmodified pinned strict MOMI/checker. It succeeds in 31.938 seconds,
without pipe intervention. All generated PNG report hashes match the corrected
review build; all 390 original PNGs and metadata members remain byte-exact.
The strict verifier reconstructs **11,759 original/variant native atlas frames**
with exact alpha, visible pixels, dimensions, placements and metadata. Its archive
SHA256 is `9cd3256ede99b50b23b106a497c69405cc7a256a7602cc72ab10715b10c5e73e`.

Current full-roster IDs, labels, order and all 6,312 animation groups are unchanged;
reconstructing the split table matches the prior actual installed script SHA256
`131d08081c2f08d1878d940e0582ead12c9f198264a9b79d971e18f366b21d27`.
The focused installed Celine table is also exactly the same row as in that full
live-tested table, and runtime/controls source scripts remain exact. These are
data-only corrections: the previous full-roster strict and GPU/menu/persistence
results remain separately scoped; this follow-up does not claim a fresh full36
installation or another live playthrough. No new GPU session or inference runs.

Packaged uninstall succeeds and independent hashes of both the lab archive and
preserved source return to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`;
the recovery directory is absent. The
[package binding](../../tmp/celine-tone-followup/package-binding.json),
[runtime table binding](../../tmp/celine-tone-followup/runtime-table-binding.json),
[installation proof](../../tmp/celine-tone-followup/install-binding.json) and
[removal proof](../../tmp/celine-tone-followup/uninstall-binding.json) retain results.

The complete [Celine before/after gallery](../../tmp/celine-tone-followup/review/index.html)
and [31.7 MiB portable ZIP](../../tmp/celine-tone-followup/review.zip) expose every
portrait, both native frames and all six new choices. Rows are Vanilla, exact
previous output, then corrected output. Twelve CPU desktop/mobile page loads,
portable source/ZIP bindings and all frame/tone/filter controls pass. The parent
inspected both reported cases and special blush. The
[current review index](../../tmp/native-tones/current-review.html) combines this
Celine revision with the unchanged other characters; original packets stay frozen.
The correction and overall approximate tone coverage still need human acceptance.
No commit or publication occurs in this follow-up.


## Celine mouth, eye and visible-blush follow-up

The user called the first blush correction "much better" while reporting
remaining issues around the mouth and eyes. They identified the nose patch as
blush and asked that it remain, perhaps in another shade. The very-deep ratio
mapping made that patch too subtle; retain its visible pattern with a muted
warmer tint. Source inspection also identifies original warm contour pixels at
mouth/eyelid boundaries, including colors shared with hair and already-mapped
skin shades. Inspect explicit source components rather than recoloring all
occurrences of these colors.

A bounded offline preset-profile override is necessary to select those facial
components only for the six new choices while retaining all four previous PNGs.
The initial shared profile, historical targets and every frozen before packet
remain intact. The original blush set is preserved byte-for-byte as
`palettes/sets/celine-native-blush-trial.json`; its regression assertions continue
to test that historical treatment; both of its opt-in corpus regressions still
pass without changing their assertions. The new
[facial source record](celine-native-tone-face.md) owns the revised source
components, twenty-three-slot profile, six authored muted-rose cosmetic tables,
thirteen-landmark red/green proof and complete portrait comparison. Native
runtime/config/UI remain unchanged. The optional profile path is an offline
preset-loader change, documented in [the authoring reference](../TOOLS.md).

The candidate changes 1,098 PNGs: all 183 portraits × six new tones. Exactly
52,656 pixels differ from the last blush correction: 6,262 newly selected source
pixels plus 2,514 existing blush pixels, multiplied by six. All 1,560 old-choice
PNG outputs and all 2,070 world outputs remain byte-exact. Metadata, alpha and
every unrelated pixel are exact across 117,002,880 decoded affected-image pixels.
Protected hair/brow, lash, iris, eye-white, tongue and mouth-interior landmarks
pass. Source and cosmetic choices are authored candidates, not recovered native
formulas or final visual approval.

Formatting, all-target Clippy with denied warnings, **463 regular tests**, release
build and all three dedicated facial tests pass. The prior two blush corpus
tests pass independently. The preset-loader tests first failed on unsupported
per-preset profiles; all eight now pass. They prove unchanged shared-preset PNG
bytes while a new preset selects additional disconnected components of the same
RGB, allow different source-ramp lengths, and reject twelve malformed/incompatible
override cases without publishing output. The 36-set authoring replay reproduces
all current recipes, including the facial overlay, and all 1,812 historical
noncosmetic targets remain exact.

Root and standalone-source Nix builds produce the identical package
`/nix/store/y0cmhz3rsiih5c53fn7g1fq3qp0gsd9z-mistria-palette-0.1.0`.
All 321 packaged recipes match current source. Relative to the last blush package,
only the offline binary and current Celine set change; the new facial profile and
frozen blush trial are added. The standalone fixture pins 1,142 text inputs. This
uses the current machine/store, not a fresh operating system.

The packaged focused installation succeeds in 24.897 seconds with the unmodified
pinned strict MOMI runner, no skipped checks or pipe intervention. All 3,900 PNG
report hashes match the reviewed candidate. All 390 source PNG and metadata members
remain byte-exact. Strict reconstruction verifies **11,759 original/variant native
atlas frames** with exact alpha, visible RGBA, dimensions and metadata; hidden RGB
under zero alpha may differ after atlas trimming. Installed archive SHA256 is
`63d5db309f4569a9f33fd5462bfdaad1a4ba28e26f60efe483ca13aa6a7af6c2`.
The generated Celine runtime row and runtime/controls scripts remain exact.
Removal succeeds; both independent archive hashes return to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`
and recovery state is absent.

The complete current-roster proof verifies all **63,120 actual PNG hashes**, using
the frozen build for the other 35 characters and this fresh Celine candidate.
Every one of the 25,248 previously accepted outputs remains exact. Only the current
Celine set differs among the original full review's input pins. All current IDs,
labels, order, 6,312 animation groups and override inventories match the previous
full table; reconstructed generated GML remains SHA256
`131d08081c2f08d1878d940e0582ead12c9f198264a9b79d971e18f366b21d27`.
This is a composite PNG proof plus focused corrected-Celine installation, not a
fresh full36 installation or a new live playthrough. Prior full strict/native/GPU
evidence remains separately scoped. No GPU or local inference is used here.

The [current portrait comparison](../../tmp/celine-face-followup/review/index.html)
and [portable ZIP](../../tmp/celine-face-followup/review.zip) contain all 183 strips,
366 native frames and six additions, ordered Vanilla → Before (last blush
correction) → After (facial correction). Twelve CPU desktop/mobile page loads and
their frame/tone/filter controls pass; 2,562 copied source bindings and all 2,570
portable ZIP entries are verified. The parent inspected the reported blush and
angry-blush cases plus open-mouth special blush. The current roster review points
to this correction while both earlier galleries remain frozen.

[Integrated checks](../../tmp/celine-face-followup/integration-checks.json) retain
the standard, package, PNG/runtime, strict-install, removal and source-only
candidate bindings. The fresh build (159 MiB), review folder (60 MiB), portable ZIP
(31.7 MiB), standalone text fixture (90 MiB) and initial twenty-slot internal
candidate (159 MiB) are retained as bounded review/reproduction evidence; no new
copy of the game archive is made. The final source-only Linux candidate excludes
game images, archives and private local state. Prior packets remain intact.

The complete facial correction is technically verified and ready for one focused
visual assessment. Overall new-art and approximate coverage acceptance still
remain; no commit or publication occurs in this follow-up. Physical controllers,
broader seasonal/action gameplay, other platform/mod combinations and the owner's
licensing/publication decisions remain separate release gates.

## Roster blush and sporadic mouth follow-up

The targeted all-character expression review prompted near-universal blush
feedback and sporadic mouth defects. The complete remaining-roster audit now
corrects confirmed facial components on 31 characters, leaves four audited
characters unchanged and preserves the improved Celine result. Full standard,
regression, Nix package, actual full36 strict install, spectral/shadow preservation
and exact removal checks pass. The
[owning correction record](native-tone-roster-face-correction.md) contains exact
evidence, material decisions, retention limits and the consolidated review.
Revised facial art and overall approximate coverage still await visual acceptance;
earlier approvals and live verification limits remain intact. No commit or
publication is included.

## Adeline and Holt review correction

The user finds the displayed roster "otherwise looks pretty good", with two
remaining Adeline cheek dots and a pre-existing Holt moustache defect. Both
focused fixes now pass technical checks. The
[owning follow-up](adeline-holt-face-followup.md) records the exact component
changes, current-only Holt profile, narrow old-choice moustache exception and
complete before/after review. Earlier profiles, recipes, approvals and packets
remain historical and unchanged. These two revised appearances and overall
approximate coverage still need assessment; no new live or publication approval
is inferred.

## Holt skin/hair boundary follow-up

The user judged the preceding moustache restoration "otherwise much better",
with remaining edge pixels. The bounded
[Holt edge correction](holt-moustache-edge-integration.md) returns confirmed
surrounding skin to its palette targets while retaining restored true hair.
Technical checks pass; the original-choice exception is narrowed to 64 Holt
portrait files. Other characters, world art and all historical packets stay
exact. One focused appearance assessment remains; complete approximate coverage
and live acceptance are separate.

## Checkpoint readiness after final portrait review

The user calls the final Holt edge revision "much better" and asks whether the
batch is ready to commit. Record qualified positive feedback on the displayed
revision, with no further defect reported and inspection depth unspecified. The
[final correction record](holt-moustache-edge-integration.md) owns that assessment
and the exact frozen review binding. All reported portrait issues are closed
for this checkpoint, and the implementation remains identical to the passing
verification inputs. Broader approximate-coverage assessment and the original
release gates remain separate. The [plan](../plans/native-tone-coverage.md) now
records the user's subsequent "do it" authorization for the checkpoint commit,
including the expansion, corrections and planning updates. Publication remains
separate.
