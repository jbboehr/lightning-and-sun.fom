# Celine's Spring everyday actions

This slice adds normal Spring blink, sit, eat and drink to the accepted Celine
idle/walk pilot. Eleven strips contain 31 frame occurrences and 23 distinct
frames. East artwork supplies another 12 native West views. Garden outfits and
other world actions remain outside this slice.

## Source and masks

Sources are `assets/animations/NPCs/Celine/Sprites/Spring/`, with the prefix
`spr_npc_celine_spring_`: blink South/East and sit/eat/drink North/South/East.
Blink has three frames per direction, sit one, drink three, and eat five
South/East or three North. All use 80×80 frames, the Default atlas and
Middle/54 origin. Original metadata, including nonuniform blink/eat timing,
is preserved byte-for-byte in `tmp/celine-actions-metadata.json`.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local extraction is `extracted/celine-actions-study`.

The profile grows from 189 to 200 regions. All previous region objects, seven
color groups and target values remain unchanged. Existing world shades
`FCD9B3`, `F0B988` and `D37A57` map to the target light/middle/shadow colors.
The separately grouped `672115` maps to deep skin only on reviewed face and
hand components. Its hair, belt and boot occurrences stay original, as do the
orange `B65932` material shade and `410808`/`9E2626` mouth interiors.

The 348 new seeds select 1,131 pixels per target: 829 core skin pixels and
302 outline pixels. Another 244 matching outline pixels remain protected.
This includes the tiny North eating/drinking hand, raised eating fingertip,
South wrists and large eating-mouth borders. East sitting's lower dark pixels
remain boots. Every frame has selected skin, ranging from two to 62 pixels.

| Action | Strips | Frames | Changed pixels per target |
| --- | ---: | ---: | ---: |
| Blink | 2 | 6 | 311 |
| Sit | 3 | 3 | 89 |
| Drink | 3 | 9 | 277 |
| Eat | 3 | 13 | 454 |

Profile SHA-256:
`c391693aaeb587e1c3d69bbbb6114e36d8aeef5aef9b854eafa32613910194c1`.
Component decisions and final counts are in `tmp/celine-actions-mask-decisions.json`
and `tmp/celine-actions-final-audit.json`. The stylized recipe's description
was updated; its color map and the preset set are unchanged.

## Review and verification

Every new source frame and all four target versions were inspected, including
the large eating mouths, lowered and raised hands, hair edges and boots.
All 800 standalone variants validate exactly. The standalone output
`generated/celine-actions-trial` matches the inspected candidate, and all 1,890
previous original/variant PNG and metadata files remain byte-identical.
All 32 repeated-frame target comparisons also pass.

The opt-in test in `tests/celine_actions.rs` checks four literal target ramps,
every new pixel, alpha, metadata, complete core-shade coverage, common selection,
per-strip totals and 24 source-art skin/material landmarks. Removing the single
raised-fingertip seed at East eating frame two `[46,42]` in an ignored profile
copy makes its boundary assertion fail; the actual profile passes afterward.
Evidence: `tmp/celine-actions-missing-fingertip-red.log`,
`tmp/celine-actions-test-final.log` and `tmp/celine-actions-duplicates.log`.

- [Compact five-choice summary](../../generated/celine-actions-preview/summary.png):
  blink, sit, drink and eat samples, approximately 52 KB.
- [Complete Vanilla/Blue review](../../generated/celine-actions-preview/blue-review/index.html):
  all 31 source frames and 12 West mirrors on eight pages, at most eight cases
  per page; approximately 884 KB for the complete review directory.

The fresh full-art crop `[28,23,23,34]` contains every visible pixel in every
choice. Exact placement checks cover 6,725,200 full-review pixels and 250,240
summary pixels, all sample/palette bindings, original metadata and West reversal.
Chromium decoded all 43 comparison images and the summary, verified all eight
pages against the coverage manifest and resolved their links. Evidence:
`tmp/celine-actions-preview-pixel-check.log` and
`tmp/celine-actions-preview-browser-check.json`.

The user accepted the offline artwork review. Static images do not show native timing,
separately drawn held items or live-game behavior. Shared integration and
installation verification are recorded with the parallel Spring-actions batch.
All extracted artwork, generated previews and local evidence remain ignored.
