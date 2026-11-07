# Roster portrait blush and mouth correction

2026-11-06. The complete authorized correction batch is implemented and technically
verified. Revised artwork and overall approximate tone coverage await visual
acceptance. No commit or publication is included.

The user found the latest Celine facial revision "much better", then requested
expressive portraits for every character. On that sheet they reported unscaled
blush and mouth discoloration, refining "all" to "many" and clarifying that blush
is near-universal while mouth issues are sporadic. The audit therefore covers the
other 35 characters across every selected portrait expression, outfit and native
frame, with changes confined to confirmed material components in the six added
tone choices. Celine's latest 23-slot result stays exact. Earlier art approvals
and their qualifications remain valid; this feedback does not accept all new art.

## Material decisions and scope

Thirty-one characters receive source-bound facial overrides. Louis, Vera,
Wheedle and Wynne were audited and need no changes. Blush keeps its original shape
and remains visibly muted rose, with distinct source highlight and shadow roles
where needed. This follows the improved Celine treatment.

Confirmed mouth or adjacent facial details are corrected in March, Valen, Dell,
Nora, Darcy and Juniper; Adeline also has tiny facial skin highlights. Seridia's
Flashback Priestess outer lips receive scaled cosmetic colors. The
[group A record](native-tone-face-group-a.md) and
[group B record](native-tone-face-group-b.md) own character-level decisions and
link the 35 source-bound component/landmark records. No universal mouth defect is
assumed. Actual tongue, teeth, mouth interiors, intentional lipstick, eyes, hair,
scars and special materials outside the reviewed components remain exact.

Only the six added presets select the new profiles, using the existing offline
per-preset override. Shared profiles and the original four preset objects stay
unchanged. Every inherited source slot, group, region binding, source hash and
component seed is retained; additions extend the existing selections. Every
existing skin-role target stays exact, including Dell's existing nose color.
Seridia uses 53 of the supported 64 source slots. Her same-color flower, tongue,
normal-form lipstick, Dragon/spectral forms and permanent white markings remain
protected. Louis's eye band and Linnet's scar remain as previously discussed.

No Rust production code, GML, menu, persistence or configuration behavior changes
in this batch. Native-frame layout and runtime selection inputs stay unchanged.

## Regression and preservation evidence

The [independent regression packet](../../tmp/native-tone-roster-face/regressions/checks.json)
records observed failure on the real retained before outputs: all 264 correction
assertions fail for the expected old colors while all 564 protected assertions
pass. Both current opt-in tests then pass in 143.51 seconds. They independently
reconstruct the allowed source components and compare the complete 35-character
corpus, rather than checking only the displayed samples.

- 5,922 strips, 59,220 actual PNGs and 59,220 metadata files checked.
- 5,478 new-tone portrait PNGs change across 913 source strips, affecting 174,945
  facial pixels. All other decoded pixels, alpha and metadata stay exact; the
  affected portrait proof examines 583,735,680 decoded pixels.
- All 23,688 older-choice PNGs and 23,424 new-tone world PNGs in that corpus are
  byte-exact. The full-roster binding separately verifies all 63,120 current PNG
  hashes, all 25,248 previously accepted PNGs, all 41,110 world PNGs and all 3,900
  latest Celine PNGs.

Formatting, all-target Clippy with denied warnings, 464 regular tests and release
build pass; 518 fixture-dependent tests remain ignored by default. The two new
opt-in tests pass separately. The historical Reina/Juniper mouth regression also
passes over 235 portrait strips, 470 frames and ten choices, with its prior pixel
assertions retained. It now reads byte-exact frozen pre-correction trial recipes:
its all-ten-choice lip-preservation contract belongs to that earlier treatment.
The current regression owns the newly reviewed cosmetic behavior. No valid pixel
assertion was removed to accept changed artwork.

The 36-set authoring replay reproduces all current recipes and preserves all
1,812 historical noncosmetic targets. A regular source-only fixture test also
pins all inherited new-six target prefixes, including Celine's latest targets.
The [roster binding](../../tmp/native-tone-roster-face/roster-binding.json)
owns exact output counts and candidate paths; its composite binding SHA256 is
`3db386d4169cbb9d9fb0af7bc81a1cdf86f5002b95b49b76eb3d76c00a7c81c5`.

## Package, full installation and removal

Root and standalone-source Nix builds produce the identical package
`/nix/store/kz5r67qf59cdq8rnwdjbwfcxl37zja2a-mistria-palette-0.1.0`.
All 354 packaged recipes match current source; the package contains 355 files.
Compared with the previous Celine package, only the 31 corrected current sets
change; 31 facial profiles and two frozen historical trial recipes are added.
The production binary remains byte-identical. The standalone fixture binds 1,177
text inputs. This uses the current machine/store, not a fresh operating system.

A fresh full36 installation from that package passes with the unmodified pinned
strict MOMI runner, no skipped checks or pipe intervention, in 434.271 seconds.
All 63,120 generated PNG report hashes match the reviewed candidates. All 6,312
original PNGs and their metadata remain byte-exact. Production strict atlas
reconstruction verifies 201,465 original/variant native frames, including exact
alpha, visible RGBA, dimensions, placements and metadata. Hidden RGB under zero
alpha may differ after atlas trimming. The installed archive SHA256 is
`912f325c72be1fd0d8976001300c7d1f50a3fb16eaf15149a315551730b461a0`.

The runtime and controls scripts are byte-exact. The installed 3,491,802-byte
runtime table retains 36 helpers and SHA256
`131d08081c2f08d1878d940e0582ead12c9f198264a9b79d971e18f366b21d27`,
matching the previous actual live-tested table. Independent source-bound
production-helper checks verify all 18 spectral strips / 48 native frames and
30 shadow strips / 149 frames unchanged in the actual installed atlas, loose PNGs
and parsed native metadata. Removal restores both lab and preserved source archive
hashes to `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`;
recovery state is absent. The
[integration packet](../../tmp/native-tone-roster-face/integration-checks.json)
binds the install, independent no-change checks, removal and source-only candidate.

All authoring, builds, checks and Chromium review loads use CPU. No new GPU/game
or local inference session is used; this batch makes no fresh live-game claim.
Prior live rendering/input evidence and its limits remain separately scoped.

## Consolidated review

The [all-character review](../../tmp/native-tone-roster-face/review/index.html)
is ordered **Vanilla → Before → After**. Before is the exact preceding candidate;
After is this correction. It defaults to Very deep (Player 35), native frame 2.
Every character links to all selected expressions/outfits, both frames and all
six additions. Full portraits accompany enlarged face details; clicking a full
portrait moves the detail crop. Seridia's alternate forms use explicit crops.

The complete gallery covers 2,201 strips, 4,402 native portrait frames and 57,226
comparison frame cases. Checks verify 23,086 copied PNG/metadata source bindings,
144 input pins, 23,182 portable entries and 240 internal links. All 74 CPU desktop
and phone page loads pass their tone/frame/filter/crop controls. The parent
inspected the default before/after sheets for all 36 characters plus focused
mouth comparisons; the authors inspected all six corrected treatments and unique
mouth patterns. These inspections support the proposed correction and do not
supply the user's visual acceptance.

The [compact portable expression review](../../tmp/native-tone-roster-face/expression-review.zip)
contains the same 36 sampled expressions, both frames, all six tones and twelve
static Very deep contact sheets: 936 comparison cases, 474 source bindings and
496 portable entries. Its desktop/phone checks and 59 internal links pass. It is
8,948,579 bytes (8.5 MiB), SHA256
`363453624c3e841e0e1a21572dcae79c5db5496f01ffa29ba2e63efff7b44b5f`.
The [complete portable portrait gallery](../../tmp/native-tone-roster-face/review.zip)
is 352,767,554 bytes (336 MiB), SHA256
`022895c58728e16fb1a3f72599688335302b25f855bfc82f3853033c0498d38d`.
Both ZIPs are self-contained. The compact sample does not replace full portrait,
world or live verification. Earlier expression, Celine and full-world packets
remain frozen; the mutable current-review index points here.

## Retention and next acceptance

Retain `generated/native-tone-roster-face/` (2.4 GiB) and
`tmp/native-tone-roster-face/` (1.7 GiB before the final small source/summary
artifacts) as the bounded build, audit, browser and review evidence. The latter
includes the 336 MiB complete ZIP, extracted gallery, standalone text fixture and
superseded Seridia blush-only candidate (about 230 MiB). That intermediate is
evidence, not an integration input. The existing owned install lab is reused and
restored; no extra game archive is retained for this batch. The source-only Linux
candidate excludes game images, archives, private local state and credentials.

One consolidated visual assessment is due for blush and confirmed mouth edges.
Recommend this corrected treatment based on source-component review and exact
preservation evidence, while leaving the aesthetic judgment to the user.
Overall six-tone artwork and approximate coverage acceptance also remain open.
Freeze newly accepted outputs separately only after actual acceptance. No commit
is requested. Physical controllers, broader seasonal/action gameplay, other
platform/mod combinations and the owner's licensing/publication decision remain
separate release gates.

## Subsequent displayed review

The user judges the displayed result "otherwise looks pretty good", except for
two remaining Adeline blush dots and Holt's moustache, explicitly identified as
pre-existing. Preserve that qualified assessment and unspecified inspection
depth. The [focused follow-up](adeline-holt-face-followup.md) owns both fixes and
the bounded moustache exception to old-four preservation. All counts and exact
old-four claims above describe this frozen first-roster candidate; its review
and verification packets remain unchanged. No complete tone-coverage or new live
acceptance is inferred.
