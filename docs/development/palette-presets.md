# Native palette catalog and shared portrait presets

The first preset slice inventories the game's creator palettes and uses three
of them on the reviewed Adeline spring portraits. Vanilla remains the session
default and Debug Blue remains available. This follows the user's approval of
the full spring blue study. It does not yet identify every NPC's skin ramp.

## Catalog evidence

The supplied archive's SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
`assets/animations/Player/Base/spr_player_base_lut.png` is 37×256, with SHA-256
`ef2b59df161aff6c7c0a17c89766ff0187851399d2ad42df9990c485ef8e7c0d`.
Column zero is the source lookup column; columns 1–36 are creator options. The
local customization-menu code derives the option count from the width minus one.

Rows 225, 186, 125, and 82 supply the four main skin colors from light to deep
shadow. These are zero-based pixel coordinates. The table has other varying rows,
including detail colors, which are not part of this four-color portrait trial.
All 36 main ramps are distinct in this build. The catalog retains every entry
even if a future supported table contains duplicate ramps, recording the first
matching entry as `same_ramp_as`.

`catalog` also scanned 2,303 PNG strips under NPC/cameo `Portraits/` paths. Their
opaque-color histograms are explicitly `unreviewed_colors`. Clothes, hair, animal
portraits, and special lighting prevent treating the histogram as a skin palette.
Manual review of representative neutral portraits confirmed that NPC portraits
need their own source-color classification. A full NPC ramp catalog remains work
for a later art slice.

Only numeric creator colors and provenance are stored in
`palettes/catalog/player.json`. Full histograms, source previews, recolored PNGs,
and generated MOMI packages stay in ignored local directories.

## Shared masks and trial configuration

`palettes/profiles/adeline-spring.json` holds the original four source colors and
all 25 source-bound region definitions. The blue recipe references it, and
`palettes/sets/adeline-trial.json` pairs that profile with Debug Blue and creator
options Player 01, 18, and 33. Every target ramp uses the same source-color order
and source pixels to resolve mask connectivity. Changing a target color cannot
select more skin or recolor matching robe and jewelry pixels outside the mask.

The shared-profile blue tree is byte-for-byte identical to the approved spring
blue output. Each of the four variants changes 80,951 pixels across 50 frames;
the combined variant report totals 323,804 changes. Source images and metadata
retain their existing validation rules. Inline region recipes remain supported.

`build-presets` creates each variant and one MOMI package in a temporary tree,
then publishes the complete bundle. `install --presets` uses the same generation
path and checks every installed variant before publishing the game archive.
The set format supports one through eight variants with unique safe IDs.
`--palette` and `--presets` cannot be combined.

The generated GML supplies a sprite table and labels in preset order. F6 cycles
Vanilla, Debug Blue, Player 01, Player 18, and Player 33 for the trial. It preserves
the current expression and raw fractional animation phase. Later speaker changes
use the active choice. Every game launch starts at Vanilla. The existing HUD
notification API displays the chosen label; rapid cycling can briefly stack
notifications over the portrait. Missing any included animation disables the
palette hotkey at initialization.

Nine local comparison pages cover both frames of all 25 expressions:
`generated/adeline-preset-previews/page-1.png` through `page-9.png`. The four-color
creator ramps are portrait adaptations, not fully curated NPC art. Lips, blush,
and outlines retain their previous treatment. Their contrast is particularly
visible in the deepest preset. The face-detail review below keeps the approved
prototype appearance and records an optional alternative.

## Lip and blush review

After the user accepted the five-choice trial, the next slice inspected face
details before making further art changes. All 25 expressions, both frames, and
all five choices were compared at 8× nearest-neighbor zoom. The close-ups are
`generated/adeline-face-review/<expression>.png`; `face-colors.json` records
source-color coordinates in each crop. The local authoring helper is
`tmp/face-review.rs`.

The review found no mouth-shape or talking-frame problem that justified changing
the approved lips. The existing skin shades around the mouth follow the selected
ramp; pink highlights and mouth-interior colors retain their original values.
Some pink lip pixels use `#E3779D`, which also occurs in the hair. Adding this
color to the shared skin map would need separately reviewed selection boundaries.
A broad recolor of pink pixels is not a suitable lip-only change.

Blush has distinct source colors beyond the four main skin shades:

| Source color | Reviewed use | Pixels across both frames of all 25 strips |
| --- | --- | ---: |
| `#E37B7B` | Most dotted cheek blush | 244 |
| `#E6687A` | Cartoon embarrassed cheek blush | 34 |
| `#DE8074` | Tired/sick nose flush | 234 |
| `#DE9374` | Lighter sick nose flush detail | 32 |

These 544 pixels occur in 14 expressions. The local experiment checked that every
occurrence lies in the inspected face bounds: frame-relative x=132..169,
y=58..92. The warm sick/tired nose patch is present in the original art; keeping
its colors makes it contrast more strongly with blue and Player 33. This is a
color-treatment choice, not an omitted patch of the four-color skin mask.

A local alternative for Debug Blue and Player 33 blends each of these four blush
colors equally with the preset's main skin color, using integer sRGB channels
rounded down. This is an art experiment, not a recovered game formula. It uses a
second source-bound `apply` recipe against the existing generated variant, with
seeds for the reviewed blush pixels. Lips and all other pixels remain unchanged.
The candidate changes 544 pixels in each preset and preserves alpha and metadata.
The 11 expressions without these blush colors retain their PNG bytes.

Current/softer comparisons for `blush`, `neutral_tired`, and `sick_eyes_closed` are
under `generated/adeline-blush-comparison/`. That directory also contains both
recipes, candidate trees, baseline validation recipes, and `study.json`. The
local helper is `tmp/blush-comparison.rs`. All generated images and helpers stay
ignored; the candidate was not installed or added to a production preset.

Decision: retain the user-approved lips and blush for this prototype. The softer
candidate reduces the peach contrast on Player 33, but makes the flush less
distinct on blue. It is not a clear improvement across both presets. These are
visual judgments; they do not establish a universally correct color treatment.
Manual NPC ramp classification is the next planned slice. Blush refinement can
be revisited if the user prefers the local alternative.

Exact recipe validation passed for all four baseline variants at 80,951 changed
pixels each. Both candidate passes also validated at 544 changes each. The local
experiment independently checked that every non-blush pixel was unchanged.
Reports are `tmp/face-review-<id>-validation.json` and
`tmp/soft-blush-<id>-validation.json`. This slice changes documentation only in the
tracked tree; runtime code, preset definitions, and the installed trial retain
their approved state. The alternative has not had an in-game visual pass.

Fresh verification after this review passed formatting, Clippy, all 59 active
tests, the release build, and the optional full-spring mask test. The unchanged
game runtime and MOMI installation were not rerun for this documentation and
local-preview slice.

## Atlas cost and installation evidence

The source spring atlas is one 4096×4096 page. Both the blue-only full spring
study and the four-variant trial use two 4096×4096 pages after MOMI builds them.
At RGBA8 that is 128 MiB of base texture storage, versus 64 MiB for the original
spring atlas. The trial adds no page beyond blue alone. This is a texture-size
calculation, not a measurement of total game RAM or GPU allocation; compressed
ZIP size does not represent runtime texture cost.

The actual MOMI trial in `tmp/cli-installer-lab` retained the existing probe mod,
verified all 250 original/variant frames and both GML files, then uninstalled
back to the exact prior archive. A second roundtrip using the single
`--palette palettes/stylized/adeline-spring.json` profile reference also passed.
The retained pristine backup matched the read-only source archive. Local reports
are `tmp/presets-install-report.json`, `tmp/presets-remove-report.json`, and the
corresponding `tmp/presets-single-*-report.json` files.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
nix-shell --pure --run 'cargo test --locked --test spring spring_masks_cover_skin_and_preserve_reviewed_clothing_landmarks -- --ignored --exact --nocapture'
nix-shell --pure --run 'cargo test --locked --test masks adeline_regions_preserve_clothing_and_cover_both_frames -- --ignored --exact --nocapture'
```

The implementation tests were observed failing before their respective changes:
missing catalog/build commands, unsupported profile references, third-preset
runtime cycling, and the aggregate changed-pixel report. Synthetic tests protect
mask boundaries, malformed set rejection without output, and corruption in the
last installed variant. The Fabricator test exercises multiple expressions and
presets, wraparound, phase preservation, missing assets, and label notifications.

The isolated game-engine run observed all 250 distinct expression/preset/frame
combinations, ran 25 full-cycle fractional-phase checks, exercised next/previous
expression wrapping, and exited normally. The HUD notifications rendered with
the chosen names. Evidence is in `tmp/presets-headless/state/game.log` and its
screenshots; the helper is `tmp/presets-session.sh`. This run uses Mesa software
rendering for automation and does not measure desktop GPU performance.

The user's existing `./tmp/play-masked` launcher now opens the five-choice trial
with GPU rendering on their desktop. F7 opens/advances the expression test, F10
moves backward, F8 restarts talking, and F6 cycles palettes. The prior approved
blue-study log is preserved in `tmp/spring-approved-user-playtest.log`. The user
reviewed the new trial and reported "looks pretty good"; these presets are the
prototype baseline. This feedback does not establish an exhaustive manual pass
through every expression and preset.

Independent correctness review identified that an explicit `profile: null` was
being treated as an omitted profile, silently enabling unrestricted replacement.
A regression reproduced four changes instead of the fixture's two reviewed
pixels. The parser now rejects explicit null. Independent test review also
demonstrated publication of a variant with `asset_kind = 'Script'`; this was a
pre-existing gap retained by the verifier refactor. Every variant must now declare
`Animation`. Both regression tests failed before their fixes and passed afterward.
No accepted unreproduced findings remain.

Test hardening also exercises one/eight accepted variants, zero/nine rejected
variants, the complete nine-choice GML cycle including Vanilla, and a missing
final asset. Final formatting, Clippy, all 59 active tests, the release build, and
the three current opt-in tests above passed after the fixes. The two older MOMI
opt-in tests were not rerun against their stale one-portrait labs.

The final generated bundle is `generated/adeline-presets-final`; its package is
byte-for-byte identical to the package exercised by the engine test. Its blue
variant tree is also identical to the approved blue spring study.

The real MOMI roundtrip was rerun after the verifier fix:

```sh
target/release/mistria-palette install --game-dir tmp/cli-installer-lab \
  --installed-mods tmp/cli-installer-lab/config/mods/manifest.json \
  --presets palettes/sets/adeline-trial.json > tmp/presets-final-install-report.json
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab \
  > tmp/presets-final-remove-report.json
cmp tmp/cli-before-othermod.zip tmp/cli-installer-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.bak.zip
```

All commands passed. Reliability verdict: **PASS_WITH_RESIDUAL_RISK**. Both
demonstrated validation defects are fixed, with fresh full checks and a real MOMI
roundtrip. Manual NPC ramp classification remains planned; the face-detail review
above leaves further lip/blush refinement optional for the prototype. Desktop
performance and larger preset sets have not been measured in the game engine.
