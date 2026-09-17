# Hayden's riding run and jump

This batch adds six Spring strips: riding run and jump North, South and East,
four frames each. The 24 source frames produce 32 direction/frame cases with
eight native West mirrors. The profile now covers 186 sources and 744 variants;
all 180 accepted region objects, color groups and mappings remain unchanged.
The user accepted the offline artwork on 2026-09-16.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`, with the
`spr_npc_hayden_specialanimation_spring_ride_` prefix. The full local corpus is
`extracted/hayden-outfit-special-study`. Raw sidecars are recorded in
`tmp/hayden-outfit-special-metadata.json`: 80×80 frames, Default atlas,
Middle/54 origin. Run retains scalar `0.1`; jump retains
`[0.15,0.15,0.15,0.1]`.

The horse and gear are baked into these strips. Reviewed component masks use
411 seeds, selecting 842 skin pixels per target while excluding 216 matching
clothing pixels. No new shade or target mapping was needed. Masks follow the
rider's vertical motion and both hands; the horse, mane, tail, saddle, boots,
hair and beard remain original. For example, zero-based run South frame 2
changes hand pixels `[36,30]` and `[43,30]` while retaining shirt pixels
`[37,26]` and `[42,26]`. East run/jump skin can extend to row 35.

All 24 frames were inspected in all four targets on
`tmp/hayden-outfit-special-art-*.png`. Their 48 target PNG/metadata files match
`generated/hayden-outfit-special-trial` byte for byte. Component evidence and
frozen hashes are in `tmp/hayden-outfit-special-refinement.json` and
`tmp/hayden-outfit-special-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-outfit-special-preview/summary.png):
  three samples, about 60 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-outfit-special-preview/blue-review/index.html):
  all 32 cases across four pages of eight, about 936 KB.

Exact checks cover the full visible crop `[17,5,45,54]`, 15,552,000 review
pixels, 583,200 summary pixels, 15 sample bindings, source/variant metadata
and native West reversal. Chromium decoded every image and checked all cases
and links. The opt-in corpus test and targeted Clippy pass: 33 literal
material landmarks, horse/gear preservation, per-frame coverage and all
1,800 prior original/variant PNG/metadata files. Evidence is in
`tmp/hayden-outfit-special-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-outfit-special-preview-browser-check.json`.

See [the combined integration](world-outfit-special.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this batch
changes no runtime code.
