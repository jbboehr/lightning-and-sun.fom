# March Spring shocked and reading animations

Adds shocked start/loop/end South and special-animation seated-reading start/loop/end South. Six strips contain 13 frames (ten distinct images). Both cycles are South-only: this slice creates no West mirror views. March now has 209 sources and 836 recolored variants.

## Source and material boundaries

The original `tmp/fields-of-mistria/assets.zip` remains read-only. Every PNG is pinned to its exact source SHA-256. All six strips use 80×80 frames, the `Default` atlas and Middle/54 offset. Reading start/end each have three frames at 0.1 seconds; the four-frame loop uses `[3.0, 0.1, 3.0, 0.1]`. The three shocked phases each have one frame with default source timing. Native integration treats these as complex cycles, with reading seated.

All six use the existing world skin ramp: `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14`. No source colors, groups or mappings change. The new regions add 276 explicit component seeds selecting 506 pixels per target, including the tiny fingers visible around the opening book and the raised hands in the shocked loop.

The book is a distinct material. Its pages (`#F6E4D7` / `#C9AF9C`), brown cover (`#B67C6F`, `#855053`, `#663A3E`) and deep binding (`#422628`) remain original. Red mouth interiors, hair, goggles, green cuffs, apron, black eyes/outlines and footwear also remain unchanged. The old 203 region objects, hashes and seeds, sixteen colors, seven groups and all preset mappings are retained exactly.

Every actual frame was inspected in Vanilla and all four targets. Source grids separate the fingertip shadows from adjacent cover pixels. The shocked loop reaches y23, so this batch's preview crop is `[28,22,24,34]`, retaining the complete hair and hand silhouette.

## Verification

`tests/march_spring_reactions.rs` contains 30 literal source-art landmarks across fingers, face, book materials, goggles, cuffs, mouth and hair. Its opt-in corpus test checks strict hashes, exact source-or-target pixels, alpha, geometry and unchanged metadata across all four choices. It compares all 2,030 previous original/variant PNG and metadata files against `generated/characters-reina-juniper-march-standard-trial/characters/march`.

Two temporary negative controls failed at the intended landmarks: removing the deep finger component in reading start, zero-based frame 2, `[35,47]`; and mapping the book paper at `[38,42]`. The accepted candidate, normal data test, formatting and targeted Clippy passed. All 836 final variants validate and match the reviewed candidate bytes. Common selection SHA-256: `bcafbc2df8f60f523cc36e7c7c3b85d80588c1fe014b749ea3781db46326d1a6`.

```sh
nix-shell --pure --run 'cargo test --test march_spring_reactions'
nix-shell --pure --run 'cargo test --test march_spring_reactions -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_spring_reactions -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-spring-reactions-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-spring-reactions-trial
```

Local previews:

- `generated/march-spring-reactions-preview/summary.png`: five palette choices across reading and shocked poses.
- `generated/march-spring-reactions-preview/blue-review/index.html`: all thirteen actual frames in Vanilla and Debug Blue across three detail pages.

Preview verification checks exact source/palette identity, hashes, complete crops, metadata, each displayed pixel and the absence of invented mirror cases. Chromium checks every page, image and link. Parent integration handles the combined package, installation and native cycle probes separately. Game images, generated variants and review artifacts stay ignored and uncommitted.
