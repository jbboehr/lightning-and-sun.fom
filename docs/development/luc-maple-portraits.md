# Luc and Maple portrait batch

This batch adds all 32 Luc and 32 Maple portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn, and Winter. Both characters
have Vanilla, Debug Blue, Hayden, Ryis, and Seridia choices, with Vanilla selected
at launch. F11 cycles Luc; F12 cycles Maple. The shared Rust and GML runtime is
unchanged.

The collection has 1,692 source strips and 6,768 generated variants across
twenty-three characters: 1,675 portraits and Adeline's 17 reviewed world strips.
The earlier characters retain their accepted recipes, including Hemlock's
deferred warm-gray beard and arm-hair contrast against Debug Blue.

Both new characters use the corresponding seasonal atlases, eight strips per
atlas. Every source name matches the native NPC outfit/expression definitions.
Character-specific art notes are in [Luc portraits](luc-portraits.md) and
[Maple portraits](maple-portraits.md).

Luc uses seven skin shades and two blends of the target complexion into the
white glasses rim. His white and gold glasses details, irises, hair, and clothing
stay original; closed-eye shading, the lower lip, and exposed forearms follow
the selected palette. Maple uses nine skin shades, including fine eye, nose,
jaw, and hand shading shared with actual hair, eyes, and dress details, plus a
softened lip highlight for each target.

Before the glasses/lips follow-up below, the author and independent reviewer
inspected all 128 unique frames in full
Source/Blue/Ryis comparisons, enlarged faces and mouths, and seasonal hand and
neck crops. Luc's final corrections cover closed-eye and forearm omissions and
restore clothing spills. Maple's correction adds two cheek/jaw pixels per
frame. The corrected boundaries received a final visual and exact delta check.
All 256 new target strips passed the independent pixel audit, including 66
literal source-art landmarks. Both galleries reconstruct the actual Blue output
exactly and reuse all 64 frames per character.

## Local visual check

The compact comparison is `generated/luc-maple-preview/summary.png`, with
`index.html` beside it. The current bundle is `generated/characters-luc-maple-details`.
Images, packages, extracted assets, and copied game files remain ignored.

The `tmp/play-characters` launcher uses `tmp/luc-maple-detail-playtest`, with separate
saves and state, and mounts the supplied game read-only. F7 opens on Luc;
F4 moves to Maple. F11/F12 cycle their palettes, F5 changes season, and F7 or
Page Up moves through expressions. F9 checks independent selections and
fractional portrait phase. These preview controls are excluded from the player
package.

The optional world-preview controls use Shift+F12 for facing and Shift+F11 for
phase/movement checks. The installed, pinned MMAPI supports these chords and
consumes their triggers so the plain Luc/Maple controls can share the keys.
F1 still spawns/resets the Spring actor and F3 changes its action. The helper
retains the scheduling fix that prevents the manually placed Adeline actor from
resuming town pathfinding. This change is confined to the ignored test helper.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test luc_portraits --test maple_portraits -- --ignored --nocapture'
```

The initial batch checks below predate the glasses/lips follow-up. The
collection test first failed with `Unsupported character: luc`. After
integration, all four character tests passed, covering standalone and combined
builds, the twenty-three-character collection, F11/F12 controls, Vanilla-first
choices, generated pixels, and metadata. The release build and three optional
GML interpreter tests also passed. All previous twenty-one registry entries
are unchanged.

Formatting, Clippy, all 87 active tests, both local corpus tests, and the release
build passed through the pinned Fenix Nix shell. The corpus tests include 103
Luc and 182 Maple literal landmarks; omission and spill failures were observed
before the corresponding recipe corrections.

A source audit matched every PNG, metadata entry, registry path, source hash,
frame size, atlas, profile region, preset arity, and catalog shade role. All
6,768 combined variants passed validation. Luc and Maple's combined variants
match the art-reviewed outputs byte-for-byte. All twenty-one previous character
trees match the accepted Hemlock/Louis bundle, including original PNGs,
recolors, metadata, and reports. The compact summary PNG was inspected.

The normal CLI installer built and published a fresh isolated MOMI archive,
verified its pixels and animation metadata, and reported all twenty-three
characters with the expected controls. The installed archive's SHA-256 matches
its receipt. The supplied source archive remains byte-identical.

The headless game exercised all 64 new sources under all five choices, plus
104 existing-character outfit representatives: 168 sources and 840 distinct
source/preset pairs. Checks covered F11/F12 cycles, seasonal and character wraps,
Vanilla-first startup, independent selections across all twenty-three characters,
sprite identity, retained fractional phase 1.25, and a hidden debug console.
Shift+F12 changed the world actor's facing and Shift+F11 verified its phase and
movement without changing Luc or Maple's palette. There were no script errors,
disabled handlers, or selection leaks. The game exited gracefully with status 0
after SIGTERM. Blue and Ryis screenshots of both new characters were inspected.
The normal desktop launcher was rebuilt, checked, and published as
`tmp/play-characters`; an interactive desktop session remains for user review.

Ignored evidence includes:

- `tmp/luc-maple-art-review-final.md`
- `tmp/luc-maple-final-checks.log` and `tmp/luc-maple-gml-checks.log`
- `tmp/luc-maple-native-portraits.json` and `tmp/luc-maple-integration-audit.json`
- `tmp/luc-maple-all-variants-validation.log`
- `tmp/luc-maple-previous-output-check.log` and `tmp/luc-maple-reviewed-output-check.log`
- `tmp/luc-maple-bundle-inputs.sha256` and `tmp/luc-maple-install-report.json`
- `tmp/luc-maple-live-run.log` and `tmp/luc-maple-live-audit.json`
- `tmp/luc-maple-headless/state/` for game logs and screenshots

## Glasses and lips follow-up

User review identified Luc's original skin color in the pale glasses rim and
Maple's conspicuous pink lip highlight. Luc's two rim blends now use the target
light shade mixed with the original white rim. Maple's lip highlight uses a
muted mauve or rose chosen for each palette. White and gold glasses details,
irises, mouth interiors, tongues, and the earlier skin selections are preserved.

Focused tests first failed on Luc's peach rim at `[142,99]` and Maple's fixed
pink lip at `[153,104]`, then passed after the recipe changes. Exact four-target
comparisons limit the delta to 768 rim pixels per Luc target and 160 lip pixels
per Maple target. Every other pixel and all metadata match the earlier outputs.
Source and corrected closeups were inspected. Both changes are encoded in the
recipes; the Rust and GML runtime remains unchanged.

Fresh formatting, Clippy, all 87 active tests, three local corpus tests, and the
release build passed through the pinned Fenix Nix shell. All 6,768 combined
variants passed validation. The new character outputs match their corrected
standalone builds byte-for-byte, and all twenty-one earlier character trees
still match the accepted Hemlock/Louis bundle. The updated summary was inspected.

The normal CLI installer published a fresh archive in the separate detail
playtest and verified its pixels and metadata. The installed SHA-256 matches
the receipt; the supplied archive remains unchanged. The headless game then
checked all 64 Luc/Maple portraits under all five choices: 320 distinct
source/preset pairs. Seasonal wraps, F11/F12 cycles, independent selections,
sprite identity, fractional portrait phase, and the hidden console passed.
There were no script errors or disabled handlers; the game exited with status 0
after SIGTERM. Blue and Ryis screenshots of both characters were inspected.
The usual desktop launcher was rebuilt and published with the corrected trial.
The user accepted the corrected appearance. The automated game run above is
the recorded runtime verification for this correction.

Follow-up integration evidence is in `tmp/luc-maple-detail-checks.log`,
`tmp/luc-maple-detail-all-variants-validation.log`,
`tmp/luc-maple-detail-previous-output-check.log`,
`tmp/luc-maple-detail-inputs.sha256`, `tmp/luc-maple-detail-install-report.json`,
`tmp/luc-maple-detail-live-run.log`, `tmp/luc-maple-detail-live-audit.json`, and
`tmp/luc-maple-detail-headless/state/`.
