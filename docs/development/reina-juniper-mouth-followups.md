# Reina and Juniper mouth follow-up audit

2026-11-05. **Historical audit before the user pinpointed Reina's lower-mouth
skin spikes.** That audit retained the existing treatment; the user rejected
Reina's result and reported no other issues. The [separate correction](reina-mouth-correction.md) is now validated and accepted as displayed.
No art data changed in the historical audit described below.

The original [parallel portrait batch](parallel-portraits.md) records acceptance
of the trial with possible mouth-color issues deferred. Neither the character,
expression nor pixel was identified then. This pass examines the currently
selected profiles rather than treating that earlier acceptance as approval of a
new correction.

## Historical finding and decision

All **235 portrait strips / 470 frames** were checked in current Vanilla/Debug
Blue mouth contacts: Reina 103 strips / 206 frames, Juniper 132 / 264. This covers
all selected portrait outfits, both animation frames, bathing, wedding and
Juniper's Beach Accident portraits. Source inspection and focused material checks
did not isolate a concrete omitted-skin or recolored-mouth-interior defect.

Reina's open mouths retain the warm `#581D1F`, `#762E21` and `#B2684A` details
identified as mouth materials in the [original authoring record](reina-portraits.md).
For example, Autumn neutral frame 2 retains `#B2684A` at frame-local `[146,87]`,
`[147,87]` and `[142,88]`. Their warm pixels can look conspicuous next to Debug
Blue. They are already present in the source, differ from the mapped skin base
`#B36644` and blush base `#B36844`, and remain attached to the animated mouth
shape. Reclassifying their appearance as skin or changing their hue would be an
art choice that this audit cannot establish from source colors alone. The
existing treatment is retained.

Juniper uses some identical colors for skin contours and lip details. Autumn
neutral frame 1 retains the lip's `#763F21` at `[152,78]`; frame 2 retains
`#592323` there. The nearby fine skin contour `#763F21` at `[157,76]` recolors in
both frames. Wild-laugh frame 1 retains the mouth contour at `[151,74]` and the
warm `#B87D4C` interior detail at `[152,80]`; frame 2 uses `[151,76]` and
`[152,79]`. The pink tongue remains original. Thus a warm isolated mouth pixel is
not evidence that the surrounding skin failed to recolor. Existing lip,
interior, tongue and fine-skin boundaries are retained.

These are source-backed reasons to preserve the present masks; they do not
prove that every mouth looks pleasing in every palette. The original subjective
concern is not declared fixed or accepted. A later specific expression or pixel
report can justify a focused correction without reopening unrelated artwork.

## Review artifact

Open `tmp/portrait-mouth-followups/review/index.html`, or unpack
`tmp/portrait-mouth-followups/review.zip` and open its `index.html`.

Columns are **Vanilla, Debug Blue, Hayden, Ryis, Seridia**. The summary contains
8 explicitly sampled cases, each with face context and a mouth enlargement. The
complete gallery contains every audited mouth frame in all five choices:
**2,350 cells**, arranged in **64 pages with at most 8 frames**, split across
13 character/outfit groups. Mouth crops include frame-local rectangle
`[132,68,42,26]`, encompassing the lip/interior shapes in the inspected source
contacts. These are focused mouth crops, not a complete portrait artwork review.
No new corrected column exists because the audit made no art change.

`coverage.json` lists every frame/cell/crop. `pixel-check.json` records
**42,039,200 exact source/current-bundle-bound preview pixel comparisons**,
including the 80 sampled face/detail cells. `candidate-bindings.json` records
source and output hashes. `browser-check.json` records **92 CPU Chromium page
loads**, all desktop pages and mobile outfit indexes; every image/link loaded
and no horizontal overflow was detected at 1200 and 390 pixels. No local GPU was
used. Every Vanilla/Blue source-frame contact and the five-choice summary were
visually inspected; all five-choice complete gallery cells were numerically
bound to their output images.

## Verification and source binding

The preserved `local/game/assets.zip` retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All **470 portrait PNG/metadata members** match the retained source corpora
byte-for-byte. Every original PNG also matches its current profile pin.

Fresh complete builds of the current selected sets passed exact recipe
validation in all four targets: Reina 278 strips and Juniper 311 strips per
target, **2,356 target strips** in total. This includes the world assets in those
sets, while the focused visual audit covers only portraits. Rebuilding the
historical standalone portrait sets independently produced **1,880 target
PNG/metadata files byte-identical** to the current selected bundles. All portrait
output pixels, alpha, transparent RGB and metadata are therefore unchanged from
the retained portrait recipes. Production profiles and sets were left unchanged.

Current selected input SHA-256 values:

| Input | SHA-256 |
| --- | --- |
| `palettes/sets/reina-world-trial.json` | `037107e408010bd5d7ccd10ef29e43c4d7b9c18d704521c102fd8f3ccb0a3c75` |
| `palettes/profiles/reina-world-trial.json` | `a7c0ba4fa490e49b15ba5c79569d22899fd471a1389ed5f452ec841db9aabe88` |
| `palettes/sets/juniper-world-finish-trial.json` | `fceb3543e485f4300f95c6d32aff039946b41c919c6473335af6e891b288c8fe` |
| `palettes/profiles/juniper-world-finish.json` | `37287466d43298255dbb7dce1496d8741583baba6d93988c586261c8611979af` |

At this audit checkpoint, the new opt-in material test read the selected character sets, filtered their
portrait regions into temporary recipes, and rebuilt all four targets. It
checked dimensions, alpha, copied metadata, exact target colors for changed
pixels, **4,032 Reina protected mouth-pixel assertions** and **144 independent
Juniper material-landmark assertions** across those targets. The landmarks
include shared-color lip preservation and adjacent fine-skin recoloring.

A local negative control added a seed at Juniper Autumn wild-laugh `[152,80]`,
recoloring the protected mouth detail. The new test rejected it with
`mouth material: ...autumn_wild_laugh.png [152,80] blue`. The unchanged selected
recipes then passed again. Both retained portrait material tests also passed.
Focused release-mode Clippy passed with warnings denied.

```sh
nix-shell --pure --run 'cargo clippy --locked --release --test portrait_mouth_followups -- -D warnings'
nix-shell --pure --run 'cargo test --locked --release --test portrait_mouth_followups --test reina_portraits --test juniper_portraits -- --ignored --nocapture'
```

Reproduction helpers and detailed logs remain under
`local/authoring/portrait-mouth-followups/`: `gallery.rs`, `check-bindings.py`,
`validate.py`, `browser.mjs`, `final-tests.log`, `mouth-spill-negative.log`, and
`validation-summary.json`. Complete current bundles are
`generated/{reina,juniper}-mouth-audit-current`. Game-derived images and local
recipes remain ignored. This pass did not install or launch the game, exercise
physical input, or establish fresh user visual acceptance. Repository-wide
checks and the combined review belong to the batch integrator.

## Subsequent user assessment

The user later identified Reina's mouth as still having issues and reported no
other issues in the second-batch review. The retained Juniper details are accepted
as displayed, with inspection depth unspecified; Reina's treatment is rejected.
The earlier source-material classification is a working hypothesis, not a reason
to override that visual feedback. A focused correction is now active; this audit's
original images and source/output bindings remain frozen.

The user further clarified that all displayed Reina examples have two or three
spikes of unrecolored skin below the mouth. This specifically identifies the
lower-mouth fringe; it is distinct from the preserved pink/red lips and interior.
The correction must cover every corresponding open-mouth frame.

The [validated correction](reina-mouth-correction.md) recolors all 425 missed
skin-spike pixels while preserving the true lip/interior colors. The historical
audit conclusion about this fringe was wrong; its frozen comparison remains
useful before/after evidence.
