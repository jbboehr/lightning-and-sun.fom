# Adeline spring and summer coverage

This slice extends the [NPC palette trial](npc-palettes.md) to Adeline's 25 summer
portrait strips while retaining the 25 approved spring strips. Each strip has two
frames. The combined trial offers Vanilla, Debug Blue, Hayden, Ryis, and Seridia;
only Adeline is recolored. Autumn, winter, beach, wedding, other special portraits,
and overworld sprites still need separate review.

## Source masks and art review

`palettes/profiles/adeline-spring-summer.json` contains the unchanged spring
regions and 25 new summer regions. Summer uses the same four source colors:
`#E3A17B`, `#D48363`, `#C47054`, and `#9F5544`. Its 1,108 seeds are bound to each
source PNG's SHA-256 and dimensions. The source archive remains
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The initial summer candidates came from four-connected source-color components
that overlapped the corresponding reviewed spring masks. They were then inspected
against the summer originals, including the newly exposed chest and arms,
disconnected skin shading, wrist jewelry, cape border, and gold skirt trim.
Both frames of all 25 expressions were reviewed across all four target ramps on
`generated/adeline-summer-previews/page-1.png` through `page-9.png`. Lips and blush
retain the current treatment; this is not a new face-detail art pass. The user
subsequently approved the summer appearance and reported the notification issue
addressed below.

Every summer variant changes 85,361 pixels and preserves 9,512 matching source-color
pixels outside the skin mask. Combined with spring's 80,951 changes, that is 166,312
changes per preset, or 665,248 across the four presets. All spring PNGs and sidecars
in the combined output match the preceding NPC trial byte for byte.

Local originals are in `extracted/adeline-seasonal-study`, and the generated
bundle is `generated/adeline-seasonal-trial`. Authoring helpers, component reports,
images, game copies, and generated mods remain ignored. Only transformations,
checksums, seeds, code, tests, and documentation belong in Git.

## Packaging and installation

The explicit exporter now accepts up to 50 distinct PNGs. Toggle packaging accepts
Adeline's 25 supported expressions in spring and summer, with exact basename
validation. The installer also requires the canonical folder for the named
season. A summer basename under `Portraits/Spring/` is rejected.

Each new animation retains its source season's atlas and frame properties. MOMI
assigns fresh animation IDs. Installed verification loads the atlas families
needed by the selected portraits, including numbered pages, and checks each frame
only within its correct family. Identical pixels on a spring page cannot satisfy
a missing summer frame. Summer-only selections are supported as well.

The existing generic GML sprite table supports both outfits without a runtime
script change. The selected preset is applied after a supported speaker/portrait
change, preserving its fractional animation phase. Vanilla remains the launch
default. Selection is session-only.

Use the combined set after removing an existing palette installation:

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --presets palettes/sets/adeline-seasonal-trial.json
```

For blue only, use `--palette palettes/stylized/adeline-spring-summer.json`.
Existing spring-only sets and the default neutral-only installation remain usable.
The previous mod-list, load-order, backup, source matching, and uninstall rules
continue to apply.

## Verification

The new 50-strip packaging, export, and mixed-season installation tests failed on
the prior spring-only implementation, then passed after the change. Synthetic
cases also cover summer-only installation, corrupt summer pixels, summer frames
on the wrong atlas, mismatched metadata atlases, and a wrong seasonal folder.
The GML lifecycle test verifies palette and raw animation phase across both
outfit directions.

The local corpus test checks all 100 frames, unchanged alpha and metadata, the
skin-only shade including the disconnected sly pixels, summer chest/arm landmarks,
clothing landmarks, and every pixel below the reviewed skin boundary. An
unrestricted recolor fails this test on clothing, as recorded in
`tmp/seasonal-negative-mask.log`. A separate mutation retaining the spring masks
but selecting every summer component fails on summer skirt trim at [114,148];
see `tmp/seasonal-negative-summer-mask.log`. `FOM_SEASONAL_RECIPE` selects an alternate recipe
for this negative check.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test seasons seasonal_masks_cover_exposed_skin_and_preserve_summer_clothing -- --ignored --exact --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

The real MOMI roundtrip used `tmp/cli-installer-lab` with its existing probe mod:

```sh
target/release/mistria-palette install --game-dir tmp/cli-installer-lab \
  --installed-mods tmp/cli-installer-lab/config/mods/manifest.json \
  --presets palettes/sets/adeline-seasonal-trial.json
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab
cmp tmp/cli-before-othermod.zip tmp/cli-installer-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.bak.zip
```

Installation verified all 500 original/preset frames across the two atlas
families. Removal restored the exact prior archive, and the pristine backup
remained unchanged. Reports are `tmp/seasonal-install-report.json` and
`tmp/seasonal-remove-report.json`. Separate exact recipe validation passed for all
four generated variants; reports are `tmp/seasonal-<id>-validation.json`.

## Local playtest

`./tmp/play-masked` now contains the NPC spring trial. It was left alone while
preparing the separate combined installation at `tmp/seasonal-playtest`.
Run `./tmp/play-summer` from the normal NixOS Wayland desktop terminal:

- F7 opens the portrait test and advances the expression; F10 goes backward.
- F5 switches spring/summer while keeping the selected palette.
- F6 cycles the five choices; F8 tests talking frames.
- F9 checks fractional animation phase through a complete palette cycle.

The local helper uses the game's wardrobe setter and seasonal atlas load/unload
functions. These controls are confined to the ignored playtest mod. They are not
part of the distributed palette package. The launcher uses a separate state
folder and a read-only mount of the original game, with only the lab archive
overlaid. The original build's logs go to `tmp/seasonal-playtest/state/game.log`;
the updated launcher below uses a new isolated copy.

The real engine run observed all **500 distinct season/expression/preset/frame
combinations**, with no missing entries against the profile-derived expected set.
It passed 60 fractional-phase cycles and retained each of the five selections
across both spring-to-summer and summer-to-spring atlas reloads. The log is
`tmp/seasonal-headless/state/game.log`; the sorted coverage lists are
`tmp/seasonal-{expected,observed,missing}-frames.txt`. The process exited gracefully.
The isolated launch reported the expected unavailable-Steam initialization error;
there were no portrait or phase assertion failures.

A separate slower visual run captured summer neutral with each preset after
notifications cleared. These screenshots are in `tmp/seasonal-visual/state/`,
named `summer-neutral-0.png` through `summer-neutral-4.png` in palette order. This
also exited gracefully. The automated runs used software Mesa under Xvfb; the
user launcher uses the desktop's GPU drivers.

## Reliability result

**PASS_WITH_RESIDUAL_RISK.** Independent Breaker and Test Attacker reviews found
no demonstrated defect or accepted static finding. The test review strengthened
the installer fixture with a correctly placed summer original whose variant
exists only on the spring atlas. It reaches the expected missing variant frame
and confirms that rejection leaves the live archive and recovery state untouched.

Fresh final formatting, Clippy, all **62 active tests**, and the release build
passed after that test change. The combined corpus and Fabricator lifecycle
checks also passed in the final run, recorded in `tmp/seasonal-final-checks.log`.
The existing spring and neutral local mask checks passed separately in
`tmp/seasonal-prior-mask-checks.log`. The two legacy MOMI opt-in fixtures were not
rerun; the current combined installation was exercised by the real roundtrip
above instead.

The user approved the summer appearance ("looks pretty good") and reported that
palette notifications obscured the portrait. A natural calendar season transition
remains untested during ordinary gameplay. The controlled engine test
exercised the same wardrobe and atlas operations directly, without advancing the
calendar or testing unrelated seasonal gameplay.

## Portrait-obscuring popup fix

F6 previously called `InfoToasts.create_notification` on every selection. The
game stacks those notifications over the left-hand portrait, and F9's full
palette cycle queued five at once. The production toggle now keeps its log entry
and creates no HUD popup. The local playtest shows the selected palette at the
start of its dialogue text and refreshes that label when the selection changes.

The Fabricator regression failed at the no-popup assertion before removing the
two notification calls, then passed with log labels and animation phase checks
intact. Formatting, Clippy, all 62 active tests, the GML lifecycle test, and the
release build passed (`tmp/quiet-toggle-{red,checks}.log`).

A fresh MOMI installation in `tmp/quiet-seasonal-playtest` verified all original
and variant frames plus the updated scripts. `./tmp/play-summer` now launches
that copy; earlier playtest copies and logs are retained. A focused real-engine
run cycled all five choices on spring and summer neutral, passed ten F9 phase
checks, and exited gracefully. Screenshots taken immediately after those cycles
show the complete portrait and the selected name in the dialogue box, without
palette popups. Evidence is under `tmp/quiet-notifications/state/`; the install
report is `tmp/quiet-seasonal-install-report.json`. The earlier 500-combination
art/atlas pass was not repeated for this notification-only change.
