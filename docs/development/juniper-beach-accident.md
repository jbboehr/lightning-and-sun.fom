# Juniper Beach Accident candidate

2026-11-05. **Authored and validated; user visual acceptance pending.**
The approved P1 batch adds all six Beach Accident strips: blink East/South,
idle East/North/South and the 22-frame South transformation. The new
`juniper-world-finish` profile preserves all 305 prior region objects, source
pins, seventeen source colors, nine groups and four preset mappings exactly.
It contains 311 regions: 132 portraits and 179 world strips. The integrator
owns collection selection, shared counts, registry and runtime integration.

## Sources and material decisions

The preserved archive is SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All six paths use
`assets/animations/NPCs/Juniper/Sprites/Beach Accident/` and prefix
`spr_npc_juniper_specialanimation_beach_accident_`. The profile pins every PNG;
the retained export additionally records source metadata hashes. Each strip uses
80×80 frames, Default atlas and offset `40.0,54.0`. Blink has three frames with
durations `[0.075,0.125,0.075]`; idle uses one-frame defaults. Transformation has
22 frames: twenty-one durations of `0.1`, then `1.0`.

Source inspection distinguishes normal exposed skin from the green accident
coloring and the baked transformation effect. Normal skin uses existing roles
`EFD89A`, `E3BF7F`, `BC8B43`, `763F21`. Green patches remain original; selecting
them as ordinary skin would erase the source condition. The hair tie's
`DF8D4B`/`FFD565`, gold bangle centers, separable bangle borders, purple hair,
eye detail, pale/lilac wraps and green-covered limbs remain unchanged.

The complete 22-frame transformation was inspected. Frames 1–2 recolor normal
skin visible around the green covering. Frames 3–17 deliberately retain their
baked full-body green/white/returning-color effect. These frames tint hair,
clothes and skin together and contain no normal skin-ramp pixels; they are not
ordinary unmasked human frames. Frames 18–22 recolor the returning normal skin
while preserving the remaining purple spell puffs, wraps and bangles. No new
palette roles, altered effects, or runtime tint assumptions are introduced.
This preserves the source effect rather than inventing target-specific spell art.

Separable `BC8B43` bangle borders stay original: frame-local `(34,45)` in East
idle/blink, `(33,45)` in South idle/blink, and `(46,45)` in North idle. The first
two transformation frames protect `(33,46)`; frames 18–22 protect all four normal
South bangle corners `(33,45)`, `(35,45)`, `(44,45)`, `(46,45)`. There are 31
protected matching-color occurrences and no new inseparable-material compromise.
The previously accepted bangle-edge exceptions in earlier assets are unchanged.

| Strip | Selected pixels per frame, each preset |
| --- | --- |
| Blink East | 39, 41, 39 |
| Blink South | 45, 47, 45 |
| Idle East | 39 |
| Idle North | 36 |
| Idle South | 45 |
| Transform South | 47, 47; fifteen zero-selection effect frames; 61, 72, 80, 82, 82 |

The batch has 31 source frame occurrences, 28 exact unique frames, three duplicate
occurrences and 847 selected skin pixels per preset. None of these new frame
masks is recorded as user-accepted.

## Complete review and verification

- [Every-frame Vanilla/Debug Blue candidate](../../generated/juniper-beach-accident-review/index.html):
  31 source frames and four native West mirrors, 35 complete cases over thirteen
  small case pages. Vanilla is left, Debug Blue right; both have full visible
  views plus enlarged details. Transformation spell particles above the head are
  included in the full view.
- [Five-choice SAMPLE overview](../../generated/juniper-beach-accident-review/summary.png):
  South/North accident idle, the white flash and the final normal-skin frame.
- [Component authoring gallery](../../generated/juniper-beach-accident-mask-review/index.html).
- Candidate build: `generated/juniper-beach-accident-build/`; full source corpus:
  `extracted/juniper-beach-accident-finish-study/`; reusable authoring inputs and
  evidence: `local/authoring/juniper-beach-accident/`.

Fresh checks passed:

```sh
nix-shell --pure --run 'cargo clippy --locked --release --test juniper_beach_accident -- -D warnings'
nix-shell --pure --run 'cargo test --locked --release --test juniper_beach_accident -- --include-ignored'
```

Two regular tests protect every prior region and palette role and package all six
synthetic strips with their exact native frame counts, duration arrays, offsets
and atlas. The local test builds all 311 real strips in all four presets and
checks every new RGBA pixel against independent source-material expectations,
fixed per-frame counts and thirteen literal skin/green-covering/bangle/effect
landmarks. Alpha and every metadata byte are preserved. It also rebuilds the
305-region predecessor and compares all 2,440 prior variant PNG/metadata files
byte-for-byte. The stable old `extracted/test-corpus/juniper` link is retained for
older local tests; this new test reads the complete dedicated 311-strip corpus.

All four variants separately passed exact CLI recipe validation over every one
of the 311 strips. Strict source hashes were used, without overrides. Two negative
controls fail at the expected pixels: omitting the South palm leaves `(32,46)`
uncolored; adding a bangle seed recolors protected `(33,45)`.

The final gallery checks 6,643,200 enlarged pixels against the current bundle,
compares the full binding set with all 70 expected Vanilla/Blue bindings, and
requires complete visible crops and metadata-derived frame/mirror coverage.
The authoring agent inspected all 35 cases. CPU Chromium passed every desktop
page and both mobile indexes: seventeen page loads, all local images/links
present and no viewport overflow. Agent inspection and automated checks do not
establish user art acceptance or natural gameplay acceptance.

## Measurements and limits

The source-inspection checkpoint was recorded at `2026-11-06T00:52:16Z`; focused
final checks were recorded at `00:57:42Z`. The roughly 5m26s checkpoint window
includes mask decisions, complete source/candidate inspection and focused tests,
but excludes earlier context reading and the integrator's preliminary extraction.
It is observed agent time, not human review throughput.

The complete 311-strip source export took 3.560s and the four-preset build 1.193s.
The focused test run took 6.87s after compilation; four exact recipe validations
took 7.911s combined. Final gallery generation/binding verification took about
one second. CPU Chromium took 44.367s. No local GPU or model inference was used.
Detailed measurements remain in the retained authoring directory.

User review, combined collection/install/remove checks and live transformation
timing/dispatch remain integration/release work. No accepted fixture baseline was
written and no source game pixels were tracked. All generated artifacts remain
ignored. The new build is about 53 MiB; predecessor comparison bundles are about
52 MiB each, with 3.5 MiB of complete extracted source and 5.3 MiB of authoring
evidence. One predecessor bundle was produced before a stdout-buffer limit was
corrected; the subsequent recorded build completed successfully.
