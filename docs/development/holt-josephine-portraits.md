# Holt and Josephine portraits

This batch covers all 32 main portrait strips for each character: eight
expressions in each of spring, summer, autumn, and winter. Each strip has two
296×180 frames. Neither corpus includes beach, bath, or wedding outfits.

Separate authors own [Holt's masks](holt-portraits.md) and
[Josephine's masks](josephine-portraits.md), with an independent art reviewer
checking the actual recolors. The user receives one compact PNG comparison and
one combined trial. Detailed component selection remains an authoring task.

## Integration

Both characters offer Vanilla, Debug Blue, Hayden, Ryis, and Seridia palettes.
Vanilla remains the default at launch; **H** cycles Holt and **P** cycles
Josephine independently. These keys are free in the supplied game's default
settings and direct keyboard handlers; custom player bindings can differ.
They remain temporary controls until a character-and-palette menu is added.

The registry records every original PNG path and its actual atlas. Holt uses
eight strips in each seasonal portrait atlas. Josephine's filenames encode the
season inside a flat `Portraits/` directory, and all 32 use `PortraitsMisc`.
The source NPC definitions recognize all 64 outfit/expression pairs directly.
This batch requires no shared Rust or GML runtime changes.

The combined collection contains 15 characters and 1,428 source strips:
1,411 portraits plus Adeline's 17 world animations. Four generated targets
produce 5,712 variants. Holt and Josephine's overworld sprites remain outside
this slice. The previously deferred Hayden, Reina, and Juniper mouth details
remain separate follow-ups.

## Local trial

The later [lower-mouth edge correction](holt-portraits.md#lower-mouth-edge-correction)
updates the current launcher to `tmp/holt-mouth-fix-playtest`, starting on Holt.
Its summary and before/after close-up are in `generated/holt-mouth-fix-preview/`.
The original batch and verification below remain the baseline.

`tmp/play-characters` uses `tmp/holt-josephine-playtest` with its own save/state
directory and mounts the supplied game read-only. F7 opens on Holt; F4 moves to
Josephine. H and P cycle their palettes. F5 changes season; F7 and Page Up move
through expressions. F9 checks independent selections and portrait phase.

The compact comparison is `generated/holt-josephine-preview/summary.png`, with
`index.html` beside it. The combined bundle is
`generated/characters-holt-josephine-trial`. Game-derived images, packages,
trial archives, and temporary review helpers stay ignored and local.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when the source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test holt_portraits --test josephine_portraits -- --ignored --nocapture'
```

The synthetic collection test first failed with `Unsupported character: holt`.
After registry integration and matching Josephine's flat source path in the
synthetic fixture, all four character tests passed. They exercise standalone
and combined builds, runtime controls, Vanilla-first choices, generated pixels,
and differing preset counts among existing characters.

Formatting, Clippy, all 87 active tests, both new local corpus tests, and the
release build passed. The retained Landen/Nora tests, including Nora's lip
correction, and the three local GML interpreter checks also passed. All 5,712
combined variants passed exact recipe validation. The previous thirteen
characters' entire output trees, including originals, variants, metadata, and
reports, matched `generated/characters-nora-lips-trial` byte-for-byte.

The independent archive audit checked complete PNG/metadata inventories, source
hashes and extracted bytes, original atlas names, dimensions and frame counts,
unique registry paths, exact profile coverage, preset arity, standalone Blue
equivalence, and catalog shade roles. Both characters use nine source colors.
The native NPC definitions recognize every added outfit/expression pair.

A fresh MOMI installation in `tmp/holt-josephine-playtest` verified all selected
Vanilla/variant atlas frames, metadata, and generated scripts before publishing
the trial archive. The real game then checked every new portrait across all
five choices: 64 source strips and 320 distinct source/preset pairs. Actual H/P
cycles, four-season wraps, independent selections, fractional portrait phase,
and an unobscured preview passed. Another 72 distinct portraits checked one
expression per available outfit for each of the previous thirteen characters.
The exact log audit recorded 136 distinct sources and 680 source/preset pairs,
with no script errors or disabled callbacks. Holt's Blue and Josephine's Ryis
screenshots were inspected. The harness completed successfully and the game
logged a graceful exit.

The supplied archive remains unchanged at SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
This slice did not rerun uninstall or the full Adeline world-action game matrix.
Desktop Wayland play and visual acceptance remain with the user; the automated
game check used Xvfb and its own state directory.

Local evidence includes `tmp/holt-josephine-final-checks.log`,
`tmp/holt-josephine-gml-checks.log`,
`tmp/holt-josephine-retained-mouth-checks.log`,
`tmp/holt-josephine-integration-audit.json`,
`tmp/holt-josephine-native-portraits.json`,
`tmp/holt-josephine-all-variants-validation.log`,
`tmp/holt-josephine-previous-output-check.log`,
`tmp/holt-josephine-bundle-inputs.sha256`,
`tmp/holt-josephine-install-report.json`,
`tmp/holt-josephine-live-run.log`, and `tmp/holt-josephine-live-audit.json`.

The independent art review finished with PASS and no remaining concrete defect.
It checked all 256 new target strips for exact colors, identical selections,
metadata and alpha, with 110 independent literal source landmarks per target.
Holt selects and changes 194,248 pixels while protecting 62,020 matching-color
pixels. Josephine selects and changes 123,754 pixels while protecting 38,368.
Every target changes every selected pixel in this batch; there are no identity
mapping exceptions.

The reviewer inspected all 128 unique full frames and enlarged Source/Blue/Ryis
face trios, plus 32 distinct five-way Josephine mouth comparisons. Corrections
preserve Holt's ginger scalp/brows, shifted moustache and Winter cuff stitching,
and cover his isolated Spring collar gap. Josephine's fine nose, collar and hand
shading recolors while hair, cosmetics and accessories remain original. Her
dark lipstick fits all four inspected targets; Holt's moustache remains ginger.
The review report is `tmp/holt-josephine-art-review-final.md`, tied to the same
recipe hashes used for combined generation, validation, and installation.

The usual `tmp/play-characters` launcher was rebuilt through Nix, checked as an
executable shell script, and pointed at the verified isolated trial. It starts
on Holt.

The user approved committing this prototype checkpoint after the lower-mouth
edge correction. Josephine's lipstick contrast remains a deferred cosmetic
follow-up: original lip colors can look harsher against some replacement skin
tones even when no skin pixels are omitted. Later polish can adjust lipstick
per palette. The pre-commit checks are recorded in
`tmp/holt-josephine-commit-checks.log`; MOMI installation and interactive gameplay
were not repeated for the commit.
