# Stillwell and Taliferro portrait batch

This batch covers all 36 Stillwell and 36 Taliferro main portrait strips in the
supplied build: nine expressions in each of Spring, Summer, Autumn, and Winter.
Stillwell includes `closed_eyes`; Taliferro includes `sly`. Both have Vanilla,
Debug Blue, Hayden, Ryis, and Seridia choices, with Vanilla selected at launch.
F4 cycles Stillwell; F5 cycles Taliferro. The shared Rust and GML runtime is
unchanged.

The combined collection has 1,828 source strips and 7,312 generated variants
across twenty-seven characters: 1,811 portraits and Adeline's 17 reviewed world
strips. Earlier characters keep their accepted recipes and controls.

All 72 new strips use `PortraitsMisc`, including every seasonal folder. The
registry retains their original paths and atlas assignment. Every strip has two
296×180 frames, with a total of 144 unique frames. All source names match the
native NPC outfit and expression definitions.

The original game mount is unavailable in this session. Authoring reads
`tmp/momi-lab/assets.bak.zip`, whose SHA-256 exactly matches the previously
supplied untouched archive:
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The backup remains read-only throughout this workflow. Game files, extracted
images, packages, and review artifacts remain ignored.

Character art notes are in [Stillwell portraits](stillwell-portraits.md) and
[Taliferro portraits](taliferro-portraits.md).

Stillwell uses six source shades. Two noncatalog shades cover fine jaw, lip,
eye and finger contours; these follow the standard deep and shadow roles.
Exposed skin in Winter's fingerless gloves and Summer's arms also recolors.
His actual blue hair, gray eyes, jewelry, gloves and clothing retain their colors.

Taliferro uses eight shades, including four eye and outer-mouth blends. Two
blend the target light/middle and middle/shadow colors; two retain small RGB
offsets from the target middle or light shade. His actual blond hair, eyebrows,
sideburns, jewelry and mouth interiors stay original, including the deep brown
shared with separate skin contours. Original expression blush remains unchanged
for both characters; its contrast is a deferred cosmetic judgment.

## Local visual check

The compact comparison is `generated/stillwell-taliferro-preview/summary.png`,
with `index.html` beside it. The combined bundle is
`generated/characters-stillwell-taliferro-trial`.
The summary shows only the first Spring neutral frame for each character in
all five palette choices: one of 72 unique frames per character. It does not
show the other expressions, speaking frames, or seasonal outfits.

The offline review is
`generated/stillwell-taliferro-preview/blue-review/index.html`, also linked from
the original preview page. The user found the first single-page, five-palette
review too large and requested Vanilla and Debug Blue only for detailed review.
The compact overview still includes all five palettes and is byte-identical.

The detailed review now has eight pages, one per character and season, each with
nine expression sheets. Every expression and both frames remain covered: 144
frames and 288 Vanilla/Blue views. Frames appear side by side, with full
portraits at 2× and face details at 4× using nearest-neighbor scaling. Each page
has previous/next navigation and expression links; individual PNGs open at full
resolution. The old `all-cases/index.html` link forwards to the smaller review.
Keep the `blue-review` directory together when copying it elsewhere.

The user accepted the smaller review format and requested committing this batch.
This does not establish an exhaustive frame-by-frame user review. The temporary
Rust generator is `tmp/stillwell-taliferro-blue-preview.rs`; its local
`coverage.json` records source paths, frame/palette identities, hashes and crop
positions. The coverage check matched all 288 combinations against the registry,
verified every placed pixel against the bundle and found no clipped visible
pixels. Chromium loaded all eight pages and all images, checked their cases
against the manifest, and verified navigation and the old-link redirect.
Evidence is in `tmp/stillwell-taliferro-blue-preview-check.log` and
`tmp/stillwell-taliferro-blue-browser-check.json`. Generated art remains ignored;
no palette or game runtime changed.

Before committing, formatting, Clippy, all 87 active tests, both local portrait
corpus tests and the release build passed again. The fresh check log is
`tmp/stillwell-taliferro-commit-checks.log`. Native-game testing was not rerun.

The isolated trial uses `tmp/stillwell-taliferro-playtest`, with separate saves
and state. The usual `./tmp/play-characters` launcher now targets this batch;
its executable, shell syntax, and match with `tmp/play-stillwell-taliferro`
were checked. It requires the restored read-only game mount to launch.
F7 opens on Stillwell and advances expressions; Page Up goes backward.
Shift+F4 changes character and Shift+F5 changes season. The modifiers distinguish
preview navigation from Stillwell's F4 and Taliferro's F5 palette controls.
F9 checks independent selections and fractional portrait phase.

The pinned MMAPI supports these chords and consumes their triggers before the
bare-key callbacks. Optional world controls retain Shift+F1 for spawning/resetting
Adeline, Shift+F3 for action, Shift+F12 for facing, and Shift+F11 for phase and
movement. The helper keeps the scheduling fix that prevents manually placed
actors from resuming town pathfinding. All preview controls and scheduling
changes are excluded from the player package.

## Verification

The collection regression first failed with `Unsupported character: stillwell`.
After registry integration, all four character tests passed, covering standalone
and combined builds, twenty-seven characters, F4/F5 controls, Vanilla-first
choices, generated pixels, metadata, and both miscellaneous atlas assignments.
All 72 paths also matched the native NPC outfit/expression definitions.

The authors and independent reviewer inspected every actual full Source/Blue/Ryis
frame trio, all 144 enlarged face trios, and eight seasonal body/hand views.
The reviewer also inspected 26 distinct Stillwell and 32 Taliferro mouth-region
crops across all four targets. No concrete omission, spill or inconsistent hair
treatment remained. All 288 target strips and 30,689,280 pixels passed the
independent exact audit, including 71 separately chosen source-art landmarks.
Both final reviewed variant trees match the approved candidates byte-for-byte.

Stillwell selects and changes 70,118 pixels per target, with no matching-color
exclusions. Taliferro selects and changes 128,504 while preserving 8,116 other
matching-color pixels. Masks agree across all four presets; no mapping is an
identity mapping. Each gallery reconstructs all 72 character frames and their
metadata exactly. Stillwell's local corpus test checks 43 literal landmarks in
every palette; Taliferro's checks 52 skin/blend and 48 protected-detail points.
Their omitted-shade and overbroad-selection controls failed as intended.

Three existing GML interpreter tests passed. A focused interpreter check of the
actual pinned MMAPI hotkey module also passed 20 simulated input frames across
both registration orders: F4/F5 call only the palette callbacks, while shifted
keys call only preview navigation. Removing the Shift modifier made that check
fail. This verifies callback routing with a simulated keyboard boundary; it does
not replace the pending check in the native game.

Formatting, Clippy, all 87 active tests, both new local portrait corpus tests,
and the release build passed through the pinned Fenix Nix shell. The first full
test run hit `Text file busy (os error 26)` while an existing installer test
launched its temporary runner. That test passed in isolation, and the complete
checks passed on rerun without changing the installer or its test. The initial
failure is preserved in `tmp/stillwell-taliferro-final-checks-first-attempt.log`;
its cause was not reproduced.

The source audit matched every new registry path, PNG, metadata entry, source
hash, frame size, atlas, profile region, preset arity, and standard or custom
shade rule. All 7,312 combined variants passed validation. Both new character
outputs match their art-reviewed standalone builds byte-for-byte. All
twenty-five earlier character trees match the accepted Merri/Terithia bundle,
including original PNGs, recolors, metadata, and reports. Their registry rows
are also unchanged. The compact summary was visually inspected.

The normal CLI installer built and published a fresh isolated MOMI archive,
verified all installed pixels and animation metadata, and reported twenty-seven
characters with the expected controls. The archive SHA-256 matches both the
installer report and its receipt; the original backup remains byte-identical.
The installed MMAPI hotkey module matches the bytes exercised by the focused
interpreter routing check.

In-game rendering and the new preview controls have not been exercised in the
native game for this batch because the executable mount is unavailable. The
local headless script and desktop launcher are prepared for that check when
the read-only game mount is restored. The user confirmed that their visual
review was limited to the summary images and accepted those overall,
noting slightly off eyebrow edges on Taliferro, especially with Ryis colors.
A fresh enlarged comparison shows the preserved warm brow colors contrasting
with the recolored skin. This remains a minor cosmetic polish item; the masks
and presets are unchanged. This acceptance does not establish a native-game
test or an exhaustive review of every frame.

Local evidence remains ignored: `tmp/stillwell-taliferro-art-review-final.md`,
`tmp/stillwell-taliferro-final-checks.log`,
`tmp/stillwell-taliferro-all-variants-validation.log`,
`tmp/stillwell-taliferro-integration-audit.json`,
`tmp/stillwell-taliferro-previous-output-check.log`, and
`tmp/stillwell-taliferro-hotkey-check.gml`. Installation evidence is in
`tmp/stillwell-taliferro-install-report.json`,
`tmp/stillwell-taliferro-installed-archives.sha256`, and
`tmp/stillwell-taliferro-installed-hotkeys.log`.
