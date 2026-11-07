# All-character expressive-portrait review

2026-11-06. After judging Celine's revised mouth, eyes and visible nose blush
"much better", the user requested another review sheet of all characters with an
expression likely to expose similar issues. This records qualified feedback,
not complete artwork or approximate coverage acceptance.

The [interactive sheet](../../tmp/native-tone-expression-review/review/index.html),
[static contact-sheet index](../../tmp/native-tone-expression-review/review/sheets.html)
and [portable ZIP](../../tmp/native-tone-expression-review/review.zip) cover all
36 supported characters. Each character has one deliberately expressive Spring
portrait, both native frames and seven choices: **Vanilla → Player 04 → March →
Wiscar → Player 16 → Player 22 → Player 35**. Default frame 2 often exposes an open
mouth. A tone selector makes a larger Vanilla/current-tone comparison, and full
portraits can be enabled for context. All characters are displayed by default.

Expressions prioritize blush, embarrassed reactions, laughing mouths and closed
eyelids. Adeline uses cartoon embarrassed, Hayden laughing blush, Ryis closed-eye
smiling blush, Reina special blush, Juniper wild-laugh blush, Celine closed-eye
blush, March super flustered, Balor special blush, Valen/Eiland happy blush,
Caldarus serious blush and Seridia closed-eye blush. The other 24 characters use
their embarrassed expressions. Their exact asset paths, selection reasons,
frame sizes and crop coordinates are in the packet's
[data file](../../tmp/native-tone-expression-review/review/data.json).

Face crops retain original pixels and use nearest-neighbor enlargement. The
shorter Dell, Luc and Maple portraits use lower crop bounds so their mouths and
chins remain visible. Twelve static PNG contact sheets show six characters and
all seven columns per page: six pages for each native frame, with frame 2 first.

Celine's six additions come from `generated/celine-native-tone-face`, the latest
twenty-three-slot facial correction. Her Vanilla comes from the original export.
All other portraits come from the unchanged `generated/native-tone-build`.
Original PNGs, native metadata and actual candidate report hashes are bound
individually; no palettes, generated source artwork or runtime code change.

## Verification and limits

The [check packet](../../tmp/native-tone-expression-review/checks.json) records:

- All 36 characters, 72 native frames and 504 comparison cases are present.
- All 252 PNG and 36 native-metadata copies match their actual source files.
  The 79 recipe/profile/build/selection inputs match their recorded SHA256 pins.
- All 310 portable entries, ZIP contents and 24 internal links validate.
- CPU Chromium checks pass at desktop widths 1600 and 1280 and phone width 390.
  Each run compares every one of the 504 crops with literal source alpha and
  opaque RGB hashes across both frames, then exercises character/tone filtering
  and full-portrait views. Direct `file:` opening also renders all 36 rows and
  252 face canvases without overflow or special browser flags.
- Louis has a few low-alpha pixels whose RGB can round during browser
  premultiplication. Every alpha and opaque RGB channel is checked in-browser;
  the entire source/candidate PNG bytes, including translucent RGB, are separately
  bound exactly. File-origin canvas readback restrictions are avoided during
  normal local-file viewing; pixel checks use a temporary loopback-only server.
- Internal inspection includes original crops for all 36, static pages 1, 4 and
  6, and browser close-ups of corrected Celine and Luc. Full face details remain
  visible, including the shorter characters. No GPU reservation or game launch
  is needed for this CPU-only artifact.

The ZIP is 6,205,951 bytes, SHA256
`bf08fa62935f5e888367b1d662407be19d0ee60ad2f5744db2d3f901d74a5823`.
The task retains about 18 MiB of portable images, static sheets, checks and
inspection captures. Existing full and Celine review/check/source packets remain
frozen. No new game archive or build is created. The runtime test suite and game
installation are not repeated for this review-only preparation.

The sheet is a targeted screen of one expression and outfit per character,
not exhaustive expression, seasonal, world or live-game acceptance. Real blush,
lips, scars, hair and eye materials can correctly differ from skin; compare each
case with Vanilla. Human feedback determines which remaining features need
bounded follow-up. No new art acceptance, commit or publication is inferred.
