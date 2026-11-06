# Reina skin spikes below the mouth

2026-11-05. **Correction authored, validated and accepted as displayed.**
The candidate changes only the missed outer-mouth skin shade in Reina's speaking
frames. All genuine lip/interior colors, first portrait frames and world artwork
retain their previous pixels.

## Report and classification

After the second-batch review, the user identified issues around Reina's mouth,
then clarified that every Reina example had "two, sometimes three, spikes of
uncolored skin" **below the mouth**. The [earlier audit](reina-juniper-mouth-followups.md)
had grouped the warm `#B2684A` fringe with mouth materials. That classification
was wrong; its historical review images remain frozen for comparison.

Source geometry isolates the missed skin transition independently of the mask
seeds. `#B2684A` is close to the main skin `#B36644` and existing blush-base skin
`#B36844`. Its pixels lie outside the pink lip/interior boundary, connecting back
into the cheek/chin skin. They occur only in **frame 2 of all 103 portrait
strips**, never in the first frame or any of Reina's **175 world strips**.

There are **425 pixels**, bounded by frame-local x142–147/y86–90. Their positions
form seven source shapes: 17 frames contain 3 pixels, 56 contain 4, and 30 contain
5. Small adjacent pixels make the two or three pointed shapes the user described.
The literal coordinates for every affected frame are retained in
`tmp/reina-mouth-correction/review/source-fringe-inventory.json`.

For example, Autumn neutral frame 2 has the missed skin at `[146,87]`,
`[147,87]`, and `[142,88]`. Those points now use the selected palette's light
skin color. The actual warm mouth interior `#581D1F`/`#762E21`, pink lip shades,
and tongue remain as before. Source inspection did not identify another missed
skin shade in the reported spikes.

## Candidate data and preservation

`palettes/profiles/reina-mouth-corrected.json` is a new lineage from the retained
`reina-world-trial.json`, with the same **278 assets**, source pins, dimensions,
and existing seeds. It appends the single source color `#B2684A` in its own
connected-color group, plus **338 component seeds** selecting all 425 fringe
pixels. Existing groups retain their original meaning.

`palettes/sets/reina-mouth-corrected-trial.json` preserves all four preset IDs,
labels and existing target mappings. Its appended shade uses each preset's
existing light-skin target: Debug Blue `#9DB9D4`, Hayden `#E8B271`, Ryis `#B06C57`,
and Seridia `#C1AFA5`. No new preset or runtime behavior is introduced. Vanilla
continues to use the exact source artwork.

Historical profiles, sets, portrait tests, accepted fixtures, and the earlier
audit page are preserved. The Reina portion of the current mouth-audit test now
protects six true lip/interior colors instead of classifying the fringe as mouth
material. All Juniper test code remains byte-for-byte unchanged. The separate
correction regression independently requires every fringe pixel to recolor.
Shared collection selection and planning are maintained by the batch integrator.

Candidate SHA-256 values:

| Input | SHA-256 |
| --- | --- |
| `palettes/profiles/reina-mouth-corrected.json` | `e7c2bf973cb8022a1c65e749e1b04bf2e0238fe5b46b79b55c89a90eaef148e2` |
| `palettes/sets/reina-mouth-corrected-trial.json` | `8400ee55bbc1f9713e42d7fbb6bf522673fb41714fe121a0ac3453e8478aef4e` |

## Verification

The new `tests/reina_mouth_correction.rs` first failed against the historical
set with `outer-mouth skin left warm: ...autumn_blush.png [442,87] blue`.
It then passed with the correction. It rebuilds both complete 278-region sets
and compares every pixel in every target palette, requiring exactly **425
corrected pixels per target / 1,700 total**. Every other pixel must equal the
historical output, including transparent RGB, blush, clothing and other mouth
shades. It also checks source alpha and unchanged metadata.

Two local mutations were rejected: removing a single spike component failed the
skin-fringe assertion; adding a recolor seed inside the genuine `#581D1F` mouth
interior failed the unrelated-pixel assertion. The test therefore rejects both
an incomplete fix and an overbroad mouth recolor.

Fresh focused release checks passed:

- New correction regression and updated current-mouth audit.
- Historical Reina portrait material test, unchanged.
- Clippy with warnings denied and formatting for both touched tests.
- Four exact recipe validations, **278 strips per target / 1,112 total**.
- All **556 source PNG/metadata members** match the preserved archive, and every
  source PNG matches its profile checksum.
- All **1,400 world PNG/metadata files** and all **1,112 metadata files** are
  byte-identical to their historical/source counterparts.
- **6,812 protected Reina lip/interior assertions** and the retained 144 Juniper
  material-landmark assertions pass across four target palettes.

The preserved archive remains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
`candidate-bindings.json` records old/new input hashes, source hashes, actual
before/after output hashes and verification summaries. Helpers and detailed logs
remain under `local/authoring/reina-mouth-correction/`, including
`before-fix-red.log`, `after-fix-green.log`, `negatives.log`, `final-tests.log`,
and the four `validation-*.json` reports.

## Correction review

Open `tmp/reina-mouth-correction/review/index.html`, or unpack
`tmp/reina-mouth-correction/review.zip` and open its `index.html`.

The compact summary repeats neutral, think, happy and bath examples from the
rejected review: **Vanilla source on the left, previous Debug Blue in the middle,
corrected Debug Blue on the right**. Enlargements show the skin immediately
below the lips. A second sampled summary shows all five choices, before above
after. These are clearly labeled samples.

Complete pages cover **all 103 affected speaking frames** in **16 small pages
across six outfits**, with at most eight cases per page. Each case has before
and after rows across Vanilla, Debug Blue, Hayden, Ryis and Seridia: **1,030
complete cells**, plus 64 sampled context/detail cells. The full mouth crop
`[132,68,42,26]` includes all 425 corrected points. First frames and world art
are excluded from this focused visual gallery because the differential test
requires them to remain unchanged.

Every before/after Blue case and both sampled summaries were visually inspected.
All five-choice gallery cells passed **19,086,576 exact preview pixel bindings**
to the actual frozen historical and newly generated candidate bundles.
**30 CPU Chromium page loads** checked all desktop pages and mobile outfit
indexes at 1200/390 pixels; images and links loaded without horizontal overflow.
The portable archive includes the complete gallery, per-frame source positions,
pixel checks, browser results and candidate bindings.

No local GPU, installation, game launch or runtime change was needed. Passing
these checks establishes the scope and pixel behavior of the correction; the
user's visual judgment of the corrected mouth remains the acceptance gate.
Repository-wide checks and consolidated feedback recording belong to the batch
integrator.

```sh
nix-shell --pure --run 'cargo clippy --locked --release --test portrait_mouth_followups --test reina_mouth_correction -- -D warnings'
nix-shell --pure --run 'cargo test --locked --release --test reina_mouth_correction --test portrait_mouth_followups --test reina_portraits -- --ignored --nocapture'
```

Before shared selection is updated, set `FOM_REINA_MOUTH_SET` to the absolute
path of the new candidate set for the current-mouth audit. The new correction
regression selects that candidate directly and supports `FOM_REINA_CORRECTED_SET`
for the recorded mutation checks.

## Integrated selected candidate

The parent selected `reina-mouth-corrected-trial.json` in the 36-character
collection. Asset membership remains 278 Reina regions / 5,154 total selected
strips; no registry, animation or runtime source change was required. All 555
other previously reviewed input/gallery fingerprints still agree; the only
shared selection change is Reina's preset-set path. The non-Reina displayed
second-batch artwork retains the user's acceptance, inspection depth unspecified.

Fresh integrated checks pass: formatting, all-target Clippy with warnings denied,
426 regular tests (493 ignored in that invocation), release build, and all three
selected source-backed correction/mouth/retained-portrait tests. The regression
protects 6,812 genuine Reina lip/interior instances and 144 Juniper landmarks.

A disposable Reina-only CLI installation generated all 1,112 variants for 278
source strips, passed strict MOMI compilation, and independently verified
installed atlas pixels, metadata and all three production scripts. Removal
restored the exact preserved archive hash and removed recovery state. The
previous full 36-character atlas round trip remains the pre-correction second-batch
checkpoint; this follow-up preserves all dimensions, alpha, metadata, mappings
and unrelated artwork and verifies every corrected Reina variant in the engine
archive. Physical rendering/input/gameplay were not exercised.

Parent evidence: `local/authoring/reina-mouth-integration/check-results.json`,
`tmp/reina-mouth-correction-checks.log`, `tmp/reina-mouth-correction-install.log`
and `tmp/reina-mouth-correction-uninstall.json`. The original review manifest and
collection/registry snapshot remain under the same authoring directory.
The `generated/review` and `generated/build` shortcuts now point to the focused
correction review and its complete Reina build. No commit was made.

## User acceptance

After the before/after review request, the user replied, "yeah I think that's
good enough". Record this qualified acceptance of the displayed Reina skin-spike
correction. The review included a sampled source/previous-Blue/corrected-Blue
summary and complete affected-frame pages; the user did not state inspection
depth. Do not infer exhaustive gallery inspection or live-game acceptance.
The candidate profile/set fingerprints above identify the accepted artwork.

Accepted output/profile/palette snapshots and the qualified acceptance record
are retained under `local/fixtures/finish-reina-mouth/`. Snapshotting the complete
Reina output provides a regression reference; it does not widen the displayed-art
approval to an exhaustive live or full-gallery inspection.
