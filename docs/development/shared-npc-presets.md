# Shared NPC portrait presets

This slice continues the sampled-palette plan after desktop acceptance of the
Adeline, Hayden, and Ryis trial. Hayden and Ryis each gain three NPC-derived
palettes in their existing preset set. Each now has five choices, including
Vanilla and Debug Blue, using the same reviewed portrait masks.

| Character | Key | Cycle after Vanilla and Debug Blue |
| --- | --- | --- |
| Adeline | F6 | Hayden, Ryis, Seridia |
| Hayden | F8 | Adeline, Ryis, Seridia |
| Ryis | F10 | Adeline, Hayden, Seridia |

Vanilla remains the session default. The character's own palette is represented
by its untouched Vanilla artwork. The new choices are adaptations of catalog
colors to the destination artwork, not reproductions of another NPC's shading.
The existing [NPC catalog](npc-palettes.md) supplies all target colors; no
interpolated colors are introduced.

## Shading assignments

Each new preset replaces the four approved blue shades with the selected
catalog's four ordered colors. The detailed source colors retain the shading
roles accepted in the blue review:

- Hayden's lighter patches remain in the main-shade band. His fine face shadows
  and mouth details use the second shade; recolored body hair uses the third.
- Ryis's fine face/body shadows use the second shade. His light fingertips use
  the main shade and their darker details use the second.

The source profiles, component seeds, color groups, and Debug Blue recipes are
unchanged. Head hair, clothing, accessories, blush, mouth interiors, and other
unselected pixels retain their original colors. The known Hayden embarrassed
expression cleanup remains deferred. A mask check cannot establish that every
art detail is correct.

Only the two preset definitions and documentation change. The Rust builder,
installer, character registry, and shared GML runtime need no changes.

## Preview and generation

`generated/characters-natural-preview/index.html` shows one compact comparison:
Hayden and Ryis with Vanilla, Debug Blue, and their three new choices. Its
`summary.png` can also be opened directly. Six additional local sheets cover
both frames of seasonal, bathing, and wedding examples; all six and the summary
were visually inspected. These are representative art checks, not an exhaustive
new manual review of every expression.

```sh
target/release/mistria-palette build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-natural-trial
```

Use a fresh output directory when rerunning generation. The bundle contains
385 original strips and 1,540 variants: 572 Adeline, 532 Hayden, and 436 Ryis.
The six new character/palette combinations account for 726 variants. The
standalone Hayden and Ryis `--presets` definitions provide the same choices.

## Verification, 2026-09-07

All 1,540 variants passed exact recipe validation, including dimensions, alpha,
metadata, and excluded pixels. An independent pixel comparison used the
previously approved blue output as the selection boundary, then substituted the
four catalog colors. It checked all 726 new variants and 5,080,146 selected pixel
occurrences. Every output matched, every unselected pixel stayed original, and
Debug Blue matched the approved output exactly.

The focused preset and character tests passed. Fresh formatting, Clippy, all
87 active tests, the three opt-in GML tests, the Hayden and Ryis corpus tests,
and the release build passed. No runtime behavior was changed, so no additional
runtime reliability review was needed for this data slice.

A fresh MOMI installation in `tmp/characters-natural-playtest` verified every
selected original and variant atlas frame, animation metadata, and generated
runtime scripts. Uninstalling a separate copy in `tmp/characters-natural-roundtrip`
restored the source archive byte-for-byte. The previous three-character trial
remains separately available in `tmp/ryis-playtest`.

The real game under Xvfb checked all 368 included portraits in each of their
five choices: 1,840 distinct portrait/palette combinations. Checks covered
independent selections, fractional portrait phase, a closed debugger console,
and Vanilla defaults on startup. Another 24 checks covered world actions and
directions while all three characters' palettes cycled, plus six outfit
transitions with the world preview active. No script errors or disabled palette
callbacks were logged. The isolated run had the same Steam initialization error
and initial room-placement warning recorded in the previous trial.

A separate run exercised every F8 and F10 choice through actual key events,
including return to Vanilla. Both characters' Seridia portraits were visually
checked with the debugger closed. This short run spaced repeated keypresses
apart; the earlier matrix's rapid final screenshot sequence had coalesced F8
events and captured Hayden's preceding palette. The per-portrait five-choice
checks in that matrix used direct callbacks and were unaffected.

The new archive has 12 portrait atlas pages instead of 11; the extra page belongs
to the Summer atlas, which also holds beach portraits. Each page is
4096×4096; the sum across all portrait atlases is 768 MiB at RGBA8, up from
704 MiB. This is texture capacity across all outfits, not measured resident
memory or a claim that every atlas is loaded simultaneously. Desktop performance
with the additional page has not been measured.

Local evidence is retained in `tmp/characters-natural-build-report.json`,
`tmp/characters-natural-*-validation.json`,
`tmp/characters-natural-review-report.log`,
`tmp/characters-natural-final-checks.log`,
`tmp/characters-natural-install-report.json`,
`tmp/characters-natural-uninstall-report.json`,
`tmp/characters-natural-atlases.json`, `tmp/characters-natural-live-run.log`, and
`tmp/characters-natural-live-game.log`. The separate key check is recorded in
`tmp/characters-natural-keys-run.log` and `tmp/characters-natural-keys-game.log`.
All extracted images, generated variants,
comparison sheets, packages, and temporary helpers remain ignored by Git.

## Desktop check

The rebuilt `./tmp/play-characters` launcher uses the new isolated trial.
F7 opens Ryis; F10 cycles his palettes. F4 switches characters, F8 cycles Hayden,
and F6 cycles Adeline. F5 changes outfit; F7 and Page Up move through expressions.
The supplied source game files are mounted read-only, and the trial uses its
own saves and state. The user reviewed the new colors and accepted this
checkpoint. That feedback is a visual spot check; desktop performance remains
unmeasured.
