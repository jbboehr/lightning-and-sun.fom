# Darren, Linnet, Wiscar, and Wynne portraits

This batch adds eight main portrait strips each for Darren, Linnet, Wiscar, and
Wynne: embarrassed, happy, mad, neutral, sad, think, ugh, and wink. All are Spring
portraits. Each 592×180 strip has two 296×180 frames. The 32 paths and their
`PortraitsMisc` atlas assignments match the original archive and native tables.

These four are defined under `assets/fiddle/cameos/`, rather than the ordinary
NPC tables. Their PNGs are directly under each character's `Portraits/` folder.
The shipped palette runtime already follows textbox portrait sprites and only
requires an NPC ID for world animations; no shared runtime changes are needed.

The combined set now has 34 characters and 1,956 source animations: 1,939 portrait
strips plus 17 Adeline world animations, producing 7,824 recolored variants.
The new controls are D for Darren, C for Linnet, W for Wiscar, and Q for Wynne.
Each starts on Vanilla, followed by Debug Blue, Hayden, Ryis, and Seridia.

The original archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The mounted game folder was empty. Original exports, recolored images, local
review pages, helpers, and packages remain ignored; only transformation data,
registry entries, tests, and documentation belong in Git.

## Art and offline review

Character-specific colors, boundaries, landmarks, and authoring evidence are in
[Darren](darren-portraits.md), [Linnet](linnet-portraits.md),
[Wiscar](wiscar-portraits.md), and [Wynne](wynne-portraits.md).
Two authors handled two characters each, with a separate art reviewer checking
the original art and actual target images.

- [Compact five-palette summary](../../../generated/four-npcs-preview/summary.png):
  one row per character, using the first neutral frame.
- [Complete Vanilla/Debug Blue review](../../../generated/four-npcs-preview/blue-review/index.html):
  four small pages, one per character, with all eight expressions and both frames.

The detailed pages show full portraits at 2× and faces at 4×: 32 sheets, 64 frames,
and 128 Vanilla/Blue views. The static HTML and separate PNGs need no server or
game. All review images come from `generated/characters-four-npcs-trial`.

The review manifest covers every asset/frame/palette combination without
duplicates or omissions. All 15,403,392 placed pixels match their source images;
no opaque source pixels are clipped. Chromium decoded all 32 sheets, checked
expression coverage against the manifest, and resolved every navigation link.
The summary, index, and first complete expression sheet were visually inspected.

Independent review inspected all 64 distinct full Source/Blue/Ryis frame trios,
all enlarged faces, and 24 distinct mouth crops in every target. Its final audit
checked 128 target strips, 13,639,680 pixels, and 153 independent literal
landmarks per target. It caught three missed Darren pocket-hand shadow pixels
at `[197..199,152]`. The correction adds exactly 48 pixels per target across all
16 frames; every other RGBA pixel and all metadata remain unchanged. The hand
was inspected in all four targets after the correction. Final review passed;
the report is `tmp/four-npcs-art-review-final.md`, with per-character manifests.
The user accepted the final offline previews. This does not establish in-game
rendering.

## Integration verification

Formatting, Clippy with warnings denied, all 87 active tests, the four new local
corpus tests, three existing opt-in GML tests, and the release build passed in
the pinned Fenix Nix shell. The synthetic character case first failed with
`Unsupported character: darren`, then passed after registry insertion, including
each new character alone and all 34 together.
After the final hand correction, formatting, Clippy, and the strengthened Darren
corpus test passed again. Its new literal landmarks first failed on the earlier
mask, then passed with the correction. The combined bundle, all variant checks,
offline previews, browser checks, and installation were regenerated afterward.

All 7,824 final variants passed exact recipe validation. The previous 30
character trees are byte-identical to the accepted Zorel bundle. The four new
characters' originals and all target images match their reviewed standalone
trees. A separate archive audit verified path coverage, source checksums,
geometry, original atlases, color arity, standard ramp roles, and custom shades.

An additional Fabricator interpreter check used the actual generated tables
for all four cameos, with NPC lookup configured to fail if called. All eight
expressions per character switched through all five choices, retained fractional
portrait phase, kept other characters' selections independent, restored Vanilla,
and applied selected palettes when reopening a textbox. Only the engine boundary
was simulated; the test ran the shipped palette GML. This is not a native
cutscene test.

A fresh isolated MOMI install into `tmp/four-npcs-playtest` passed packed-atlas,
metadata, and runtime-table verification. Its installed archive SHA-256 is
`165e456e3cdd62bfc4c19268435ef0b54b8fb182b8165c24bc7682e084bfad83`.
The receipt and retained previous archive match the original SHA-256 above.
The source backup stayed unchanged. The earlier candidate installation was
uninstalled before installing the corrected package; that roundtrip restored
the original archive exactly. The final package contains no preview helper.

Root evidence is under `tmp/four-npcs-`: `final-checks.log`, `integration-audit.json`,
`native-coverage.log`, `all-variants-validation.log`, `bundle-comparison.log`,
`preview-check.log`, `blue-browser-check.json`, `cameo-runtime.log`,
`install-report.json`, `source-after.sha256`, and `frozen-inputs.sha256`.
Recipe and registry inputs stayed unchanged through generation and validation.
Correction evidence includes `darren-final-checks.log`, `darren-delta.log`, and
`candidate-uninstall-report.json`. Final inputs were frozen again after the hand
correction; the earlier manifest remains `candidate-inputs.sha256`.

## Native testing limit

The existing F7 helper constructs `NpcSpeaker`. Native `CameoSpeaker` requires
an active cutscene and its cameo instance, so substituting the constructor would
not make the town preview work. `tmp/play-characters` retains the previous Zorel
package and helper. The new batch is reviewed offline; native cameo cutscenes
and in-game rendering have not been exercised.
