# Seridia portraits

All 164 main strips are included: 26 per season, 27 Beach, eight Wedding,
17 spectral Priestess, seven Flashback Priestess and one Dragon. Every strip
is 592×180 with two 296×180 frames at 0.2 seconds. The 328 occurrences contain
326 distinct full frames: Dragon repeats its frame, and Flashback `eyes_shine`
and `neutral` share their first frame. Atlas counts are 26 each Spring, Autumn
and Winter, 53 Summer including Beach, and 33 `PortraitsMisc`.

The source is the read-only `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Originals are exported to `extracted/seridia-portraits-study`. All game-derived
images, packages, galleries and temporary helpers remain ignored.

## Colors and material boundaries

Vanilla keeps the original art. Debug Blue, Adeline, Hayden and Ryis recolor
the 146 human strips, including warm Flashback Priestess. The 17 spectral
Priestess strips deliberately retain their monochrome blue treatment of skin,
hair and clothing. Those and Dragon use explicit empty seed lists; all 18
PNGs and metadata files remain byte-identical in every target. This pass does
not redesign spectral effects or dragon scales.

The profile has 143,676 seeds in 164 source-hash-bound regions, using 20
singleton color groups. Each target changes 1,129,994 pixels and protects
100,852 matching material pixels. None of the four targets has identity maps.

| Source | Target role | Selected | Matching protected |
| --- | --- | ---: | ---: |
| `#C1AFA5` | Light, horned skin | 897,757 | 39,082 |
| `#A69084` | Medium | 72,267 | 1,562 |
| `#8E746D` | Shadow | 103,003 | 24,976 |
| `#624A48` | Deep | 41,864 | 21,172 |
| `#B4A095` | Light + `[-13,-15,-16]`, upper-lip blend | 112 | 0 |
| `#B29E93` | Light + `[-15,-17,-18]`, subtle torso line | 2,490 | 0 |
| `#AD7165` | Medium + `[7,-31,-31]`, outer-lip fringe | 222 | 0 |
| `#FCDEBE` | Light, Flashback skin | 5,974 | 0 |
| `#F5BB90` | Medium | 1,738 | 0 |
| `#EA9866` | Shadow | 267 | 0 |
| `#B35A35` | Deep | 442 | 2,448 |
| `#9A4825` | Deep | 135 | 1,022 |
| `#F7C49D` | Medium, fine nose blend | 142 | 0 |
| `#E28A55` | Shadow, jaw blend | 56 | 0 |
| `#DA8A52` | Shadow, exposed neck and facial detail | 714 | 7,790 |
| `#C3AA9B` | Light, exposed hand blend | 2,192 | 1,904 |
| `#95726A` | Shadow, exposed hand blend | 564 | 896 |
| `#A5462C` | Deep, Flashback outer mouth contour | 34 | 0 |
| `#F3AC87` | Medium + `[-2,-15,-9]`, Flashback lower-lip fringe | 14 | 0 |
| `#FFE1C5` | Light + `[3,3,7]`, Flashback below-lip blend | 7 | 0 |

Custom offsets apply per RGB channel, clamped to 0–255. They retain subtle
source shading or lip tint without leaving isolated original skin pixels.
Selections include exposed faces, ears, necks, torso, hands and thighs, plus
Flashback's bare upper arm and narrow thigh slit. Flashback's hands are gloves.

Shared colors require component boundaries: beige horns and tail spines,
orange Flashback hair, brown chain links, Autumn/Wedding gauntlet lines and
Winter bone-plate shading stay original. Nearby actual skin using those same
colors still recolors. White face/body markings and nails, pink eyeliner,
lipstick, mouth interiors, blush, eye details, hair, gloves, gold and clothing
retain their artwork. Original makeup contrast against different skin tones
remains a subjective cosmetic detail.

## Art and data verification

Final variants are in `generated/seridia-portraits-reviewed/variants` under
`blue`, `npc_adeline`, `npc_hayden` and `npc_ryis`; all four pass exact recipe
validation. During the initial art pass, the author inspected 128 full
Source/Blue/Ryis trios, representative source forms and enlarged faces/bodies,
then all-target corrected material regions. Independent art review inspected
all 326 distinct full trios, face crops for all 291 distinct human frames,
80 distinct all-target mouth/material crops and nine outfit/body comparisons.
Exact deltas bind those views to the subsequent corrections.

The first correction restored 7,352 Flashback hair and forehead-chain pixels
per target. The second restored 18,428 body-chain, gauntlet and bone-material
pixels per target. Every other RGBA pixel and metadata byte remained identical.
The corrected regions passed enlarged all-target inspection, including literal
skin neighbors beside gauntlets. The initial independent audit checked 656
target strips, 69,903,360 pixels and 103 source-art landmarks per target.

User review identified exposed forehead skin between the circlet and hairline
that had been excluded with the horns. The follow-up selects 43 pixels per
frame inside `[160,41]`–`[168,47]`, across 139 seasonal, Beach and Wedding strips
and both frames: 11,954 added skin pixels per target. All other pixels and
metadata remain identical, including the circlet, hair and actual horns.
Flashback already selected this skin; its seven strips and all 18 deliberate
no-op strips remain byte-identical. Both distinct source patches (normal and
Wedding) were inspected enlarged in all four targets. See the
[forehead comparison](../../../generated/seridia-preview/forehead-detail.png)
and `tmp/seridia-forehead-author-audit.json`.

```sh
nix-shell --pure --run 'cargo test --locked --test seridia_portraits -- --ignored'
```

The focused test checks all four targets, dimensions, metadata, alpha, exact
replacement colors, unrelated colors and equal selections. It preserves
independently chosen source-art landmarks as 49 skin and 101 protected
occurrences per target, covering every source shade and both material/skin
sides of important boundaries. All 18 no-op seed lists and exact PNG bytes are
asserted. `FOM_SERIDIA_RECIPE` overrides Blue for local negative controls:
omitting `#C3AA9B` fails on Autumn hand skin at `[172,79]`; the saved early
candidate fails on Autumn gauntlet material at `[108,156]`. Both failures were
observed, and the final four-target test passes. Earlier literal controls also
reproduced the forehead-chain, Flashback bang and Winter bone spills before
their corrections. Twelve forehead assertions were added before the follow-up
fix: the prior recipe failed at Spring neutral `[162,43]`, then the corrected
recipe passed all four targets. Existing horn and circlet guards remain intact.

The initial component gallery reused all 328 occurrences in 326 groups without
conflicts. Reconstructing its selections reproduced every pre-forehead-fix
Blue PNG and metadata file byte for byte. Inclusive source bounds are
`[60,18,257,179]`. Current offline review face crops are `[127,40,55,51]` for
horned forms, expanded upward to show the forehead edge, and `[126,40,56,58]`
for Flashback/spectral forms; the Dragon detail crop is `[114,68,80,100]`.
Full-art previews use separate bounds to retain all visible pixels.

Ignored evidence includes `tmp/seridia-author-final-audit.json`,
`tmp/seridia-author-validated-*.json`, `tmp/seridia-author-test-*.log`,
`tmp/seridia-author-candidate20c-*.png` and
`tmp/seridia-art-review-final.md`. Follow-up evidence includes
`tmp/seridia-forehead-author-selection.json`,
`tmp/seridia-forehead-author-test-red.log` and
`tmp/seridia-forehead-author-test-green.log`.
No live rendering was part of this art pass.

Final profile SHA-256:
`02e3c1e950d80618c00fc8dfbcc82e83f5adbaf1216398656db5e10665652ca6`.
Final preset set SHA-256:
`eaf8f44df4291715fd0eb91b573e8fef6a76f7e27b503e7b73ddfcba0db4b083`.

## Integration and offline review

The combined collection now covers all 36 characters in the current NPC
palette catalog. Its 2,218 source animations comprise 2,201 portrait strips and
17 Adeline world animations, producing 8,872 variants. Seridia uses S to cycle
Vanilla, Debug Blue, Adeline, Hayden and Ryis. Shared Rust and GML runtime code
is unchanged. All 164 registry paths and atlas assignments match her native
portrait table; `flashback_priestess` and `bath_neutral` retain their complete
outfit/expression names.

- [Five-palette summary](../../../generated/seridia-preview/summary.png):
  the first Spring neutral frame, labeled as a sample.
- [Complete Vanilla/Debug Blue review](../../../generated/seridia-preview/blue-review/index.html):
  all 164 strips and both frames, including unchanged spectral and dragon forms.

The complete review presents 656 frame/palette views across 26 pages, with at
most eight expressions per page. Exact checks matched 107,859,680 placed
detail-page pixels and 661,780 overview pixels, with no visible source art
clipped. Chromium decoded all 164 sheets and verified links and expression
coverage on every page. The overview, index and representative outfit layouts
were visually inspected. The review totals about 32 MB, split into separate
images and small pages. Evidence: `tmp/seridia-forehead-preview-pixel-check.log`
and `tmp/seridia-forehead-browser-check.json`.

Verification on 2026-09-12 passed formatting, Clippy with warnings denied,
87 active tests, the local Seridia corpus test, three opt-in Fabricator runtime
tests, and the release build (`tmp/seridia-forehead-checks.log` and
`tmp/seridia-forehead-final-tail.log`). The first command was terminated during
the local test after the active suite passed; the unfinished local/runtime
tests and release build were rerun successfully. The standalone
and combined character fixture first failed with `Unsupported character:
seridia`, then passed after registration.

The combined build and all 8,872 variant validations passed. Previous character
trees are byte-identical to the Caldarus batch (20,715 files); Seridia's packaged
originals and four variants byte-match the reviewed outputs (1,645 files).
Evidence: `tmp/seridia-forehead-all-variants.log`,
`tmp/seridia-forehead-bundle-comparison.log` and
`tmp/seridia-forehead-integration-audit.json`.
All six recipe, registry and review inputs still match
`tmp/seridia-frozen-inputs.sha256`.

The actual generated GML table passed all 164 rows and five choices, including
fractional animation phase, Vanilla restoration, textbox reopening and one-time
key registration. S lookup, registration and polling passed against the pinned
MMAPI module; its installed copy is byte-identical to that tested source.
Evidence: `tmp/seridia-runtime.log`, `tmp/seridia-hotkey-check.log` and
`tmp/seridia-forehead-installed-hotkeys.log`. The regenerated table is
byte-identical to the table tested before the forehead correction.

A fresh MOMI installation in `tmp/seridia-playtest` verified packed pixels,
animation metadata and the runtime table. Its installed archive SHA256 is
`31be9cf46c05d88cbdc54cd4da40ed1e3a0f2e1b68e09d4c17c09b0de6906412`.
The source backup and retained `previous.zip` both still match the original
source hash above. See `tmp/seridia-forehead-install-report.json` and
`tmp/seridia-forehead-source-after.sha256`.

The user accepted the updated offline previews after the forehead correction.
This records preview approval, without a recorded case-by-case user review.

The mounted game directory was empty. This isolated player package has no
preview helper, and `tmp/play-characters` remains on the earlier Zorel copy.
An uninstall roundtrip and native Seridia gameplay were not exercised.
