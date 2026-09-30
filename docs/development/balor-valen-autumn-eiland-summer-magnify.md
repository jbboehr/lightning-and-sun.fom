# Balor and Valen's Autumn sprites and Eiland's Summer magnifying glass

Following the approved batch in `c38deb4`, this slice adds 17 strips: 53 source
frames and 23 native West mirrors, giving 76 review cases. The user approved the
offline artwork on 2026-09-30.

| Character | New strips | Frames + mirrors | Total sources | Current folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 5 | 26 + 12 | 195 | Autumn 22/28 |
| Valen | 6 | 15 + 5 | 172 | Autumn 6/31 |
| Eiland | 6 | 12 + 6 | 162 | Summer 37/41 |

The 36-character collection now has 3,714 sources and 14,856 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change; runtime Rust and GML
code remain unchanged.

## Sources and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 512 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Existing regions, source roles, color groups and
target ramps remain unchanged. New metadata preserves 80×80 frames, the Default
atlas and the native Middle/54 origin.

Balor's Autumn general actions, sleep and kiss use his existing four world skin
shades. Faces, necks and moving hands change while scarf, clothing, eyes and hair
remain original. General actions have seven frames per North/South/East direction
with the native last-frame hold; sleep has one East frame and kiss has four.
East sources also provide the West mirrors.

Valen's Autumn idle/walk sprites use her four world skin shades. Her face, neck
and visible hands change; goggles, coat, shirt, cuffs, trousers, boots, eyes and
hair stay original. The North idle frame and all four North walk frames expose
no skin. Both strips deliberately have empty seed lists and remain byte-identical
in every palette. Tests cover this exception explicitly. Idle frames are single
images; walking uses four frames per direction at 0.15 seconds, with West mirrors.

Eiland's Summer magnifying-glass start/loop/end cycles use his five world skin
shades. Face and moving fingers change around the unchanged rim, lens and handle;
gold trim, pink clothing, eyes and hair remain original. The warm `#BE6D44` rim
shadow is a material color. Both South and East cycles have 3/1/2 start/loop/end
frames, with starts at 0.15/0.125/0.125 seconds and ends at 0.125/0.15. East mirrors
West. Summer needs none of the shared skin-color clothing exclusions from Spring.

Local source inventories, raw metadata, per-frame skin counts and material
landmarks use `tmp/balor-autumn-standard-author-`, `tmp/valen-autumn-world-author-`
and `tmp/eiland-summer-magnify-author-`. Complete fresh corpora are in the
corresponding `extracted/<character>-<batch>-study` directories.

## Offline review

- [Five-choice sample summary](../../generated/balor-valen-autumn-eiland-summer-magnify-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-valen-autumn-eiland-summer-magnify-preview/index.html).

The combined index is the entry point for the whole batch. Its three character
galleries contain every source frame and native West mirror, including Valen's
unchanged North views. Each character also has a five-choice sample sheet.
All game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 208 active tests and the release
build passed. The normal suite leaves 257 local opt-ins ignored. Thirty-nine
local tests passed separately: three new material tests, thirty-three retained
corpus tests and three GML runtime tests. Six deliberate omission/spill controls
generate successfully before failing the intended pixel assertions. Valen's test
also checks empty North masks and exact original PNG bytes in all four targets.
The synthetic package test rejected unregistered Balor Autumn action East before
the registry additions and passed afterward with native metadata and hotkeys intact.

All 14,856 combined variants validate. All 2,116 standalone character variants
pass strict recipe validation and match the combined output. All 5,120 prior
original/variant PNG and metadata files for these characters and all
32,015 files for the other 33 characters remain byte-identical. Previous runtime
rows, choices and hotkeys remain exact. The thirty-three retained tests change
only corpus paths and source totals.

The current archive's native animator and NPC object pass 380 frame/palette
observations, 80 linear completions, 40 last-frame hold checks and 60 complex-phase
transitions with simulated engine services and direct cycle selection. Checks
exercise native mirroring, palette binding, wrapper idempotence, frame phase,
cycle counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain untested.

All 36 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks bind all 76 cases to current artwork, raw metadata and native mirrors;
the full crops contain every opaque pixel. The combined summary checks all fifteen
source/palette bindings. The four preview directories total
1,504,453 bytes (about 1469 KiB).

MOMI installation passed in the fresh isolated
`tmp/bve-autumn-standard-world-summer-magnify-c38deb4-playtest` copy, including required
compilation and installed-pixel/animation-metadata verification. Installed archive:
`39518f6b5f77dd54cc9e58b7e8763b3eec50d9f63721801ca305d17becfb968b`.
The mounted archive and lab's original backup retain the source hash above; all
eleven registry/recipe inputs remained unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall roundtrip,
other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/bve-autumn-standard-world-summer-magnify-`: `evidence.json`,
`final-checks.log`, `focused.log`, `{balor,valen,eiland,gml}-local.log`,
`comparison.{json,log}`, `input-audit.log`, `metadata-check.log`, `runtime.log`,
`{balor,valen,eiland}-final-output.log`, `summary-check.log`,
`landing-browser.log`, `install-report.json`, `frozen-check.log` and
`source-after.sha256`. Character `author-{omission,spill}-red.log` files record
the negative controls. No game-derived assets enter Git.

## Next coverage

Balor's six remaining Autumn strips are seated reading and gem inspection.
Valen can continue with Autumn blinking, sitting, eating and drinking. Eiland's
four remaining Summer strips are axe, pickaxe, trowel and brush animations.
Folder inventories are in `tmp/bve-autumn-standard-world-summer-magnify-remaining.json`.
