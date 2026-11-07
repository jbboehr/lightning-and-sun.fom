# Holt moustache skin-edge correction

2026-11-06. After the [moustache restoration](adeline-holt-face-followup-holt.md),
the user reported remaining light pixels around the moustache and otherwise
found it "much better". This bounded follow-up corrects those skin edges while
preserving the restored copper hair. Implementation and source-bound offline
checks pass. The new appearance still requires user visual assessment; the
qualified feedback does not establish full artwork acceptance.

## Material distinction and correction

The preceding restoration protected too much. It correctly restored the missing
central moustache in speaking frames, but it also restored **320 source-colored
skin pixels** around the hair:

- **192 `#E5AF7E` fringe pixels**, in 176 components, lie outside the copper-hair
  silhouette. This color is Holt's intermediate skin shade, including the
  isolated transition pixels around the moustache boundary. Retaining their
  source skin color made them conspicuously light on dark skin choices.
- **128 isolated shading pixels below the outline**, in 128 components, use
  `#CD9067` (80 pixels) and `#B45732` (48 pixels). The same colors occur inside
  real hair. Their coordinates and expression-dependent motion identify these
  components as skin. The preserved historical `tests/holt_portraits.rs`
  explicitly identifies and asserts these exact below-moustache skin points.

The corrected current base and six-tone override reinstate only these **304
original skin component seeds**, in the historical seed order. Existing source
color slots, groups, targets, recipes, region bindings, blush additions and
historical profiles remain exact. No general rectangle or palette-wide color
replacement is used. All prior follow-up packets, snapshots and galleries are
left unchanged.

The true restored hair is **64 components / 576 source pixels** across sixteen
speaking portrait strips (embarrassed/happy/neutral/wink in four seasons).
These four components per strip include the 27-pixel main `#CD9067` patch and
three `#B45732` components. They remain source-colored and connected to the
surrounding preserved moustache. The new current profiles equal the historical
profiles with only those 64 hair component seeds removed. This keeps the actual
fix while returning the skin fringes to their intended recoloring.

## Machine-readable contract

The current [Holt record](native-tone-face-corrections/holt.json) separates:

- `regions`: the existing new-tone blush correction, unchanged;
- `restored_components`: only the 64 true hair components, source-bound by asset,
  PNG hash, size, seed, color and pixel count;
- `edge_skin_regions`: all 304 reinstated components, with the same bindings,
  `moustache_edge_skin` role, material reason and existing target slot;
- `edge_skin_landmarks`: all 304 literal points and expected colors for all ten
  choices;
- `edge_protected_hair_landmarks`: sixteen literal source-colored hair points,
  covering every Spring expression and both frames;
- `protected_landmarks`: the surviving true-hair component seeds and the
  existing unrelated mouth/eye/outline protections.

Examples on Spring neutral distinguish the same source RGB by material:
`[148,67] #CD9067` is below-outline skin and follows the skin target;
`[143,62] #CD9067` is real first-frame hair and stays source-colored;
`[439,60] #CD9067` is the restored speaking-frame hair patch and also stays
source-colored. `[148,61] #E5AF7E` is the light skin fringe and follows its
existing intermediate skin target.

## Fresh verification

The candidate is `generated/holt-moustache-edge`. CPU helpers are retained in
`local/authoring/holt-moustache-edge`; fresh evidence and byte-exact input copies
are under `tmp/holt-moustache-edge/author`. The before candidate is pinned by
all 2,860 variant PNG/metadata hashes rather than copied again.

Observed the literal skin regression fail before editing: **3,040 of 3,040
assertions failed** because the points still retained source skin colors.
The same check passes after rebuilding, with zero failures. All **160 literal
hair assertions**, **680 protected landmarks**, **5,760 restored hair pixel
assertions**, and **six existing blush assertions** also pass.

The complete proof compares **143 selected strips × ten choices = 1,430 PNGs**
against `generated/adeline-holt-face-followup/holt`. Exactly **320 portrait PNGs /
3,200 RGBA pixels** change: 128 PNGs / 1,280 pixels in the original four choices,
and 192 PNGs / 1,920 pixels in the six additions. Every changed pixel follows
its existing skin target. All other portrait pixels and alpha match over
**34,099,200 decoded affected pixels**. All **1,110 world PNGs** and **1,430
metadata files** are byte-exact. No color targets or recipe objects change.

A second construction compares the final candidate with the pre-restoration
roster candidate. Exactly **160 portrait PNGs / 5,760 pixels** differ, all real
hair restored to source: 64 old-four PNGs / 2,304 pixels, and 96 new-six PNGs /
3,456 pixels. All other images match. The resulting old-four exception is thus
limited to the true hair components, with no bright skin fringe included.

All **32 portrait strips / 64 native frames / 640 palette frame cases** are
covered. All 64 seasonal source mouth crops and 640 actual ten-tone mouth crops
match their sixteen reviewed Spring expression/frame patterns. The author
inspected the complete Vanilla/Before/After pattern sheet and all 176
Vanilla-plus-ten-choice crops. Bright isolated fringes are removed while the
moustache's central speaking shape remains intact.

`checks.json`, `regression-before.log`, `literal-fail-before.json`,
`regression-after.log`, `pattern-audit.json`, `final-vs-pre-moustache.json`,
`source-inputs.json`, `candidate-files.json` and `handoff-inputs.json` retain
counts and hashes. `before-inputs.json`, `before-candidate-ref.json` and
`before-candidate-files.json` pin the previous inputs and outputs.

The parent owns the shared regression fixture and current-only old-four
exception, authoring replay, package build, integrated installation/removal and
one focused review. This author changed no tests, runtime, shared planning or
historical artifacts. No GPU/game rendering, image generation, staging, commit
or publication occurred. Automated checks do not establish visual acceptance or
a new live-game result.

## Retention

Retain the approximately 50 MiB complete candidate and small frozen input/proof
packet for review and reproduction. No complete game archive or redundant prior
candidate copy was created. Earlier review and correction packets remain intact.
