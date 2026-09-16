# Hayden's alternate riding idles and blink

This batch adds six Spring riding strips: blink East/South (three frames each),
idle 2 East/North/South (nine/eight/four), and idle 3 East (eight). There are
35 source frames and 20 native West mirrors. The profile now has 180 sources
and 720 variants; all 174 accepted regions, groups and mappings remain
unchanged. The user accepted the offline artwork review.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`, with the
`spr_npc_hayden_specialanimation_spring_` prefix. The full local corpus is
`extracted/hayden-outfit-actions-study`. Raw sidecars are recorded in
`tmp/hayden-outfit-actions-metadata.json`: 80×80 frames, Default atlas,
Middle/54 origin. Blink retains `[0.05,0.15,0.05]`; idle 2 North/South retain
scalar `0.125`; East idles retain their arrays, including idle 2's `0.8` hold.

The horse, mane/tail and gear remain original through their blink, head, leg
and tail movements. No new skin shade was needed. The masks use 634 component
seeds, selecting 1,240 skin pixels per target and excluding 242 matching shirt
pixels. Most skin components match the accepted riding idle. South idle 2's
zero-based frames 0/2 have a changed hand silhouette: `[35,29]` and `[36,29]`
are skin, while neighboring shirt `[37,27]` remains original. Horse muzzle
shade `#958676` and saddle `#E7BA83` stay untouched.

All 35 frames were inspected in every target on
`tmp/hayden-outfit-actions-art-*.png`. Their 48 target PNG/metadata files match
`generated/hayden-outfit-actions-trial` byte for byte. Component evidence and
frozen hashes are in `tmp/hayden-outfit-actions-refinement.json` and
`tmp/hayden-outfit-actions-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-outfit-actions-preview/summary.png):
  three cycle samples, about 58 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-outfit-actions-preview/blue-review/index.html):
  all 55 cases across seven pages, at most eight per page, about 1.56 MB.

Exact checks cover crop `[22,9,38,48]`, 20,064,000 full-review pixels,
437,760 summary pixels, 15 sample bindings and source/variant metadata.
Chromium decoded every image and checked all cases and links. The opt-in
corpus test and targeted Clippy pass: 31 literal material landmarks, complete
horse/gear region checks, per-frame skin coverage and all 1,740 prior files.
Evidence: `tmp/hayden-outfit-actions-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-outfit-actions-preview-browser-check.json`.

See [the combined integration](world-outfit-actions.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing, schedules or state transitions. Artwork remains ignored; this
batch changes no runtime code.
