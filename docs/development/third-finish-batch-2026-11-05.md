# Taliferro and Vera world-art batch

2026-11-05. **Authoring, integration and verification are complete. Displayed
Taliferro/Vera artwork is accepted.** This is a P2 batch under the approved
finish plan. After the consolidated review, the user replied "looks good to me.
commit and next batch please". This accepts the displayed artwork; inspection
depth is unspecified and no live-game acceptance is inferred.

## Scope and preservation

| Candidate | New world strips | Native frames | West mirrors | Complete cases |
| --- | ---: | ---: | ---: | ---: |
| Taliferro, all seasons | 60 | 200 | 64 | 264 |
| Vera, all seasons | 60 | 200 | 64 | 264 |
| Total | 120 | 400 | 128 | 528 |

Every native animation and its West mirror must appear in full Vanilla/Debug
Blue comparisons, with small outfit pages and enlarged details. Five-choice
samples are explicitly samples. The [Taliferro](taliferro-world.md) and
[Vera](vera-world.md) records own their material decisions, exact colors,
protected landmarks, negative controls, source timings and gallery evidence.

Prior portrait recipes and outputs are retained. The shared registry adds only
these 120 source paths; every old mapping and other registry field is unchanged.
All 554 other frozen second-batch source/gallery inputs still match. Reina's
accepted corrected profile, palette identity and output snapshots remain under
`local/fixtures/finish-reina-mouth/`; that acceptance retains its qualified
"good enough" wording and unspecified inspection depth.

## Inventory and source boundary

The collection selects 5,274 source strips for 36 characters: 2,201 portraits
and 3,073 world strips, with 21,096 nonvanilla variants. Taliferro has 96 selected
regions and Vera 92. Twenty-four characters now have complete seasonal source
coverage; this is inventory coverage, separate from art acceptance.

The frozen original omission inventory now has 365 covered strips. Its remaining
1,086 raw omissions are 1,038 authoring/review strips, thirty audited shadows and
eighteen audited spectral no-change strips. P2 has 113 remaining strips (Stillwell
68 and cameos 45), P3 has 558, and P4 has 367.

All 10,548 selected PNG/metadata members match both the preserved archive and
`tmp/fom/assets.zip`. The preserved source remains pinned to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This member agreement does not establish general update compatibility.

## Integration verification

The central profile/registry coverage test passes for the two new selections.
Fresh standard formatting, all-target Clippy with warnings denied, 430 regular
tests (495 ignored), release build, eight affected material/package/retained
cases and five GML interpreter cases pass. Both source-backed suites preserve
all 544 prior portrait PNG/metadata files. Omission and material-spill negative
controls fail at their independent landmarks. The complete galleries bind
62,797,824 preview pixels and pass 220 CPU desktop/mobile browser loads.
The full 36-character CLI install generated all 21,096 variants, passed strict
MOMI compilation, and independently verified installed atlas pixels, animation
metadata and all three production scripts. The actual installer report also
binds both new characters and the accepted Reina images to the frozen author
bundles/snapshot by PNG hash. Removal restored the exact source archive hash above and
removed recovery state. No user installation was modified.

The combined two-character batch build matches all 1,504 author PNG/metadata
files byte-for-byte. All old portrait roles remain unchanged; no new colors,
groups or preset choices were needed. Taliferro protects true red/dark eating
mouths, hair, eyebrows and outfits; Vera preserves her peach Summer collar,
earrings, food and garments beside exposed skin. Both include all eating frames,
exposed hands/feet and native mirrors.
The native source inventory is independent of authoring selections and comes
from every source strip's metadata. No runtime or installer code changed.

Reproduction inputs and evidence belong under `local/authoring/third-batch/`:
`source-inventory.json`, `characters.json`, `checks.sh`, `collection-counts.py`,
`inventory-progress.py`, `retained-input-check.json` and the review builder.
Counts are retained in `tmp/third-batch-counts.json`; inventory reconciliation is
`tmp/third-batch-inventory-progress.json`. Author records preserve their own
source-backed evidence instead of replacing accepted fixtures.

No local GPU or local model inference is used. Visible UI, physical input,
natural gameplay and release packaging remain P6 gates. Earlier native API/IO
checks retain their recorded limits. No real game installation, saves or settings
are changed; disposable lab verification is isolated from the user game.

## Completed review and limits

The [consolidated review](../../tmp/third-batch-review/index.html) and portable
`tmp/third-batch-review.zip` contain every one of the 528 native/mirror cases.
Samples show Vanilla, Debug Blue, Hayden, Ryis and Seridia in that order; full
comparisons put Vanilla left and Debug Blue right. All sources, palettes and
gallery files are fingerprinted. Following user acceptance, the unchanged
profile, original preset set and all generated variants are frozen under
`local/fixtures/finish-taliferro/` and `local/fixtures/finish-vera/`, with the
acceptance scope recorded alongside them. Reina and the other accepted artwork
keep their prior judgments.

Standard/material/GML evidence is `tmp/third-batch-checks.log`. Full installation
and exact removal are `tmp/third-batch-full-install.log` and
`tmp/third-batch-full-uninstall.json`; the restored lab retains only its original
archive, Maybe.toml and lock file. Batch/artifact helpers remain in the authoring
directory. The accepted Reina snapshot is 22 MiB; new author evidence is about
45 MiB combined, and the reusable paired build is retained for reproduction.
Historical fixtures and shared Rust caches remain intact.

The final install report matches 1,864 PNG hashes for Taliferro, Vera and accepted
Reina against the current author builds and frozen accepted Reina snapshot.
The user acceptance of Reina is preserved in the owning correction record.
Taliferro/Vera acceptance is based on the explicit response to this batch's
review, separate from the passing installation checks.

The consolidated page and both review indexes passed six additional CPU desktop/
mobile browser loads, with every image/link available and no horizontal overflow.
The ignored `generated/review` and `generated/build` shortcuts point to this
completed combined review and paired build.
