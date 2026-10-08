# Josephine Spring blinking and seated actions

Following `b974134`, this slice adds eleven Spring strips to Josephine's accepted
38 sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and twelve native West mirrors give 43 review cases.
The user approved the artwork on 2026-10-07. Outputs use `generated/review/` and
`generated/build/`.

Josephine has 49 sources; the 36-character collection has 4,510 sources and
18,040 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only eleven regions and registry entries
are added. Source colors, color groups, target mappings, sets and previous
regions remain unchanged. No runtime code changes. Historical pixel tests
retain fixed expectations; only the central Josephine coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-109-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 38 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 620 component seeds select
1,258 skin pixels across all 31 frames: face, ears, neck, eyelids, moving hands
and exposed ankles. The shades belong exclusively to skin in these sources;
no footwear cutoff is needed. Brown hair, eyelashes and eye detail, pink
earrings, orange shawl, pink skirt, shoes and mouth interiors stay original.
Red `#9E2626` and dark `#410808` mouth pixels remain unchanged during eating.
Profiles contain explicit seeds and source hashes, without new runtime rules.

Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
samples and source grids around the eyelids, mouth, hands and feet. The material
test independently checks every output pixel against a shade oracle, 62 fixed
landmarks and per-frame selected counts, plus transparency and metadata.
Closed-eyelid landmarks distinguish skin from black lashes; moving-hand and
mouth landmarks protect the eating frames.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Blinking has three frames with 0.075/0.125/0.075-second durations. Drinking and
North-facing eating have three one-second frames. East/South eating has five
frames with 0.125/0.15/0.175/0.125/0.6-second durations; sitting has one frame.
All cycles are linear. Seated status and eating/drinking's 240–360-tick
final-frame holds are preserved; West mirrors East.

## Offline review

- [Five-choice sample summary](../../generated/slice-110-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-110-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every source frame and native West mirror, with Vanilla left
and Debug Blue right, full character views and enlarged face, hand and ankle
details. The gallery totals 780,247 bytes (about 762 KiB). Game-derived images
and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 322 regular tests and the
release build passed. The regular run left 392 opt-in tests ignored; 180 local
tests passed separately: the new material test and 179 retained checks,
including three GML tests. The package test failed on the unregistered blinking
strip before registration, then passed afterward.

Four negative controls generated successfully and failed the intended pixel
assertion: missing face, hand, ankle and eyelid selections. All 196 standalone
variants passed strict recipe validation and matched the candidate and combined
build. All 18,040 combined variants validated. The 380 accepted Josephine
PNG/metadata files and 44,785 other-character files stayed byte-identical, as did
previous runtime rows, choices and hotkeys. Earlier regions and all mappings and
sets stayed exact. Historical pixel tests remain unchanged.

The archive's native animator and NPC object passed 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks. Engine
services were simulated and cycles selected directly. Checks cover native
mirroring, palette binding, frame phase, wrapper idempotence, seated status,
cycle counters, completion and Spring portrait synchronization. Natural
scheduling, pause/interaction dispatch and full-engine rendering remain untested.

All twenty gallery pages passed Chromium image/link and overflow checks. Exact
pixel checks covered every frame, mirror, metadata binding, full character view,
enlarged detail and all fifteen summary bindings: 7,766,304 rendered pixels.
The short review/build links resolve to this slice; Chromium also loaded the
review through its short link.

MOMI built and installed the combined package in the isolated
`tmp/slice-110-playtest` lab, including installed-pixel and metadata checks.
Five registry/recipe inputs stayed frozen, and both original archive backups
retained the source hash above. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other opt-in tests
were not rerun. The user approved the offline artwork on 2026-10-07.

Evidence uses `tmp/slice-110-`; author records use
`tmp/josephine-spring-actions-author-`. Fresh exports are in
`extracted/josephine-spring-actions-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-109-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves nineteen Spring strips and 33 each for Summer,
Autumn and Winter. Continue Spring with general actions and shocked reactions
before special activities. Inventory is recorded in `tmp/slice-110-remainder.json`.
Natural gameplay dispatch and live rendering remain pending;
this coverage describes the pinned archive.
