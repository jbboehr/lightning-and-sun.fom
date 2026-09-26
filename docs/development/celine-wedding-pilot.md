# Celine's Wedding idle and walk

Six strips add Wedding idle/walk North, South and East. Their 15 source
frames contain nine distinct images; native West mirroring adds five review
cases. The profile grows from 375 to 381 regions. Other Wedding animations
remain outside this slice.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Beach-swimming batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Wedding/`, named
`spr_npc_celine_wedding_{idle,walk}_{north,south,east}.png`. The full export
is `extracted/celine-wedding-pilot-study`; raw sidecars and frame counts are
in `tmp/celine-wedding-pilot-author-metadata.json`.

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. Idle
uses single-frame defaults; walking retains four frames at 0.15 seconds.
All prior region objects, seven color groups and source/target mappings
remain unchanged. The existing `FCD9B3`, `F0B988`, `D37A57` and selected
`672115` retain light/middle/shadow/deep target roles. No additional shade
is needed.

Actual Wedding materials were inspected across every frame and all four
targets. Masks cover face, neckline, exposed wrists, ankles and toes while
preserving the white gloves, pink dress/veil, hair, eye details and pink
shoe straps/heels. The footwear interpretation follows the source topology:
all three views show skin ankles above the pink strap; front and East views
also expose light toes, while North retains a pink heel. The stepping poses
show the same separation. An independent inspection agreed with this choice.

Literal examples, using frame-local coordinates and zero-based frames:
idle East neck `[39,41]`, wrist `[35,44]`, ankle `[38,51]` and toe `[39,53]`
change. Adjacent glove `[35,46]`, strap `[38,52]`, veil `[30,41]` and dark
hair `[32,41]` stay original. North heel `[37,53]` stays pink. Walk East
frame 3 maps the deep wrist `[38,45]` but protects hair `[34,45]`; walking
North exposes a tiny wrist at `[45,45]` in frame 1 and `[34,45]` in frame 3.

The 173 new seeds select 485 pixels per target: 367 core skin and 118 deep
contour pixels. Another 173 matching dark hair pixels remain original.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Idle North | 1 | 4 |
| Idle South | 1 | 52 |
| Idle East | 1 | 44 |
| Walk North | 4 | 14 |
| Walk South | 4 | 200 |
| Walk East | 4 | 171 |

The focused opt-in test checks 41 independent source-art landmarks, four
target ramps, every new pixel, complete core skin coverage, alpha,
metadata, per-frame coverage and equal target selections. Negative controls
fail on an omitted face contour `[44,36]` and a selected hair component
`[32,41]` in idle East. The unchanged candidate then passes. Local controls
use `FOM_CELINE_WEDDING_PILOT_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_wedding_pilot -- --ignored'
```

All 1,524 standalone variants validate with strict source hashes and exact
recipes. The final `generated/celine-wedding-pilot-trial` matches the
inspected candidate PNG and metadata files byte for byte. All 3,750 earlier
original/variant files match
`generated/characters-world-beach-swim-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

Profile SHA-256:
`88462013bee93f38ccb8fec150fe79d0367d1da70e02809c27642fd9e3939330`.
The common new-frame selection digest is
`dc374b005de47acc1b5552fbdba2c5ef46977c9cb0d81557af5cdc4db26a5733`.

- [Five-choice summary](../../generated/celine-wedding-pilot-preview/summary.png):
  idle North/South and walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/celine-wedding-pilot-preview/blue-review/index.html):
  all 20 direction/frame cases on three content pages, at most eight per page.

Crop `[28,24,24,32]` includes all visible source pixels and is symmetric for
native West reversal. Exact checks cover 3,072,000 full-review pixels,
245,760 summary pixels, all 20 summary bindings, raw metadata and West
reversal. All five West pairs were visually inspected. Chromium verifies
four HTML pages, 21 decoded images, all links and no horizontal overflow.
Evidence under `tmp/celine-wedding-pilot-author-` includes
`initial-audit.json`, `green.log`, `omission-red.log`, `spill-red.log`,
`validation.json`, `preview-pixels.json` and `preview-browser.json`.

See [combined integration](world-wedding-pilot.md) for shared checks, native
metadata and installation. Static images do not establish live timing or
natural outfit/state transitions. Source art, generated variants, previews
and temporary helpers remain ignored. This slice changes no runtime code.

The user approved this artwork for commit on 2026-09-25.
