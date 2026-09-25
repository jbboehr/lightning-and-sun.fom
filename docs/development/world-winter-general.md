# Celine's Winter general actions, sleep and kiss

The user approved the [Winter special batch](world-winter-special.md), committed
as `1bae814`. This slice adds five Celine strips: general actions North, South
and East, sleep East and kiss East. Their 26 source frames and twelve native
West mirrors give 38 offline review cases. The user approved the artwork for
commit on 2026-09-25.

Celine now has 350 sources, including 22 of her 33 Winter strips. The combined
36-character trial has 2,802 sources and 11,208 variants. Hayden and Ryis retain
complete Winter folders. Insert keeps Celine's five choices and Vanilla default;
no runtime Rust or GML behavior changed. Material decisions and focused checks
are in [Celine's art notes](celine-winter-general.md).

## Source and native metadata

The read-only mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding archive. New source pins are strict; no previous hash
was refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and the author export agree. All five strips keep
80×80 frames, Default atlas and Middle/54 origin. General actions have seven
frames with durations `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss has four with
`[0.15,0.15,0.8,0.15]`; sleep uses single-frame defaults. The current native
cycles support Winter, including the general action's random final-frame hold
of `[240,360]`.

The synthetic package regression first failed on the unregistered Winter action
East path, then passed after registering the five paths. It checks complete
animation properties, frame counts, PNG outputs and the existing Insert binding.
Evidence: `tmp/world-winter-general-package-{red,checks}.log`,
`tmp/world-winter-general-inventory.log` and
`tmp/world-winter-general-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/celine-winter-general-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/celine-winter-general-preview/blue-review/index.html):
  all 38 frame/direction cases, paginated for review without launching the game.

Static previews do not establish natural NPC scheduling, automatic outfit
changes, separately drawn props or live rendering. Game-derived files, previews,
packages and temporary helpers remain ignored.

## Verification and installation

Formatting, Clippy with warnings denied, all 118 active tests and the release
build passed. The normal suite leaves 114 local opt-in tests ignored; five were
run separately and passed: the new Celine corpus test, the retained Celine world
corpus test and the three GML runtime tests. The new material test also rejected
deliberately omitted cheek skin and a deliberately selected boot component.
Older opt-in tests and live gameplay were not rerun. Shared logs are
`tmp/world-winter-general-{final-checks,retained-celine,gml-checks}.log`;
character-specific evidence is linked in the art notes.

Every one of the combined package's 11,208 variants passed exact recipe
validation. All 3,450 accepted Celine PNG and metadata files and all 24,695 files
belonging to the other 35 characters remain byte-identical. Celine's 1,400
standalone variants match the combined output. Existing runtime rows, controls,
region objects and color maps are unchanged; the twenty retained Celine test
edits only update corpus paths and totals. Evidence:
`tmp/world-winter-general-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the mounted archive
byte for byte. With simulated engine services and direct cycle selection, they
passed 190 frame/direction observations, forty linear completions and forty
final-frame hold checks across all five choices. Checks include West mirroring,
palette wrapper idempotence, frame phase and loop-counter preservation. These
probes do not establish natural NPC schedules, speaking interruptions, automatic
outfit dispatch or full-engine state transitions. Evidence:
`tmp/world-winter-general-{native-source,runtime}.log`.

The exact-pixel summary/gallery checks, native West reversal and Chromium
navigation/image checks passed, as detailed in the art notes. The summary has
one sample from each new strip; the full review includes every frame and mirror.

MOMI v0.16.4 installation passed in the isolated
`tmp/world-winter-general-playtest` lab, including strict lints, required
compilation, installed pixels and animation metadata verification. Installed
archive SHA-256:
`d581447462f3c234c58118cc46931d27cb2e51eab79b54aeeaa633b62fc4eb9b`.
The mounted source and lab's `previous.zip` both retain the source hash above.
All five frozen recipe/registry inputs remained unchanged through generation and
installation. Evidence:
`tmp/world-winter-general-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Next coverage

Finish Celine's eleven Winter special strips: seated and standing reading
start/loop/end, sweeping start/loop/end, watering and harvesting. That completes
her Winter folder. Folder evidence is `tmp/world-winter-general-remaining.json`.
