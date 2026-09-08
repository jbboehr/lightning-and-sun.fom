# Balor and Valen portrait batch

This continues the [parallel portrait workflow](celine-march-portraits.md):
one author per character, a separate art reviewer, and a shared integration owner.
The user receives one compact completed preview. Recipes, source-bound masks,
configuration, tests, and documentation belong in Git; game images and packages
remain under ignored local paths. Earlier mouth-detail follow-ups for other
characters stay deferred.

## Coverage

| Character | Spring | Summer | Autumn | Winter | Beach/bath | Wedding | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Balor | 21 | 21 | 21 | 21 | 20 | 6 | 110 |
| Valen | 17 | 17 | 17 | 17 | 17 | 7 | 92 |

These profiles cover all main portrait strips in the supplied archive. Overworld,
UI, and child sprites remain outside them. Vanilla remains the default; Debug
Blue, Hayden, Ryis, and Seridia colors follow. The nine-character collection has
1,186 originals: 1,169 portraits and the existing 17 Adeline world animations.
Four recolors produce 4,744 variant strips.
Authoring details are in [Balor](balor-portraits.md) and
[Valen](valen-portraits.md).

## Integration

Both characters fit the existing 256-strip export limit. The standalone/combined
package test first failed on unsupported Balor, then passed after adding the
registry entries. The previous seven entries remain unchanged. Shared production
Rust and GML behavior does not change for this batch.

All 202 new paths and their atlases were read from source metadata. Beach/bath
portraits use `PortraitsSummer`; wedding portraits use `PortraitsMisc`. Every
new filename resolved against its native NPC outfit/expression definition without
special translation in the local preview.

I cycles Balor; O cycles Valen. MOMI accepts these single-letter key names, and
neither occurs in the supplied default input bindings or direct keyboard handlers.
Existing palette keys remain available. Customized bindings can still overlap
these prototype controls; the planned palette menu can replace them later.

## Local build and preview

```sh
target/release/mistria-palette build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-balor-mouth-trial
```

The initial batch comparison is `generated/balor-valen-preview/index.html`.
The current Balor mouth comparison is `generated/balor-mouth-preview/index.html`
and its `summary.png`. The ignored launcher uses `tmp/balor-mouth-playtest` with
separate state and read-only source game mounts. F7 opens Balor and advances expressions;
Page Up goes back, F4 switches characters, and F5 switches outfits. I cycles
Balor's palette; O cycles Valen's. Existing palette keys remain available.

For optional Adeline world checks, press F1 after portrait switches to restore
the reviewed Spring actor before using F3/F12/F11. The scheduling guard from the
earlier pathfinding fix remains in the local helper. These preview controls are
excluded from the player package.

## Initial batch verification

Both authors inspected every unique frame and validated all four target palettes.
Independent art review passed all 808 new variant strips against every decoded
pixel and literal source landmarks: 47 for Balor and 44 for Valen. The checks
cover source hashes and dimensions, metadata, alpha and transparent RGB,
unchanged unrelated colors, exact target mappings, and identical changed masks
across all four targets. Balor's wrist/sleeve openings recolor while his smile,
pink scar, and clothing remain original. Valen's fine skin shading recolors
while eye/mouth details, sunglasses, and clothing remain original.

The combined build passed exact recipe validation for all 4,744 variant strips.
The source/registry audit matched every new path and atlas, profile/preset arity,
standalone Blue mapping, and sampled NPC target roles. All 202 new portrait names
resolved against the native NPC definitions. Build input hashes remained stable.

MOMI installed the nine-character package into the isolated trial and verified
installed pixels, metadata, generated scripts, and mod selection. Uninstalling
a separate copy restored the source archive byte for byte, retaining SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The combined portrait atlases use 17 pages versus 16 in the accepted seven-character
trial. Their full RGBA8 texture capacity is 1,088 MiB versus 1,024 MiB; this is
not measured resident memory. The preview retains Spring and loads the atlas
groups needed for the current outfit.

Fresh formatting, Clippy, all 87 active synthetic tests, three opt-in GML tests,
both new character corpus tests, and the release build passed. The author tests
first failed on missing skin shades or unprotected facial details before the
final masks passed. No production runtime code changed.

The real game ran with software rendering in isolated sessions. The portrait
pass covered every one of the 202 new strips, plus one expression per outfit for
each of the seven accepted characters: 244 distinct strips and 1,220 exact
portrait/palette pairs. The audit confirmed complete new-character coverage and
rejected unexpected source/palette pairs. Each check asserted the displayed
sprite, fractional animation phase, independent selections across all nine
characters, and a hidden debugger console. I and O each completed a full cycle
back to Vanilla. Blue and natural-palette screenshots were inspected alongside
the compact preview.

The same session passed six atlas transitions with an Adeline world actor active
and switching all nine characters within the beach group. A separate fresh world
session completed all 24 action/facing phase and movement checks. It also observed
every supported action/facing combination; the existing helper skips the absent
north-facing blink artwork. No script or callback error was logged.

The first session's world phase was stopped because the Bash test driver became
slow parsing its growing log. Extracting the latest label with whole-string glob
removal took 18.294 seconds in a local reproduction. Reading lines backward with
`tac` returned the same label in 0.006 seconds. Navigation still waits for the
current label and fresh check completion. The fresh world run waits for each
completed phase check before advancing. All portrait evidence from the first
session was preserved; only the world phase was rerun. This changes the ignored
test driver, not the game or mod runtime.

Desktop review identified a Balor mouth issue; the correction is recorded below.

Local evidence includes `tmp/balor-valen-characters-red.log`,
`tmp/balor-valen-integration-green.log`, `tmp/balor-valen-final-checks.log`,
`tmp/balor-valen-art-review-final.md`, `tmp/balor-valen-integration-audit.json`,
`tmp/balor-valen-native-portraits.json`, `tmp/balor-valen-build-report.json`,
`tmp/balor-valen-build-inputs.sha256`, `tmp/balor-valen-validation-count.txt`,
`tmp/balor-valen-install-report.json`, `tmp/balor-valen-uninstall-report.json`,
`tmp/balor-valen-atlas-memory.json`, `tmp/balor-valen-live-verification.txt`,
`tmp/balor-valen-main-game.log`, `tmp/balor-valen-world-headless/state/game.log`,
and `tmp/balor-valen-reader-timing.txt`.

## Balor mouth correction

Desktop review found original peach lip shading standing out against the Ryis
palette. The [mask correction](balor-portraits.md#mouth-correction) selects two
lip-shading colors while retaining dark mouth contours and red/pink interiors.
It changes 1,720 additional pixels per target across Balor's 110 strips. The
other eight characters' images and metadata remain byte-identical.

Fresh formatting, Clippy, all 87 active synthetic tests, both Balor corpus tests,
and the release build passed. The new Ryis regression failed on a missed lip
pixel before the correction and passed afterward. All 4,744 combined variants
passed exact recipe validation. A new MOMI installation into
`tmp/balor-mouth-playtest` verified installed pixels, metadata, scripts, and mod
selection before the launcher was retargeted.

A focused real-game run covered all 21 Balor spring expressions plus one
expression in each other outfit: 26 strips and 130 exact palette pairs. It
checked sprite selection, animation phase, independent character palettes, and
the hidden console. The Ryis screenshot showed the corrected mouth shading;
no script or check errors were logged. The earlier full nine-character and world
matrices and uninstall roundtrip were not repeated for this mask-only change.
The user accepted the corrected mouth appearance and approved committing the
batch. This was a desktop visual spot check; the earlier mouth-detail follow-ups
for other characters remain deferred.

Evidence: `tmp/balor-mouth-test-red.log`, `tmp/balor-mouth-test-green.log`,
`tmp/balor-mouth-final-checks.log`, `tmp/balor-mouth-final-art-verification.json`,
`tmp/balor-mouth-build-report.json`, `tmp/balor-mouth-validation-count.txt`,
`tmp/balor-mouth-install-report.json`, and
`tmp/balor-mouth-live-verification.txt`.
