# Hemlock and Louis portrait batch

This batch adds all 32 Hemlock and 32 Louis portrait strips in the supplied
archive: eight expressions in each of Spring, Summer, Autumn, and Winter.
Each strip has two 296×180 frames. Both characters have Vanilla, Debug Blue,
Hayden, Ryis, and Seridia choices, with Vanilla selected at launch. F cycles
Hemlock; V cycles Louis. The shared Rust and GML runtime is unchanged.

The complete collection has 1,628 source strips and 6,512 recolored variants
across twenty-one characters: 1,611 portraits and Adeline's 17 previously
reviewed world animations. Earlier characters retain their accepted recipes,
including Errol's wrist and chest hair-fringe correction.

Hemlock uses the corresponding seasonal atlases. All Louis portraits use
`PortraitsMisc`, despite their seasonal directories. The registry and isolated
trial use the actual source metadata. All 64 names match the game's native
NPC outfit and expression definitions.

Character-specific mask details are in [Hemlock portraits](hemlock-portraits.md)
and [Louis portraits](louis-portraits.md).

Hemlock uses seven skin shades, including fine eye, neck, and arm shading.
Gray stubble and arm hair, the earring, and clothing retain their original colors.
Louis uses sixteen shades, including nearly identical opaque blends at the
raised hand and forehead. His glasses, hair, cravat, and cuffs stay original.
The complete visual sweep caught Louis's lower-neck omissions and Summer cuff
spill. The final correction adds 544 neck pixels and restores 176 cuff pixels
per target across sixteen strips, with no other pixel or metadata changes.

The author and independent reviewer inspected both frames of every source
strip in Source/Blue/Ryis comparisons, enlarged faces, and seasonal hand, neck,
and mouth crops. All 256 new target strips passed exact pixel checks, including
83 independent source-art landmarks. Hemlock changes 118,560 pixels per target.
Louis selects 70,164; Ryis changes 69,148 because 1,016 selected eye-fringe
pixels already equal that palette's darkest shade. Both authored galleries
reconstruct the actual Blue output exactly and reuse all 64 frames.

## Local visual check

The compact comparison is `generated/hemlock-louis-preview/summary.png`, with
`index.html` beside it. The combined bundle is
`generated/characters-hemlock-louis-trial`. Images, packages, extracted assets,
and trial game copies remain ignored and local.

The `tmp/play-characters` launcher uses `tmp/hemlock-louis-playtest` with separate
saves and state, mounting the supplied game read-only. F7 opens on Hemlock;
F4 moves to Louis. F/V cycle their palettes. F5 changes season; F7 and Page Up
move through expressions. F9 checks independent selections and fractional
portrait phase. These preview controls are excluded from the player package.

F has no default game action. V is used for Ctrl+V in the visible debug console;
plain V has no default action. Delete also opens that console, and End is not
supported by the pinned MMAPI hotkey resolver, so neither was assigned.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when the source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test hemlock_portraits --test louis_portraits -- --ignored --nocapture'
```

The collection test first failed with `Unsupported character: hemlock`.
After integration, all four character tests passed, covering standalone builds,
the twenty-one-character collection, F/V controls, Vanilla-first choices,
generated pixels, and Louis's seasonal paths with `PortraitsMisc`.

Formatting, Clippy, all 87 active tests, the two local corpus tests, and the
release build passed through the pinned Fenix Nix shell. The three optional
GML interpreter tests also passed. The new corpus tests include 37 Hemlock and
121 Louis literal landmarks, with omission and spill failures observed before
the corresponding recipe corrections.

An independent source audit matched every PNG, metadata entry, registry path,
source hash, frame size, atlas, profile region, preset arity, and sampled shade
role. All 6,512 combined variants passed validation. The new variants match
the art-reviewed outputs byte-for-byte; all nineteen previous character trees
match the accepted Errol-arm bundle, including original PNGs, recolors,
metadata, and reports. The compact summary PNG was visually inspected.

The normal CLI installer built and published a fresh isolated MOMI archive,
verified its pixels and animation metadata, and reported all twenty-one
characters with the expected controls. The installed archive's SHA-256 matches
its receipt. The supplied source archive remains byte-identical.

The headless game exercised all 64 new sources under all five choices, plus
96 existing-character outfit representatives: 160 sources and 800 distinct
source/preset pairs. Checks covered F/V cycles, seasonal and character wraps,
Vanilla-first startup, independent selections across all twenty-one characters,
sprite identity, retained fractional phase 1.25, and a hidden debug console.
There were no script errors, disabled handlers, or selection leaks. The game
exited gracefully with status 0 after SIGTERM. Blue and Ryis screenshots of
both new characters were inspected. The normal desktop launcher was rebuilt
and published as `tmp/play-characters`; an interactive desktop session remains
for user review.

Ignored evidence includes:

- `tmp/hemlock-louis-art-review-final.md`
- `tmp/hemlock-louis-final-checks.log` and `tmp/hemlock-louis-gml-checks.log`
- `tmp/hemlock-louis-native-portraits.json` and `tmp/hemlock-louis-integration-audit.json`
- `tmp/hemlock-louis-all-variants-validation.log`
- `tmp/hemlock-louis-previous-output-check.log` and `tmp/hemlock-louis-reviewed-output-check.log`
- `tmp/hemlock-louis-bundle-inputs.sha256` and `tmp/hemlock-louis-install-report.json`
- `tmp/hemlock-louis-live-run.log` and `tmp/hemlock-louis-live-audit.json`
- `tmp/hemlock-louis-headless/state/` for game logs and screenshots
