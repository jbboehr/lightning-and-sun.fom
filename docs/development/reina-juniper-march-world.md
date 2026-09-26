# Reina, Juniper and March's first overworld pilots

The user approved the [Wedding completion batch](world-wedding-finish.md),
committed as `68223e0`. This slice starts Spring idle/walk coverage for Reina,
Juniper and March. Each adds six strips, fifteen source frames and five native
West mirrors: eighteen strips and sixty offline review cases together. The user approved this artwork for commit on 2026-09-25.

| Character | Total sources | Spring sprite coverage | Art notes |
| --- | ---: | ---: | --- |
| Reina | 109 | 6/38 | [Reina](reina-world.md) |
| Juniper | 138 | 6/41 | [Juniper](juniper-world.md) |
| March | 187 | 6/53 | [March](march-world.md) |

The combined 36-character trial has 2,918 sources and 11,672 variants. Home,
Page Down and U keep five choices and Vanilla defaults; each character's
portraits and supported overworld sprites share their existing selector.
Other actions/outfits remain original. No runtime Rust or GML behavior changed.

Each new `*-world-trial.json` profile and preset set extends its accepted
portrait definitions, following the Hayden/Ryis/Celine first-world pattern.
The portrait-only definitions remain available and unchanged. The combined
collection switches just these three entries to their new world sets. Character
notes explain each new source shade, mapping role and material boundary.

The [Hayden Shadow audit](hayden-shadow-audit.md) separately confirms that all
29 Shadow-folder strips are ground shadows or transparency, with no skin to
recolor. They retain their native files and rendering behavior.

## Source and metadata

The read-only mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding slice. Source pins remain strict. No earlier hash
was refreshed, and generation does not use `--allow-source-hash-mismatch`.

All eighteen strips use 80×80 frames, Default atlas and Middle/54 origin.
Idle uses the single-frame defaults; walking has four frames at 0.15 seconds.
Native idle/walk cycles support Spring and North/South/East, with South as the
default direction. Native pause, turning and outfit behavior remain unchanged.

The synthetic package test first rejected the unregistered Reina Spring idle
East animation, then passed with all eighteen paths registered. It checks
complete animation properties, frame counts, PNG outputs and the three existing
character/hotkey bindings. Evidence:
`tmp/world-next-pilots-package-{red,checks}.log`,
`tmp/world-next-pilots-inventory.log` and
`tmp/world-next-pilots-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/reina-juniper-march-world-preview/summary.png):
  South-facing walking frame two for each character.
- [Complete Vanilla/Debug Blue review](../../generated/reina-juniper-march-world-preview/index.html):
  all 45 source frames and fifteen native West mirrors, grouped by character,
  with separate five-choice summaries.

Static previews do not establish natural NPC scheduling, automatic outfit
changes or live rendering. Source art, generated variants, previews, packages
and temporary helpers remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 127 active tests and the release
build passed. The normal suite leaves 133 local opt-in tests ignored; nine ran
separately and passed: three new world corpus tests, three retained portrait
corpus tests and three GML runtime tests. Independent material controls exercise
omitted skin and spills into hair, jewelry and jacket folds. Older opt-in tests
and live gameplay were not rerun. Shared evidence:
`tmp/world-next-pilots-{final-checks,retained-checks}.log`; the character notes
link their focused tests and controls.

Every author-exported sidecar matches the independent archive read by asset
path (`tmp/world-next-pilots-metadata-check.log`). All 11,672 combined variants
pass exact recipe validation. All 4,160 accepted Reina/Juniper/March original and
variant PNG/metadata files remain byte-identical, as do all 25,005 files for the
other 33 characters. All 1,736 standalone variants match the combined output.
Previous runtime rows and controls remain unchanged. Evidence:
`tmp/world-next-pilots-comparison.{json,log}`.

The definition audit confirms all 416 earlier region objects, source-color
roles, color groups and preset mapping prefixes are preserved in the new world
definitions. The portrait-only recipes and retained tests are unchanged. The
registry adds exactly eighteen paths, and the collection switches exactly three
preset references. All 36 export reports retain the earlier source entries and
current archive hash. Evidence: `tmp/world-next-pilots-input-audit.log`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, probes
pass 300 frame/direction observations and 120 linear completions across all
five choices. They check West mirroring, palette wrapper idempotence, frame
phase and cycle-counter preservation. These probes do not establish natural
scheduling, automatic outfit dispatch, pause policy or full-engine rendering.
Evidence: `tmp/world-next-pilots-{package-checks,runtime}.log`.

The combined summary passes fifteen source/palette bindings and 139,200 exact
displayed-pixel checks, including raw metadata and complete opaque artwork
bounds. Each character gallery passes exact-pixel, crop, metadata, West-reversal
and Chromium checks. All local links and images load on the combined landing
without horizontal overflow. The four preview directories occupy about
1.28 MiB. Evidence:
`tmp/world-next-pilots-{summary-check,landing-browser}.log`, plus character records.

MOMI installation passed in the fresh isolated `tmp/world-next-pilots-playtest`
lab, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`c56597ed507d69d6553e8c0fd22c691d92c60eebda9fc78438e08f81274e8351`.
The mounted archive and the lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/world-next-pilots-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Continue these pilots with Spring blinking and seated/eating/drinking animations
once the initial material choices are accepted. Remaining Spring inventories
are in `tmp/world-next-pilots-remaining.json`; they also include character-specific
special actions that should be inspected in later batches.
