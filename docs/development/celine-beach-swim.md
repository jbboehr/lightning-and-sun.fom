# Celine's Beach swimming

Two `bath_swim` strips add East and South swimming. Their eight source frames
are distinct; native West mirroring adds four review cases. The profile
grows from 373 to 375 regions.
The user approved this slice for commit on 2026-09-25.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Beach-actions batch. The exact paths are
`assets/animations/NPCs/Celine/Sprites/Beach/spr_npc_celine_beach_bath_swim_{east,south}.png`.
Both strips retain four 80×80 frames, 0.15-second duration, Default atlas and
Middle/54 origin. The complete export is
`extracted/celine-beach-swim-study`; independently exported raw sidecars are
in `tmp/celine-beach-swim-author-metadata.json`.

Only the head is visible above the water. Masks cover the partly submerged
face while preserving hair, black outlines, gray eye details and the water
and splash colors `328BC9` and `9DEBFC`. The existing skin colors
`FCD9B3`, `F0B988`, `D37A57` and selected `672115` retain their original
light/middle/shadow/deep target roles. No extra shade or mapping is needed.

The 62 new seeds select 190 pixels per target: 132 core skin and 58 deep
contour pixels. Twelve identical dark hair pixels remain original. East
changes 28, 28, 21 and 21 pixels in its four frames; South changes 26, 26,
20 and 20. The lower face disappears behind the water in frames 2 and 3;
its visible skin still changes, while the water boundary stays intact.
Frame numbers in this document start at zero.

East frame 0 maps facial contour `[44,51]` but protects the same dark color
in hair `[34,47]`. Water `[33,55]`, surface highlight `[40,55]` and detached
splash `[48,59]` stay original. South frame 2 maps cheek contours `[35,53]`
and `[43,54]`; frame 3 retains the hair at `[34,54]` and ripple `[35,61]`.
These are literal source-art landmarks, independent of the mask seeds.

All eight actual frames were inspected in all four targets. The focused
opt-in test checks 22 landmarks across every frame, exact target shades,
all pixels, alpha, metadata, complete core skin coverage and equal target
selections. Deliberately omitting East `[44,51]` and selecting hair
`[34,47]` each caused the expected material assertion to fail; the unchanged
candidate then passed. Overrides use `FOM_CELINE_BEACH_SWIM_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_beach_swim -- --ignored'
```

All 373 earlier region objects, seven color groups and mappings remain
unchanged. All 3,730 prior original/variant PNG and metadata files match
`generated/characters-world-beach-actions-trial/characters/celine` exactly.
The final `generated/celine-beach-swim-trial` validates all 1,500 variants
with strict source hashes and matches the inspected candidate byte for
byte. No prior pin or mask was refreshed.

Profile SHA-256:
`8b6f2002e71f4051f63765c18b0d1b5a9f058365ae7a3b47e67e69ae698f8eb8`.
The common new-frame selection digest is
`4b5e54c8528d63dd07aff4420bd3bdc01a1ef8a71e2edc65c4889f340c9281f3`.

- [Five-choice summary](../../generated/celine-beach-swim-preview/summary.png):
  two swimming phases in both source directions.
- [Complete Vanilla/Blue review](../../generated/celine-beach-swim-preview/blue-review/index.html):
  all 12 direction/frame cases across three content pages.

The symmetric crop `[28,40,24,24]` includes the entire visible head and
every detached splash. Exact preview checks cover 1,382,400 full-review
pixels, 184,320 summary pixels, 20 summary bindings, raw metadata and West
reversal. All four West pairs were visually inspected. Chromium verifies
four HTML pages, 13 decoded images, all links and no horizontal overflow.
Evidence under `tmp/celine-beach-swim-author-` includes `audit.json`,
`green.log`, `omission-red.log`, `spill-red.log`, `validation.json`,
`preview-pixels.json` and `preview-browser.json`.

See [combined integration](world-beach-swim.md) for shared checks, native
metadata and installation. Static previews do not establish live animation
timing or natural outfit/state transitions. Source images, generated art
and helpers remain ignored; this slice changes no runtime code.
