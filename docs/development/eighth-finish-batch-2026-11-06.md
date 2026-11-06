# Caldarus and Seridia world-art batch

2026-11-06. **Authoring, integration and verification are complete; displayed
Caldarus/Seridia artwork is accepted.** The
user accepted Terithia artwork and requested a commit and the next batch after
checkpoint `9ef0e08`. Two independent authors own character candidates; the
parent owns shared registry, selection/counts, integrated checks and one
consolidated review after the whole batch is complete.

## Source scope and preservation

| Candidate | World strips | Native frames | West mirrors | Complete cases |
| --- | ---: | ---: | ---: | ---: |
| Caldarus | 179 | 605 | 204 | 809 |
| Seridia recolorable forms | 188 | 603 | 168 | 771 |
| Total authored scope | 367 | 1,208 | 372 | 1,580 |

The parent independently enumerated every original source member, checked native
metadata and PNG dimensions, and matched hashes to the frozen omission inventory.
All 367 selected source atlases are `Default`; earlier registry mappings and
other character fields remain unchanged. Source evidence and the original
registry are under `local/authoring/eighth-batch/`.

Caldarus covers all four seasons, Beach and Wedding. Seridia covers those
outfits plus Flashback Priestess. The eighteen spectral Priestess world strips
(48 native frames) remain outside the selected profile and registry, with their
existing no-change disposition and exact source hashes retained separately.
Installed original atlas pixels and metadata for these assets require a separate
check, alongside the selected variants. No source asset is silently dropped.

All 12,624 selected PNG/metadata members and 36 separately audited spectral
members are byte-identical between the preserved archive and supplied `tmp/fom`
archive. Per-member hashes are retained in `provided-source-agreement.json` in
the authoring directory. This agreement does not establish general update or
live-game compatibility.

The preserved source archive remains pinned to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The exactly restored 601 MiB disposable install lab is reused without another
archive copy. Only needed character sources are exported.

The parent froze 3,390 prior recipe/gallery fingerprints, excluding shared
registry, collection selection and central coverage files intentionally extended
and separately pinned by this batch. All 23,780 prior generated variants and
accepted snapshots, including Terithia, must remain unchanged. Earlier
first/fourth-batch approval limits remain in force. New art acceptance and
live-game acceptance remain separate from source coverage and automated checks.

## Bounded authoring-color extension

Seridia's meditation frames tint exposed skin with multiple purple glow shades.
Retaining all twenty prior source roles while covering those frames requires
50 source colors. The existing 32-color authoring/profile validation blocked that
candidate. A source trace found maps, arrays and numeric component indices rather
than 32-bit palette masks; the parent raises both validation bounds to 64 without
new palette choices or profile machinery. Boundary tests first reproduced the
33/64-color rejection, then exercise actual shades through index 63, shuffled
mapping order and excluded matching components. Empty and 65-color inputs must
still fail before outputs are published. The focused before/after logs stay in
`tmp/eighth-color-limit-before.log` and `tmp/eighth-color-limit-after.log`.

All three boundary/material tests pass after the change. A fourth CPU Chromium
case checks the actual shade-63 preview, selects component 64, saves/loads progress,
exports the 64-color profile and consumes that export through Rust; the selected
high-index duplicate changes while an excluded component remains original.
`tmp/eighth-color-limit-browser.log` records that pass. No editor algorithm,
profile format or game runtime code changed.

## Character evidence

The [Caldarus record](caldarus-world.md) owns all seasonal/Beach/Wedding world
forms, skin beside body markings, source colors shared with horns/tail, literal
material landmarks and two negative controls. All 98 portraits, including three
deliberate dragon/statue no-ops, and 784 prior PNG/metadata outputs remain exact.
The masks select 31,355 skin pixels per target and preserve 17,382 shared-color
horn/tail pixels. One Beach calf shade uses the existing medium target.

His 809-case gallery binds 216,954,280 preview pixels and passes 310 CPU browser
loads. The author inspected all unique native/mirror pairs and five-choice
samples. Focused package/material/retained tests and all four exact validations
pass. The parent reviewed the final candidate and verified all 6,597 frozen
input/output fingerprints. This is separate from user art acceptance.

The [Seridia record](seridia-world.md) owns all human and Flashback Priestess
forms, glow-shaded skin and protected horns/tail, markings, nails, makeup,
garments and equipment. Her final 50 colors retain twenty prior colors and add
thirty necessary world colors, including 25 meditation glow shades. Five unused
horn-only colors were removed from an earlier candidate without changing any
generated output. Glow targets retain their source RGB difference from the
ordinary skin role, with channel clamping. No palette choice is added.

Her masks select 32,254 skin pixels per target and preserve 9,917 matching-color
world material pixels. Fifteen fully occluded rear-view strips deliberately have
empty masks, covering 33 native occurrences. All 164 prior portraits and 1,312
PNG/metadata outputs remain exact. The eighteen spectral strips have zero overlap
with the final human source colors and remain outside the profile and registry.

Her 771-case gallery binds 163,304,064 preview pixels and passes 302 CPU browser
loads. All unique native/mirror full and enlarged pairs and seven outfit samples
were inspected. Four focused cases, four exact validations and independent
forehead-omission/horn-spill negative controls pass. The parent reviewed the final
candidate and verified all 7,512 frozen input/output fingerprints. Across both
authors, 2,096 prior portrait PNG/metadata files remain exact, 380,258,344 preview
pixels are bound and 612 CPU browser loads pass. These checks establish no user
art or live-game acceptance.

Caldarus and Seridia retain their existing five choices: Vanilla, Debug Blue,
Adeline, Hayden and Ryis. Vanilla already supplies their native Seridia ramp.
Their sample order therefore differs from earlier characters without changing
any prior choice or target mapping.

The selected collection contains 6,312 strips for 36 characters: all 2,201
portraits and 4,111 world strips, yielding 25,248 nonvanilla variants. All 36
recolorable world inventories are complete. Of the 1,451 baseline omissions,
1,403 are covered and 48 retain evidenced no-change dispositions: thirty shadows
and eighteen spectral strips. No planned authoring gap remains; this does not
close the recorded art-acceptance or release gates.

## Combined verification

Each author inspects every outfit/form, unique native/West full and enlarged
Vanilla/Debug Blue case, and a five-choice sample. Earlier portrait regions,
intentional no-change forms, palette roles and mappings remain unchanged.
Independent full-pixel/material checks and intentional omission/spill controls
supplement exact validation of all four generated presets. The parent reviews
the final candidates. Fresh formatting, all-target Clippy with warnings denied,
451 regular tests (508 ignored), release build, twelve affected material,
package, retained-portrait and color-boundary cases, and five GML interpreter
cases pass. The affected cases include the real 64-color CPU browser round trip.
All fourteen changed recipe/config/test/tooling inputs are pinned across these
checks. The integrated 629-source build matches all 5,032 author PNG/metadata
files exactly; all 3,390 prior recipe/gallery fingerprints remain unchanged.
Central coverage changes only Caldarus's count from 98 to 277 and Seridia's from
164 to 352, with both candidates independently pinned.

Standard/material/GML results are in `tmp/eighth-batch-checks.log`; collection
counts and baseline reconciliation are `tmp/eighth-batch-counts.json` and
`tmp/eighth-batch-inventory-progress.json`. Source inventory, input pins, combined
bundle bindings, reproduction helpers and the prior actual install-report hash
are under `local/authoring/eighth-batch/`.

Full 36-character CLI installation generates all 25,248 variants and passes
strict MOMI compilation plus installed atlas-pixel, native metadata and all
three production-script checks. The actual install report preserves all 23,780
prior PNG hashes, binds 2,516 Caldarus/Seridia PNGs to the authors' bundles and
5,456 PNGs to ten accepted snapshots, including Terithia. The small comparison
record is `local/authoring/eighth-batch/full-install-summary.json`.

The separate spectral check ran against that same installed archive before
removal. All eighteen original loose PNGs remain byte-identical, all parsed
native TOML fields remain identical, and all 48 visible native atlas frames
match their original pixels and alpha. RGB differences are permitted only when
both pixels are transparent, following the production verifier. A local harness
copies the exact production atlas lookup/frame-check helpers without changing
their visibility or the shipped verifier. Source/helper fingerprints and explicit
Cargo dependency bindings are retained. Its pristine positive control passes;
an in-memory visible-pixel mutation fails as intended. Results are
`spectral-installed-check.json`, `spectral-pristine-check.json`,
`spectral-negative-check.json` and `spectral-harness-source-binding.json` in the
authoring directory. No spectral asset was added to the recolor profile/registry.

Removal restores the exact original archive hash, independently checked against
both the preserved source and restored lab, and removes recovery state. The
persistent installer lock file remains as expected. Install/remove logs are
`tmp/eighth-batch-full-install.log` and `tmp/eighth-batch-full-uninstall.json`.
No real installation, saves or settings were modified. The existing 601 MiB lab
is restored; no additional retained archive copy was made.

## Consolidated review

The [review](../../tmp/eighth-batch-review/index.html) and portable
`tmp/eighth-batch-review.zip` cover all 1,580 native/mirror cases in small outfit
pages, with a separate original-source spectral audit. Samples show Vanilla,
Debug Blue, Adeline, Hayden and Ryis. Complete comparisons put Vanilla left and
Debug Blue right, with full sprites and enlarged anatomical details.

The validation page is bound to this owning record and checked by its title.
Candidate fingerprints bind final inputs and galleries; the portable manifest
binds every packaged file. Small installed-output, bundle and spectral summaries
are included. Author inspection and automated checks establish no visual
acceptance; the user's subsequent assessment is recorded below.

The consolidated page and both character indexes pass six additional CPU
desktop/mobile browser loads, with all images and links present and no horizontal
overflow. `generated/review` and `generated/build` point to this review and its
integrated build. Historical reviews and accepted fixtures remain separate.

Character records own source/inspection retention and scoped removal of
superseded helpers. The combined build retains about 157 MiB, the complete review
about 33 MiB, its portable ZIP about 9.6 MiB and the parent source/verification
helpers about 14 MiB. These retain reproducible final candidates and actual
installed spectral evidence without another retained game-archive copy.

## User acceptance — 2026-11-06

After the consolidated review, the user said "looks good. commit and then next
batch please". This accepts the displayed Caldarus and Seridia world artwork.
Inspection depth is unspecified; no live-game acceptance is inferred. Earlier
first/fourth-batch approval limits remain unchanged.

Accepted snapshots are frozen under `local/fixtures/finish-caldarus/` and
`local/fixtures/finish-seridia/`, with profile/set copies, all current variants,
per-file hashes and acceptance records bound to the reviewed ZIP. Keeping prior
portraits for regression does not broaden displayed-world-art acceptance.
Original review/check metadata retain their pre-acceptance status as historical
evidence; this section and snapshots record the later assessment. Tested input
and review fingerprints still match; only acceptance documentation changed after
the combined checks.

CPU authoring, tests and browser checks require no local GPU or local inference.
No further character-authoring batch is planned. The next batch covers CPU
Linux release checks and packaging. Earlier art-acceptance gaps, visible
menu/physical input and natural gameplay remain gates; neither finish milestone
is established by this batch alone.
