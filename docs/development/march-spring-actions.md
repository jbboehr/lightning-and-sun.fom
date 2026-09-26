# March Spring actions

Adds eleven regular Spring strips: blink East/South, and sit, eat and drink North/South/East. They contain 31 source frames (23 unique images), with twelve native mirrored West views. March now has 198 sources and 792 recolored variants. His 181 portraits and first six world strips remain unchanged.

## Source and material choices

The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256 `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`. Every PNG has its own strict source hash. All new strips retain their original metadata: 80×80 frames, `Default` atlas and Middle/54 offset.

| Animation | Frames per direction | Source duration |
| --- | --- | --- |
| Blink East/South | 3 | `[0.075, 0.125, 0.075]` |
| Sit North/South/East | 1 | Native default |
| Drink North/South/East | 3 | `1.0` |
| Eat North | 3 | `1.0` |
| Eat East/South | 5 | `[0.125, 0.15, 0.175, 0.125, 0.6]` |

The existing four world skin shades cover face, neck and moving hands. Drinking East/South additionally uses `#E8B171`, one green step below the existing `#E8B271`. This is visibly skin on the raised arm and hand; leaving it unchanged produced 24 warm pixels. It now has a separate appended group and uses the existing midtone role: Debug Blue `#7F9FBD`, Hayden `#CA9052`, Ryis `#814A3A`, Seridia `#A69084`.

The old 187 region objects, hashes, seeds, six groups and all fifteen earlier source/target values are preserved. There are 585 new explicit component seeds, selecting 1,077 pixels per target. All occurrences of the five world skin shades in these eleven strips are skin. The added color has no seeds in previous regions; byte comparisons confirm it does not leak into old outputs.

Actual red mouth interiors (`#410808` and `#9E2626`), black closed eyes and outlines, red hair, gray-blue goggles/apron, green cuffs, trousers and boots stay original. The source drinking animation includes tiny near-identical goggle/trouser/outline colors; those are material colors and remain unchanged. Held food and cups are drawn separately by the game; the previews show the complete character strips, without inventing those attachments.

## Verification

Every actual new frame was inspected in Vanilla and all four targets. Literal source grids resolved closed-eye boundaries, open-mouth edges, cuffs and the fine drinking shade. The focused test contains 33 independently selected skin/material landmarks and also checks every new pixel against its exact source or expected target, alpha, dimensions, source hashes and metadata. It compares all 1,870 older original/variant PNG and metadata files to `generated/characters-reina-juniper-march-world-trial/characters/march`.

The initial four-shade recipe failed at drinking East, zero-based frame 1, `[40,40]`: `#E8B171` was unchanged. A separate temporary spill control mapped the red mouth interior and failed eating East, frame 2, `[42,37]`. The final candidate passed; normal data assertions, formatting and targeted Clippy also passed. No production runtime changes were needed.

Selection SHA-256: `a49187e05d7f8c4464cffc47c4091960960b41ff78f56630da7312621c91ecdd`. The selection is identical across all targets. All 792 final variants validate and match the inspected candidate bytes.

```sh
nix-shell --pure --run 'cargo test --test march_spring_actions'
nix-shell --pure --run 'cargo test --test march_spring_actions -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_actions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-actions-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-actions-trial
```

Local previews:

- `generated/march-spring-actions-preview/summary.png`: five choices across blinking, sitting, eating and drinking poses.
- `generated/march-spring-actions-preview/blue-review/index.html`: all 43 direction/frame cases in Vanilla and Debug Blue, including twelve West mirrors, across eight small detail pages.

Preview evidence verifies exact source/palette bindings, hashes, complete crops, every displayed pixel, metadata and native West mirroring. Chromium checks every page, image and link. Shared integration separately covers the combined package, isolated installation and native animation probes with simulated engine services; live gameplay is not covered. Source images, generated variants and review sheets remain ignored local files.
