# Complete Celine's Winter sprites

The user approved the [Winter general-action batch](world-winter-general.md),
committed as `475e780`. This slice adds Celine's eleven remaining Winter strips:
seated and standing reading start/loop/end, sweeping start/loop/end, watering
and harvesting. Their 51 source frames and fourteen native West mirrors give
65 offline review cases. The user approved the artwork for commit on 2026-09-25.

Celine now has 361 sources, including all 33 Winter strips. The combined
36-character trial has 2,813 sources and 11,252 variants. Insert retains her
five choices and Vanilla default. No runtime Rust or GML behavior changed.
Material decisions and focused checks are in [Celine's art notes](celine-winter-finish.md).

## Source and native metadata

The read-only mounted archive retains SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding archive. New source pins are strict; no previous hash
was refreshed and generation does not use `--allow-source-hash-mismatch`.

Independent archive metadata and the author export agree by asset path. All
eleven strips keep 80×80 frames, Default atlas and Middle/54 origin. Reading
start/end have three frames at 0.1 seconds; both four-frame loops use
`[3,0.1,3,0.1]`. Sweeping starts/ends with single-frame defaults and has a
fifteen-frame loop with its original timing. Harvest retains ten frames and
water four. The native Winter cycles use complex start/loop/end sequences for
both reading poses and sweeping, with seated reading marked as seated. Harvest
uses the regular Winter outfit; its Autumn counterpart is in the garden outfit.

The synthetic package regression first failed on unregistered seated-reading
end, then passed after registering all eleven paths. It checks complete native
animation properties, frame counts, PNG outputs and the existing Insert binding.
Evidence: `tmp/world-winter-finish-package-{red,checks}.log`,
`tmp/world-winter-finish-{inventory,metadata-check}.log` and
`tmp/world-winter-finish-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/celine-winter-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/celine-winter-finish-preview/blue-review/index.html):
  all 65 frame/direction cases, paginated for review without launching the game.

Static previews do not establish natural NPC scheduling, automatic outfit
changes, separately drawn props or live rendering. Game-derived files, previews,
packages and temporary helpers remain ignored.

## Verification and installation

Formatting, Clippy with warnings denied, all 119 active tests and the release
build passed. The normal suite leaves 115 local opt-in tests ignored; five were
run separately and passed: the new Celine corpus test, the retained Celine world
corpus test and the three GML runtime tests. The material regression observed
the missing watering contour before correction and rejected a deliberate
garment spill. Older opt-in tests and live gameplay were not rerun. Shared logs:
`tmp/world-winter-finish-{final-checks,retained-celine,gml-checks}.log`;
character-specific evidence is linked in the art notes.

All 11,252 combined variants passed exact recipe validation. All 3,500 accepted
Celine PNG and metadata files and all 24,695 files belonging to the other 35
characters remain byte-identical. Celine's 1,444 standalone variants match the
combined output. Existing runtime rows, controls, region objects and color maps
are unchanged; the 21 retained Celine test edits only update corpus paths and
totals. Evidence: `tmp/world-winter-finish-{comparison.json,input-audit.log}`.

The native animator and NPC object used by the probes match the current archive
byte for byte. With simulated engine services and direct cycle selection, they
passed 325 frame/direction observations, twenty linear completions and sixty
complex reading/sweeping transitions across all five choices. Checks include
West mirroring, palette wrapper idempotence, frame phase and loop-counter
preservation. These probes do not establish natural NPC scheduling, speaking
interruptions, automatic outfit dispatch or full-engine state transitions.
Evidence: `tmp/world-winter-finish-{native-source,runtime}.log`.

Exact-pixel summary/gallery checks, native West reversal and Chromium
navigation/image checks passed, as detailed in the art notes. The summary
samples all five action families; the full review includes every frame and
mirror, including reading and sweeping start/end poses.

MOMI v0.16.4 installation passed in the isolated
`tmp/world-winter-finish-playtest` lab, including strict lints, required
compilation, installed pixels and animation metadata verification. Installed
archive SHA-256:
`fc69e29efa4b5b391090ff9427ad96c25c5b561aab7fb9d45abe6d6903792175`.
The mounted source and lab's `previous.zip` both retain the source hash above.
All five frozen recipe/registry inputs remained unchanged through generation and
installation. Evidence:
`tmp/world-winter-finish-{install-report.json,install.log,source-after.sha256,frozen-check.log}`.
No preview helper was installed and the desktop launcher was not changed.

## Coverage milestone and proposed next slice

The current archive inventory confirms complete Spring, Summer, Autumn and
Winter sprite-folder coverage for Hayden, Ryis and Celine, including the riding
and garden animations within those folders. Evidence:
`tmp/world-winter-finish-coverage.{json,log}`.

Each character still has fourteen Beach and fifteen Wedding strips. A proposed
next slice is a small Beach idle/walk pilot for the same three characters,
following the established per-character batching workflow. Hayden also has 29
Shadow strips that need separate inspection before deciding how to handle their
effects. No additional outfit work is included in this slice.
