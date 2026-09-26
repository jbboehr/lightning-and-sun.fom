# March world pilot

March now has Spring idle and walking animations in all four palette choices, alongside his accepted portraits. Six strips cover North, South and East; the game derives West by mirroring East. This is 15 source frames (nine unique images), five additional West views, and 24 new recolored strips. Other March world actions and outfits remain for later slices.

## Sources and masks

The source archive is the read-only local `tmp/fields-of-mistria/assets.zip`, SHA-256 `0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`. Each new PNG is pinned independently in the profile. All six use the `Default` atlas, 80×80 frames and native Middle/54 offset. Idle has one frame; walking has four at 0.15 seconds per frame. Raw sidecars remain unchanged.

The world profile retains all 181 portrait regions, their hashes, dimensions and seeds, the original 13 colors and four groups. Two colors are appended in separate groups:

| Source | Role | Debug Blue | Hayden | Ryis | Seridia |
| --- | --- | --- | --- | --- | --- |
| `#EEDDA5` (existing) | light | `#9DB9D4` | `#E8B271` | `#B06C57` | `#C1AFA5` |
| `#E8B271` (new) | mid | `#7F9FBD` | `#CA9052` | `#814A3A` | `#A69084` |
| `#D37A57` (new) | shadow | `#6687AD` | `#B27146` | `#63342A` | `#8E746D` |
| `#7D3B14` (existing) | deep | `#445F83` | `#6E4922` | `#491F1B` | `#624A48` |

All occurrences of those four colors in these six world strips are skin: face/ear edges, neck and moving hands, including disconnected dark knuckles and jaw pixels. Explicit component seeds still bind each selection to the exact source file. The six new regions contain 296 seeds selecting 536 pixels. Red hair, gray-blue goggles and apron, green shirt/cuffs, black eyes/outlines, burgundy trousers and boots remain original. The new groups have no seeds in the portrait regions; actual portrait outputs were compared byte-for-byte to confirm that the new colors do not leak into them.

The portrait-only profile, set and Debug Blue recipe remain untouched. The new world definitions total 187 strips and 748 recolored variants.

## Verification

All 15 actual frames were inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. Source grids resolved the tiny neck/knuckle components and boundaries next to the goggles and cuffs.

`tests/march_world.rs` checks preservation of all portrait definitions and palette roles without game data. Its opt-in corpus test exercises all four actual outputs, source hashes, literal skin/material landmarks, every new pixel, alpha, geometry and metadata. It verifies all 1,810 prior original/variant PNG and metadata files against the accepted combined baseline.

Two temporary negative controls demonstrated failures before the accepted candidate passed: removing the deep jaw component failed the literal `idle_south` `[36,38]` skin landmark; deliberately mapping/seeding actual red hair failed the protected `[39,30]` landmark. These controls change temporary candidates only. The accepted selection is identical across targets and changes 536 pixels per target; mask SHA-256 is `b4cf24deecc5580472ac943d180bdcfae1f5e6a9e133b0b066050f8d5f536a8e`.

```sh
nix-shell --pure --run 'cargo test --test march_world'
nix-shell --pure --run 'cargo test --test march_world -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/march-world-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-world-trial
```

The local five-choice overview is `generated/march-world-preview/summary.png`. The paginated `generated/march-world-preview/blue-review/index.html` shows all 20 direction/frame cases in Vanilla and Debug Blue, including all five West mirrors. Coverage evidence binds each displayed image to its exact source, palette, hash, crop and frame. Local preview checks compare displayed pixels and metadata; Chromium checks every page, image and link. Parent integration handles the combined package, installation and native game checks separately.

All extracted images, generated variants, summaries and inspection helpers remain ignored local artifacts; only authored recipes, tests and documentation belong in Git.

The user approved this artwork for commit on 2026-09-25.
