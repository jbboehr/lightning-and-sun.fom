# Adeline seasonal everyday actions

This adds 33 blink, sit, eat, and drink strips from Adeline's Summer, Autumn,
and Winter overworld outfits. They contain 93 source frame occurrences and
69 distinct images. Together with the [seasonal idle/walk slice](overworld-seasonal-idle-walk.md),
the existing `adeline-world-actions-trial.json` set covers 219 sources and
produces 876 variants: all 126 portraits and 93 overworld strips. F6 keeps both
renderers on the same choice: Vanilla, Debug Blue, Hayden, Ryis, or Seridia.

The combined 36-character collection contains 2,294 sources: 2,201 portrait
strips and 93 Adeline overworld strips, producing 9,176 variants. Idle, walking,
blinking, sitting, eating, and drinking now have masks in all four seasonal
outfits. Shared Rust and GML runtime code is unchanged.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New paths use
`assets/animations/NPCs/Adeline/Sprites/{Summer,Autumn,Winter}/spr_npc_adeline_{summer,autumn,winter}_{blink,sit,eat,drink}_{north,south,east}.png`.
Blink has South/East source strips only; the other actions have North/South/East.
The game mirrors East artwork for West. No North blink or separate West strip
is registered.

Each outfit adds 11 strips and 31 frames. All source sidecars specify 80×80
frames on the Default atlas with Middle/54 origin. Sit omits frame count and
duration. Blink has three frames with durations `[0.075,0.125,0.075]`. Drink and
North eat have three frames at duration `1.0`; South/East eat have five frames
with durations `[0.125,0.15,0.175,0.125,0.6]`. Source metadata is preserved.
The native NPC data marks sit, eat, and drink as seated, with randomized
240–360 final-frame holds for eat/drink. The original animator owns those holds,
facing, progression and movement.

The profile appends 396 seeds, selecting 3,083 additional pixels per target.
It preserves 355 matching material pixels, including clothing trim. The existing
portrait/world color groups and all preceding 186 masks are unchanged. Every new
frame has visible selected skin, ranging from three pixels on a rear-facing hand
to 60 pixels. Mouth interiors, eye lines, cups, food and clothing remain original.
The author and an independent reviewer inspected all 69 distinct frames in all
four target palettes; 66 independently chosen skin/material landmarks passed.

Frozen profile SHA-256:
`533badc5f4071bbceea9719b44f751446863f5a2ad110d7ce8766ca70bc7834d`.
Audit evidence is in `generated/adeline-seasonal-actions-author/final-audit.json`
and `tmp/adeline-seasonal-actions-art-review-verification.json`.

## Offline review

- [Five-palette summary](../../generated/adeline-seasonal-actions-preview/summary.png):
  one representative row per outfit.
- [Complete Vanilla/Debug Blue review](../../generated/adeline-seasonal-actions-preview/blue-review/index.html):
  all 93 source frames and 36 mirrored West views, for 129 cases and 258 views.
  Twenty-four small pages are grouped by outfit, action and direction, with at
  most eight cases per page.

The summary uses the second Drink South frame in each outfit at 6×. Detailed
pages use 10× nearest-neighbor enlargement with bounds checked against the full
source artwork. Every placed pixel, all 15 summary samples and the West reversals
match the final combined bundle; no visible artwork is clipped. Browser checks
passed on all 24 pages and 129 cases, with all images decoded and links resolving.
Evidence: `tmp/adeline-seasonal-actions-preview-pixel-check.log` and
`tmp/adeline-seasonal-actions-preview-browser-check.json`.

## Verification

Formatting, Clippy, all 87 active tests, the release build, three local world
corpus tests and three Fabricator GML tests passed. The world corpus now covers
93 strips and 265 source frames. The material-boundary test includes 20 new
literal skin/material landmarks, retaining the previous 68 and checking all
four target palettes, alpha, metadata and identical mask selection.

The packaging regression first rejected the unregistered `autumn_blink_east`,
then passed after the 33 exact registry entries were added. It preserves native
frame timing and origin metadata and rejects invented North blink/West strips
and the still-unreviewed Summer general action. A candidate mask with the Summer
drink-East fingertip seed `[120,44]` removed fails its independent landmark;
the final profile passes. Evidence:
`tmp/adeline-seasonal-actions-package-{red,green}.log`,
`tmp/adeline-seasonal-actions-missing-finger-red.log`, and
`tmp/adeline-seasonal-actions-checks.log`.

The combined build preserves all 1,860 previous Adeline original/variant PNG and
metadata files and all 20,925 files for the other 35 characters. All 876 Adeline
variants match the reviewed author outputs, and the generated table retains
prior rows, choices and controls. See `tmp/adeline-seasonal-actions-comparison.log`
and `tmp/adeline-seasonal-actions-build-report.json`. Input hashes are recorded
in `tmp/adeline-seasonal-actions-frozen-inputs.sha256`.

The focused Fabricator probe passed with the original native animator, NPC
`animate` method, palette wrapper and final 219-row Adeline table. It covers all
129 new direction/frame cases, 60 outfit changes, 60 loop boundaries, 30 native
240–360 hold checks and 45 configured blink-to-idle returns. Checks preserve
seated flags, West flips, fractional/paused phase, portrait synchronization and
Hayden's independent choice. Engine lookups are simulated; a small NPC adapter
routes reset calls to the idle collection. Automatic blink triggering, the full
NPC state machine, external props and real-time scheduling are outside this
probe. Evidence: `tmp/adeline-seasonal-actions-runtime-root.log` and
`tmp/adeline-seasonal-actions-runtime-inputs.json`.

All 9,176 generated variants passed exact validation
(`tmp/adeline-seasonal-actions-all-variants.log`). A fresh MOMI installation in
`tmp/adeline-seasonal-actions-playtest` verified installed pixels, animation
metadata and the generated table. Installed archive SHA-256:
`0dc2c3d4dde73c03c811ff9566dfaa8944a55f2ff531e496c91d8f06d96514a9`.
The original source backup and retained `previous.zip` both match the pristine
source hash. Evidence: `tmp/adeline-seasonal-actions-install-report.json` and
`tmp/adeline-seasonal-actions-source-after.sha256`.

Live gameplay, natural outfit/action transitions and an uninstall roundtrip
were not exercised. The isolated package has no preview helper; the desktop
launcher remains on its earlier Zorel copy. All images, archives, helpers and
local game data stay ignored by Git.

## Remaining coverage

Adeline's other overworld actions/outfits and other characters' overworld sprites
remain original. The complete Spring set and everyday actions in the other three
seasons now have masks. The user accepted the offline art review; native gameplay
timing and natural scheduling remain unverified.
