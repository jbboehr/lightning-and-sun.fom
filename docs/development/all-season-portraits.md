# Adeline's four seasonal outfits

The subsequent [beach extension](beach-portraits.md) adds 19 strips to this set.
This page preserves the four-season slice's results.

This extends the [spring/summer trial](seasonal-portraits.md) with 25 autumn and
25 winter strips. All 100 strips have two frames and the same five choices:
Vanilla, Debug Blue, Hayden, Ryis, and Seridia. Beach, wedding, other special
portraits, overworld sprites, and additional characters remain outside this set.
Lips and blush retain the preceding treatment.

## Source masks

The new profile is `palettes/profiles/adeline-all-seasons.json`; the first 50
regions equal the preceding spring/summer profile. Its preset set is
`palettes/sets/adeline-all-seasons-trial.json`, and its blue-only recipe is
`palettes/stylized/adeline-all-seasons.json`. The older smaller sets still work.

The four source colors remain E3A17B, D48363, C47054, and 9F5544. Every region is
bound to its source PNG's SHA-256 and dimensions. The archive used for review is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

| Outfit | Strips | Seeds | Changed pixels per preset | Matching pixels preserved outside skin |
| --- | ---: | ---: | ---: | ---: |
| Spring | 25 | 872 | 80,951 | 20,598 |
| Summer | 25 | 1,108 | 85,361 | 9,512 |
| Autumn | 25 | 600 | 48,691 | 11,390 |
| Winter | 25 | 498 | 41,341 | 1,384 |

Autumn and winter candidates started with connected source-color components
overlapping the reviewed spring skin. The body regions were then reviewed
separately: autumn's exposed neckline, both hands, and the gold collar, cuffs,
and cape trim. Seven poses raise the torso by one pixel: blush, embarrassed,
gloomy_special, hope_special, sigh, sly, and wink. The source-bound seeds account
for those offsets. No runtime image matching or inferred region selection is used.

Both frames of every new expression were inspected across all four target ramps
on `generated/adeline-autumn-previews/page-1.png` through `page-9.png`, and the
equivalent winter pages. The combined output changes 256,344 pixels per preset,
or 1,025,376 across the four. All generated spring/summer PNGs and metadata match
the preceding trial byte for byte.

## Packaging and checks

Export now accepts up to 100 distinct explicit PNG paths. Packaging recognizes
the existing 25-expression allowlist in all four seasons. Installation requires
each basename's canonical source folder. Autumn uses `Autumn` and
`PortraitsAutumn`; the game calendar calls this season `Season.Fall`.

The existing installed-frame verifier checks originals and generated variants
against their own atlas family, including numbered atlas pages. The production
GML needs no changes: the generated asset table supplies the additional groups.
The Rust changes only extend the existing season cases and count bounds.

The 100-export, 100-portrait package, and autumn installation tests were observed
failing before the implementation changed, then passing. Synthetic installation
cases cover summer, autumn, and winter independently and mixed with spring,
wrong folders, wrong pixels, and original or variant frames placed only in the
wrong atlas family. Rejections preserve the archive and leave no recovery receipt.

The local corpus check verifies both frames, unchanged alpha and metadata,
unrelated colors, hand/chest landmarks, and preserved collar/cuff/trim landmarks.
It also checks every D48363 pixel, a skin-only shade in this reviewed corpus.
The GML interpreter exercises selection and fractional phase through all four
outfits and back to vanilla.

Two deliberate bad recipes proved the corpus assertions detect missed hands and
recolored cuff trim: removing neutral autumn's body seeds failed at `[170,95]`,
and adding the gold cuff seed `[181,104]` failed the preservation assertion.
Evidence is in `tmp/all-seasons-{missing-hands,painted-cuff}.log`. Exact recipe
validation passed for all four presets. The previous spring/summer, spring-only,
and neutral mask checks also passed (`tmp/all-seasons-prior-masks.log`).

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test autumn_winter -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

Originals are in `extracted/adeline-all-seasons-study`; the generated bundle is
`generated/adeline-all-seasons-trial`. Source art, modified art, authoring helpers,
preview sheets, game copies, and saves remain ignored. Only transformations,
checksums, seeds, tests, code, and documentation belong in Git.

## Installation and playtest

A real MOMI roundtrip in `tmp/cli-installer-lab` checked all 100 original and 400
variant strips before publication. Uninstall restored the prior probe-mod archive
exactly (`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`),
and `assets.bak.zip` still matched the source game. Reports are
`tmp/all-seasons-roundtrip-{install,uninstall}.json`.

`./tmp/play-seasons` launches `tmp/all-seasons-playtest` with its own archive and
save/config directories. The original game is mounted read-only. Existing
`play-summer` and `play-masked` copies are retained. Controls:

- F7: open the portrait or advance to the next expression; F10: previous.
- F5: cycle spring, summer, autumn, winter.
- F6: cycle the five palette choices; the dialogue label shows the current name.
- F8: replay talking animation; F9: check fractional phase across a full cycle.

The local helper also has an F4 calendar-rollover check. Start a fresh session,
open the portrait, and select a palette. F4 seeds the current season's last day
at 2am, then the game performs its normal faint, overnight summary, calendar
increment, atlas reload, wardrobe update, save, and wake-up sequence. Choose
Next Day and dismiss the oversleep dialogue, then F7 reopens Adeline. F4 requires
the displayed outfit to match the calendar; use a fresh session if F5 has moved
it elsewhere. This control is local test scaffolding, not part of the palette mod.

The successful calendar run is in `tmp/calendar-rollover-v2/state/`. It observed
all four normal `new_day()` rollovers, including the new year, with Debug Blue
entering summer, Hayden entering autumn, Ryis entering winter, and Seridia entering
spring. The helper asserted the game's resulting wardrobe and retained choice,
without assigning the outfit after the rollover. Both neutral frames and F9 phase
checks passed after each wake-up; `after-<season>.png` records the rendered result.
The first automation attempt stopped short of the later transitions because the
normal oversleep dialogue needed dismissal before reopening the test portrait.
The successful run handles that dialogue and asserts the portrait has reopened.

The calendar date and clock were seeded to the boundary in an isolated new game.
This tests the normal overnight transition, not a month of unscripted play or
naturally encountered story dialogue. Palette choice remains session-only; the
save operation does not make it persist after restarting the game.

A separate engine sweep in `tmp/all-seasons-engine/state/` observed all **500**
new combinations: two outfits × 25 expressions × five choices × two frames.
Comparing the expected and observed sets found zero missing combinations. It
also passed 70 fractional-phase cycles and 23 outfit changes, including a full
four-outfit cycle with each selected palette. Spring/summer's full expression
sweep was not repeated; their generated files were compared byte for byte above.
Both engine runs exited gracefully. They used software Mesa under Xvfb; the
desktop launcher uses the host GPU drivers. The only engine error was the
expected unavailable Steam initialization inside the isolated environment.

Fresh formatting, Clippy, all 62 active tests, the release build, the new local
corpus check, and the Fabricator lifecycle check passed. The final run is recorded
in `tmp/all-seasons-final-checks.log`. The two legacy MOMI opt-in fixtures were not
rerun; the current 100-portrait roundtrip above exercised the real installer.
The new outfits still need the user's visual approval.
