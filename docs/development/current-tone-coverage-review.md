# Current twelve-choice tone coverage review

2026-11-08. **Prepared and technically verified; owner coverage assessment is
pending.** The owner approved planning
with "sure", then approved the returned
[implementation plan](../plans/current-tone-coverage-review.md) with "Go for it".
This record owns the new compact current comparison; the
[native-tone plan](../plans/native-tone-coverage.md) owns broader scope.

Open the [current worksheet](../../tmp/current-tone-coverage-review/index.html)
or download the [self-contained ZIP](../../tmp/current-tone-coverage-review.zip).
The [new decision record](current-tone-coverage-review.json) has all 70
assessments pending. Historical packets and decisions remain frozen.

## Purpose and review boundaries

The historical [coverage worksheet](../plans/native-tone-coverage-review.md)
predates Player27. This separate review brings the current twelve choices
together across all 36 characters, using existing sampled portraits/world
strips and the retained diagnostic supplement. It preserves the historical
worksheet, pending decisions, matrices, helpers and receipts.

The worksheet compares sampled source ramps with proposed representatives.
It does not introduce a new color metric or acceptance threshold. Numerical
nearest-reference rankings guide navigation; visual assessment must consider
depth, undertone, shade contrast and the actual recolored examples. A meaningful
gap remains open until addressed or explicitly accepted as a limitation.

The [Player27 representative-sheet feedback](player27-review-sheet.md#owner-feedback)
is qualified acceptance of that displayed neutral-portrait presentation, with
inspection depth unspecified. It does not transfer to this worksheet's different
frame/crop selections or establish complete artwork/cosmetic/coverage acceptance.
All new coverage decisions start pending with empty owner feedback.

## Reading the comparison

Source swatches appear beside the selected reference swatches. In the artwork
inspector, Vanilla is the baseline on the left and the selected current palette
is on the right. The reference label and swatches must follow the actual
selection, including character-specific Vanilla alternatives.

Priority comparisons call attention to earlier approximations and the three
common proposals changed by Player27: creator Player24, Player27 and Player30
now propose Player27 instead of Player22. Every sampled source remains
accessible; special gray is identified separately and Debug Blue supplies
diagnostic context. Neutral portrait and world examples, face/full-portrait
controls and native-frame access provide context beyond source swatches.

Any linked complete Player27 gallery is a four-choice comparison, ordered
Vanilla, Player22, Player27, Player35. Earlier ten-choice galleries are historical
context. These optional galleries require the checkout; the new worksheet's
selected examples must be self-contained in its portable download.

## Verification and preservation

The [final technical receipt](../../tmp/current-tone-coverage-review-checks/final/checks.json)
records the complete data, independent decoding, CPU browser and portability
checks. The [supplemental receipt](../../tmp/current-tone-coverage-review-checks/supplemental-checks.json)
adds explicit requested-last-frame assertions. The
[parent review](../../tmp/current-tone-coverage-review-checks/parent-review.json)
records Sol's final helper/receipt/framing review and fresh artifact/preservation
verification after worker completion.

- The worksheet includes all 36 characters with their actual twelve ordered
  choices, 70 distinct ramps/72 source identities and 2,592 source-character
  assignments. It preserves the historical 72 selected strips, crops, dimensions
  and native metadata. All 864 PNGs and 72 metadata files have exact current
  character/choice/asset/report associations; all 792 prior-choice PNGs remain
  byte-identical.
- The current report is bound through artwork and integration receipts. All
  986 input pins and 68 effective-profile bindings agree. Independent source
  coordinate decoding reproduces all 72 source identities. The historical
  matrix's catalog pin resolves to the six-family fixture; current diagnostics
  pin the current catalog separately.
- The existing common-representative rule changes only Player24/27/30 from
  Player22 to Player27. There are 19 priority rows. The 164 source-identity
  alternatives appear as 159 buttons after duplicate source rows are grouped.
  No metric, threshold, coverage percentage or owner assessment was added.
- CPU Chromium at 1440×1000 and 390×844 exercises all 432 character-choice
  selections per viewport, displaying 864 portrait/world samples. Each run
  checks all 70 Inspect buttons, 159 nearest-alternative buttons, filters,
  face/full-portrait controls, gray/Vanilla resolution and Debug Blue context.
  Pillow independently supplies and rechecks 2,160 crop/frame hashes; browser
  output passes 2,304 exact premultiplied-RGBA comparisons per viewport. The
  separate last-frame harness asserts all 72 requested controls and adds 288
  canvas comparisons per viewport.
- Negative controls reject missing Player27, stale PNG hashes, incorrect choice
  and asset routing, stale displayed labels and stale displayed swatches for
  their intended reasons. The prior stale-Vanilla-swatches regression passes.
- All 946 packet/ZIP members have matching inventories and hashes; CRC checks,
  14 required HTML links and 72 optional checkout links pass. A ZIP extraction
  outside the checkout passes the full mobile interaction suite by direct file.
- All 1,360 frozen paths match the
  [baseline](../../tmp/current-tone-coverage-review-checks/baseline.json) before
  and after preparation; nine transitive legacy artifact pins also agree.
  Parent verification freshly confirms the frozen paths, all current input
  pins, all 936 source/copy media associations and every archive member.

Sol inspected the [desktop context](../../tmp/current-tone-coverage-review-checks/desktop-context.png),
[mobile context](../../tmp/current-tone-coverage-review-checks/mobile-context.png)
and [gray case](../../tmp/current-tone-coverage-review-checks/final/gray.png).
Source/reference swatches, actual labels and portrait/world pairs are legible.
Mobile content fits horizontally and scrolls vertically; the first 390×844
screen shows the introduction and initial controls, with artwork farther down.
This is assistant framing inspection, not owner art or coverage acceptance.

One checker development issue was corrected: a relative `--checks` destination
caused screenshot receipt path handling to fail. Resolving that argument to an
absolute path fixes it; the same relative invocation passes in the final run.
The [inspection/retention note](../../tmp/current-tone-coverage-review-checks/visual-inspection.json)
records this development correction and the retained captures.

The local helper copies are
`local/authoring/current-tone-coverage-review/{build.py,check.py,review.js,style.css}`.
The source manifest and final receipt pin their contributing bytes. Commands:

```sh
nix-shell -p python3Packages.pillow --run 'python3 local/authoring/current-tone-coverage-review/build.py'
nix-shell -p python3Packages.pillow --run 'python3 local/authoring/current-tone-coverage-review/check.py --checks tmp/current-tone-coverage-review-checks/final'
```

The builder refuses existing final outputs and the checker refuses an existing
receipt destination. These commands document the completed run; do not overwrite
the delivered packet or historical artifacts to repeat them.

## Delivered fingerprints and resources

| Artifact | Bytes | SHA256 |
| --- | ---: | --- |
| Portable ZIP | 8,789,587 | `ddaeabb27173ddbf9f7be55574b4a9ab475c8092222740beccdc4afec7bfdad8` |
| [Portable manifest](../../tmp/current-tone-coverage-review/portable-manifest.json) | 102,532 | `f1a3126faf22ee2d2efea83f411f85d435e3cfc388505b760d9d6ed555538471` |
| [Source manifest](../../tmp/current-tone-coverage-review/source-manifest.json) | 1,336,742 | `c655e1d37aca6c97ff5da11f9dfd0494a044f9724f279fadf9285b522c290b97` |
| Pending decision snapshot | 117,728 | `12ebbf227d4fcebd0152aae725d8149158a74332f1b60924f4b8496ecb9e57c0` |

Retain the 17,417,533-byte worksheet, ZIP, manifests and approximately 1.45 MiB
of checks/screenshots: about 26.4 MiB total. Temporary extracted copies and
approximately 1.1 MB of superseded task-created check output/bytecode were
removed after passing verification. Earlier evidence remains retained.

This is a CPU presentation/evidence task. It changes no game artwork, recipes,
profiles, runtime, dependencies or saved choices. Full generation, package
builds, installation/removal and game sessions are outside the bounded checks.
No local GPU or local model inference is needed. The sealed source-only Linux
candidate remains frozen; local game-derived review media remains outside
release contents and attachments.

## Owner assessment

When the owner asked "what do you think?", Sol inspected all 70 source/reference
swatch pairs and considered the previously inspected Holt/Adeline/Caldarus
current artwork examples. The [assistant assessment](../../tmp/current-tone-coverage-assessment/assessment.json)
recommends keeping Player27 and identifies muted light/medium approximations,
especially Stillwell and Terithia, with Player13/20 midpoint differences also
remaining. This is a limited assistant opinion, not owner acceptance or an
exhaustive rendered-art assessment. The owner replied "go for it" to the proposed
[small muted-tone pilot](muted-tone-pilot.md), authorizing that comparison only.

Pending actual owner feedback. Preserve the delivered packet's
pending decision snapshot and ZIP when recording subsequent feedback in the
new decision record. Bind exact feedback to the reviewed artifact and record
affected groups, character exceptions and inspection limits.

Physical-controller acceptance, unsampled expressions/outfits/actions, ordinary
animation reachability, licensing and publication retain their separate scope.
