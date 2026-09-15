# Hayden's remaining non-riding Spring specials

This batch adds seven `spr_npc_hayden_specialanimation_spring_` strips:
East hammer (6 frames), harvest (10), pet (7), till (5), water (4),
and South sigh (4), wipebrow (6). The 42 source frames plus 32 native West
mirrors produce 74 direction/frame cases. The extended profile has 168 sources
and 672 variants; all 161 accepted regions, color groups and mappings remain
unchanged. The user accepted the offline artwork review.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`. The full local corpus is
`extracted/hayden-special-study`; `tmp/hayden-special-metadata.json` records
each raw sidecar and frame count. All frames retain 80×80 geometry, Default
atlas and Middle/54 origin. Pet uses scalar duration `0.15`; the other six
strips retain their original duration arrays, including long end holds.

The additions use 867 component seeds, selecting 2,008 skin pixels per target
and excluding 408 matching shirt-shadow pixels. Existing world skin colors
are sufficient. Hair, beard, mouth, clothing, hammer head/wooden shaft/impact
effects, hoe and watering can stay original. In zero-based harvest frame 3,
the far forearm component at `[49,48]` changes while the same-colored sleeve
edge `[45,48]` stays original. Raised hands during hammer, pet, watering and
wipebrow use separately reviewed components around their moving cuffs.

All 42 frames were inspected in all four targets on
`tmp/hayden-special-art-*.png`; the 56 reviewed target PNG/metadata files
match `generated/hayden-special-trial` byte for byte. Component evidence and
frozen hashes are in `tmp/hayden-special-refinement.json` and
`tmp/hayden-special-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-special-preview/summary.png):
  four samples, about 64 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-special-preview/blue-review/index.html):
  all 74 cases across ten pages with at most eight cases each; about 2.1 MB.

Exact checks cover crop `[28,20,51,46]`, including every impact particle,
34,720,800 full-review pixels, 750,720 summary pixels, 20 sample bindings and
source/variant metadata. Chromium decoded every image and checked every case
and link. The focused opt-in corpus test and targeted Clippy pass; 34 literal
landmarks cover skin, shirt edges, tools, mouth and hair. Comparison preserves
all 1,610 prior original/variant PNG and metadata files. Evidence is in
`tmp/hayden-special-{preview-check,corpus,preservation,art-binding}.log` and
`tmp/hayden-special-preview-browser-check.json`.

See [the combined integration](spring-world-special.md) for full checks,
native probes and isolated installation. Static review does not exercise live
timing, schedules or state/outfit changes. Game artwork remains ignored; this
slice changes no runtime code.
