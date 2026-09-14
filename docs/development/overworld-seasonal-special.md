# Completing Adeline's seasonal overworld sprites

This adds the 30 remaining reading, working, thinking, and fainting strips in
Summer, Autumn, and Winter. Together with the
[writing pass](overworld-seasonal-writing.md), all 156 strips in the four seasonal
sprite folders are included: 42 Spring and 38 each in Summer, Autumn, and Winter.
The existing `adeline-world-actions-trial.json` set contains 282 sources
(126 portraits and 156 overworld strips), producing 1,128 variants.

F6 keeps supported portraits and world sprites on the same Vanilla, Debug Blue,
Hayden, Ryis, or Seridia choice. The combined 36-character package has 2,357
sources and 9,428 variants. Shared GML runtime code is unchanged. The Rust
exporter's selection limit increases from 256 to 512 so it can export Adeline's
282 sources together; explicit paths and duplicate rejection remain required.

## Sources and masks

The source is the read-only `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New files are under `assets/animations/NPCs/Adeline/Sprites/{Summer,Autumn,Winter}/`,
named `spr_npc_adeline_specialanimation_{summer,autumn,winter}_` followed by
`read_sit_{start,loop,end}_south`, `work_sit_{start,loop,end}_south`,
`think_{start,loop,end}_south`, or `faint_south`, with `.png` extensions.

Each outfit adds ten strips and 31 frame occurrences. The full batch contains
93 occurrences and 81 distinct frames, all 80×80 on the Default atlas with
Middle/54 origin. These animations face South only; no mirrored directions are
added. The native metadata is preserved:

| Sequence | Start / loop / end frames | Duration |
| --- | --- | --- |
| Reading | 3 / 4 / 3 | Start/end `0.1`; loop `[3.0,0.1,3.0,0.1]` |
| Working | 1 / 7 / 2 | Start uses the engine default; loop `[0.5,0.25,0.25,0.25,0.25,0.25,0.25]`; end `[0.25,0.175]` |
| Thinking | 2 / 1 / 1 | Start `0.1`; single-frame loop/end use the engine default |
| Fainting | 7 in one strip | `[0.2,0.1,0.15,0.8,0.1,0.1,1.6]` |

Source inventory: `tmp/adeline-seasonal-special-metadata.json`. Comparing the
original ZIP's seasonal directory listing with the profile confirms complete
coverage of all 156 seasonal strips.

The new masks select 4,067 skin pixels per target while preserving 438 matching
`C47054` material pixels. Every frame has 28–62 selected skin pixels. Independent
inspection covers all 81 distinct frames in Vanilla and all four targets, with
42 literal source/selection landmarks and 2,380,800 exact target-pixel checks.
Book pages, cuffs, collars, tears and mouth interiors retain their original
colors. Evidence: `tmp/adeline-seasonal-special-art-review.md`.

The profile appends 470 seeds, for 6,131 in total, preserving all 252 earlier
region objects and both color groups. Its frozen SHA-256 is
`c94e1c38a4ba93c7e0a2fb594e958b9f051a93c08df3c8b05ea77662367fc4b9`.
The complete Adeline set changes 440,444 pixels per target. Evidence:
`generated/adeline-seasonal-special-author/final-audit.json` and
`tmp/adeline-seasonal-special-frozen-inputs.sha256`.

## Offline review

- [Five-palette summary](../../generated/adeline-seasonal-special-preview/summary.png).
- [Every-frame Vanilla/Debug Blue review](../../generated/adeline-seasonal-special-preview/blue-review/index.html).

The 155 KB summary shows reading loop frame 2, working loop frame 4, thinking
loop frame 1 and fainting frame 7 for all three outfits and five choices: 60
samples at 4× enlargement. The full review includes all 93 South-facing frames
and 186 Vanilla/Blue views on 18 pages, at most eight cases per page, at 10×.
The complete preview directory is 2.27 MB.

Fresh crop bounds `[25,23,30,34]` contain every visible source pixel, including the final
fainting pose. Exact checks passed for 18,972,000 review pixels and 979,200 summary
pixels, source hashes, archived metadata, palette bindings and sequence order.
Chromium checked every page, case, image and link. Evidence:
`tmp/adeline-seasonal-special-preview-pixel-check.log` and
`tmp/adeline-seasonal-special-preview-browser-check.json`. Static images do not
reproduce native animation timing. The user accepted this offline art review.

## Verification

The export regression first failed at the previous 256-strip limit, then passed
with 256, 282 and 512 strips while preserving PNG and metadata bytes and the
source archive. Empty, duplicate and 513-strip requests are rejected without
creating output. Evidence: `tmp/adeline-seasonal-special-export-{red,green}.log`.

The package fixture first rejected the unregistered Autumn fainting strip,
then passed after all 30 exact paths were added. Native timing and offsets are
preserved; invented directions and unreviewed outfits remain rejected. Evidence:
`tmp/adeline-seasonal-special-package-{red,green}.log`.

The material-boundary test retains all 128 earlier landmarks and adds 20 for this
batch. Removing the isolated Summer working-hand seed `[514,46]` from an ignored
candidate copy fails the independent finger assertion at `[515,48]` in Debug
Blue, as expected. Evidence: `tmp/adeline-seasonal-special-missing-hand-red.log`.

Formatting, Clippy, all 87 active tests, the release build, three local world
corpus tests and three Fabricator GML tests passed. Together the world corpus
covers 156 strips and 484 frame occurrences. The material-boundary test covers
139 of those strips and 438 frames in all four targets, checking alpha, metadata
and identical mask selection. Evidence:
`tmp/adeline-seasonal-special-final-checks.log` and
`tmp/adeline-seasonal-special-local-checks.log`.

The combined build preserves 2,520 previous Adeline original/variant PNG and
metadata files and all 20,925 files for the other 35 characters. Every one of the
1,128 Adeline variants matches the reviewed author output, and previous runtime
table rows, choices and controls are unchanged. Evidence:
`tmp/adeline-seasonal-special-build-report.json` and
`tmp/adeline-seasonal-special-comparison.log`. All 9,428 combined variants passed
exact validation in `tmp/adeline-seasonal-special-all-variants.log`.

The focused native probe passed 93 South/frame cases, 465 palette observations,
180 complex start/loop/end transitions at actual last frames and 30 fainting
completions with and without holding the last frame. It uses the original
animator, pack factories and NPC `animate` method with the shipped wrapper and
actual 282-row table. Seated flags, fractional/paused phase, source/pack state,
portrait synchronization and Hayden's independent selection are preserved.
Engine services are simulated, metadata durations use relative units, and
seasonal packs and loop requests are selected directly. Full NPC state machines,
natural scheduling, automatic outfit dispatch and speaking/background pause
policy are outside the probe. Evidence:
`tmp/adeline-seasonal-special-runtime-root.log` and
`tmp/adeline-seasonal-special-runtime-inputs.json`.

A fresh MOMI installation in `tmp/adeline-seasonal-special-playtest` passed
installed pixel, metadata and table validation. Installed archive SHA-256:
`44572c02984f0391e22c346296e5a7aa42843a56537348a556f6eaaab044d87c`.
The source archive and retained `previous.zip` still match the pristine hash.
Evidence: `tmp/adeline-seasonal-special-install-report.json` and
`tmp/adeline-seasonal-special-source-after.sha256`. Live gameplay and an uninstall
roundtrip were not rerun. The isolated package has no preview helper, and the
desktop launcher remains on its earlier Zorel copy.

## Remaining coverage

Other overworld outfits and other characters' overworld sprites remain original.
Generated artwork, extracted source files, packages and isolated installation
copies remain ignored by Git.
