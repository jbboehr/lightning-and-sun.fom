# Elsie seasonal world completion

2026-11-05. The final six Winter seated-writing strips extend Elsie from 158 to
164 selected sources: 36 portraits and all 128 seasonal world strips. Standing,
walking, blinking, sitting, food/drink, general actions, writing and singing are
covered where available in the native seasonal folders.

The user replied **"accept"** following the displayed-art review request covering
70 pending Summer/Autumn/Winter strips, including this final batch. This records
acceptance of the displayed artwork; the depth of the user's image inspection was
not separately stated, and no live-game acceptance is inferred. Accepted outputs
and the profile are preserved at `local/fixtures/finish-elsie/`, with the response
and scope in `acceptance.json`. Previous historical baselines are unchanged.

## Final Winter batch

| Cycle / direction | Frames | Selected skin pixels per frame |
| --- | ---: | --- |
| Seated writing end, East | 2 | 33, 50 |
| Seated writing end, South | 2 | 40, 54 |
| Seated writing loop, East | 4 | 30, 37, 34, 38 |
| Seated writing loop, South | 4 | 37, 40, 40, 40 |
| Seated writing start, East | 2 | 50, 33 |
| Seated writing start, South | 2 | 54, 40 |

The 16 source frames and eight native West mirrors have 650 selected source skin
pixels. Only exposed facial skin and closed eyelids change. The four existing
world shades occur only within the independently inspected facial bounds in these
pinned sources. Gray hair, hood, cream trim, gloves, green clothing, boots, eye and
mouth interiors, book and quill preserve every original pixel. No color role or
target mapping changed; all 158 earlier regions remain equal.

The frames remain 80×80, on the Default atlas at offsets 40/54. Start retains
duration `0.125`; loop retains `[0.1, 0.125, 0.1, 0.3]`; end retains
`[0.125, 0.1]`. Package checks preserve exact native animation properties. No NPC
cycle configuration or runtime script was changed for this batch.

## Verification and review

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six native registry entries. The new opt-in material
test checks every pixel of all four variants, fixed per-frame counts, independently
inspected face bounds, exact metadata and byte equality of all previous outputs
against the pre-change snapshot. A valid omitted-face-mask mutation fails its
intended pixel assertion. The temporary candidate is never an accepted baseline.

All retained Elsie artwork tests and the three GML runtime tests pass. Normal
formatting, Clippy, regular tests and the release build also passed at this
checkpoint; subsequent pilot integration is separate work in progress. Strict
generation preserves source image pins and the recorded original archive.

- [Final six-strip review](../../generated/finish-elsie-final-six-review/index.html):
  24 Vanilla/Debug Blue cases, 48 full-view bindings and a five-choice sample.
- [Consolidated pending-art review](../../generated/finish-elsie-pending-review/index.html):
  70 strips, 248 source frames and 72 mirrors, giving 320 cases.

Every gallery full view contains the complete native frame. Native mirrors,
metadata and all rendered source crops were checked against actual candidate
images: 4,955,136 final-batch pixels and 61,332,480 consolidated-review pixels.
All local HTML image/link targets resolve. Chromium loaded both entry pages and
a detailed multi-frame page without missing images or horizontal overflow.

Authoring sources, the pre-change profile/output comparison, masks and grids are
retained in `local/authoring/finish-elsie/`; the complete pinned source corpus is
`extracted/elsie-finish-study`, reached by `extracted/test-corpus/elsie`.
Galleries and generated game material remain local and ignored. No game was
launched or installed for this batch. Current combined install/removal and natural
gameplay verification remain later wave/release gates.
