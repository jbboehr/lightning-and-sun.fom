# Landen and Nora portrait batch

This batch adds all 32 main portrait strips for Landen and all 32 for Nora in
the supplied game archive. Each character has eight expressions in each of
Spring, Summer, Autumn, and Winter. Every strip has two 296×180 frames; there
are 128 source frames across both characters. Neither source corpus includes
beach, bath, or wedding portraits.

Two authors own separate character profiles and tests, with an independent
art reviewer checking the rendered results. The individual notes describe
[Landen's masks](landen-portraits.md) and [Nora's masks](nora-portraits.md).
The user receives one compact comparison and one combined game trial.

## Integration

Each standalone set offers Debug Blue and adapted Hayden, Ryis, and Seridia
palettes. Vanilla remains the first choice at every launch. **L** cycles Landen
and **N** cycles Nora. The supplied `Settings.gml` default bindings and direct
keyboard handlers do not assign these keys; custom player bindings can differ.
These remain temporary controls until a character-and-palette menu is added.

The registry includes exactly the PNG paths present in each source portrait
directory. Their metadata places eight strips per character in each of
`PortraitsSpring`, `PortraitsSummer`, `PortraitsAutumn`, and `PortraitsWinter`.
The source NPC definitions recognize every outfit/expression pair directly.
No new native outfit translation or shared Rust/GML hook is needed.

The combined collection now includes 13 characters and 1,364 source strips:
1,347 portraits plus Adeline's 17 world animations. Four generated palettes per
strip produce 5,456 variants. Landen and Nora's overworld sprites are outside
this portrait slice. Previously deferred Hayden, Reina, and Juniper mouth
details remain separate follow-ups.

## Local trial

The later [Nora lip correction](nora-portraits.md#lip-color-correction) updates
the current launcher to `tmp/nora-lips-playtest`, starting on Nora. Its compact
comparison is `generated/nora-lips-preview/index.html`; the current two-character
summary is `generated/landen-nora-lips-preview/summary.png`. The original trial
and verification below remain as the batch baseline.

`tmp/play-characters` uses the fresh `tmp/landen-nora-playtest` archive and its
own save/state directory. The supplied game directory is mounted read-only.
F7 opens the trial on Landen; F4 moves to Nora. L and N cycle their respective
palettes. F5 cycles the four seasons; F7 and Page Up move through expressions.
F9 checks independent selections and portrait animation phase.

The compact comparison is `generated/landen-nora-preview/index.html`, with
`summary.png` beside it. The combined bundle is
`generated/characters-landen-nora-trial`. All game-derived images, packages,
trial archives, and temporary review helpers stay ignored and local.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when the source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test landen_portraits --test nora_portraits -- --ignored --nocapture'
```

The synthetic collection test first failed with `Unsupported character: landen`.
After registry integration it passed for Landen alone, Nora alone, and the full
collection, checking controls, Vanilla-first choices, generated pixels, and the
existing characters' differing preset counts.

Formatting, Clippy, all 87 active tests, the Landen/Nora and Eiland/Olric local
corpus tests, the three local GML interpreter tests, and the release build passed.
The Eiland/Olric checks retain the eye and collar corrections accepted before
this batch. All 5,456 combined variants passed exact recipe validation. The
previous eleven characters' complete output trees, including metadata and
reports, matched `generated/characters-olric-eyes-trial` byte-for-byte.

The independent source audit checked PNG/metadata inventories, original atlas
names, frame dimensions/counts, source hashes, profile coverage, unique registry
paths, preset arity, standalone Debug Blue equivalence, and NPC catalog shade
roles. Native NPC definitions recognize all 64 added outfit/expression pairs.

The art audit checks the same selected pixels across all four targets. Landen
selects 164,968 pixels; Ryis changes only 147,464 because 17,504 selected pixels
already have their target color. Nora changes all 89,160 selected pixels in
every target. A smaller changed-pixel count for Landen/Ryis therefore does not
indicate a mask omission.

Both authors and the independent reviewer inspected all 128 unique source
frames with actual Blue/Ryis outputs, including enlarged eyes, brows, mouths,
and neck gaps. The independent four-target audit covered all 256 new variants
and 101 additional literal source landmarks. The final visual verdict was
PASS with no outstanding concrete correction.

A fresh MOMI installation in `tmp/landen-nora-playtest` verified selected
Vanilla/variant atlas frames, metadata, and generated scripts. The real game
then checked all 64 new portrait strips across all five choices (320 distinct
source/preset pairs), actual L/N key cycles, four-season wraps, independent
character selections, fractional portrait phase, and a hidden debug console.
Another 64 distinct portraits checked one expression per available outfit for
each of the previous eleven characters. The final log audit confirmed every
expected new source/preset pair and recorded 128 distinct sources and 640
source/preset pairs in total, with no script errors or
disabled callbacks. Landen's Blue and Nora's Ryis screenshots were inspected.
The harness terminated the game after its final PASS marker; its shell returned
143 during cleanup, and the game log recorded a graceful exit.

The usual `tmp/play-characters` launcher was rebuilt through Nix, verified as
executable, and checked with `bash -n`. Its desktop Wayland session awaits the
user's visual acceptance; the automated game run used Xvfb and separate state.
This batch did not rerun uninstall or the full Adeline world-action game matrix.
The supplied archive remained unchanged at SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

Local evidence includes `tmp/landen-nora-final-checks.log`,
`tmp/landen-nora-gml-checks.log`, `tmp/landen-nora-integration-audit.json`,
`tmp/landen-nora-native-portraits.json`,
`tmp/landen-nora-all-variants-validation.log`,
`tmp/landen-nora-previous-output-check.log`,
`tmp/landen-nora-art-review-final.md`, `tmp/landen-nora-install-report.json`,
`tmp/landen-nora-live-run.log`, and `tmp/landen-nora-live-audit.json`.

The user accepted the batch after the Nora lip correction on 2026-09-08 and
approved committing it. This is a prototype visual checkpoint; further cosmetic
polish remains optional. The pre-commit formatting, Clippy, full active suite,
Landen/Nora corpus tests, and release-build rerun are recorded in
`tmp/landen-nora-commit-checks.log`.
