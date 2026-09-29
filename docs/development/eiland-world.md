# Eiland's first overworld batch

Six Spring idle/walk strips add North, South and East, with 15 source frames, nine distinct images and five native West mirrors. The new world profile, preset set and Debug Blue recipe extend all 78 accepted portrait regions to 84 sources and 336 recolored variants. The portrait-only definitions remain unchanged. Choices are Vanilla, Debug Blue, Hayden, Ryis and Seridia.

[Five-choice summary](../../generated/eiland-world-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/eiland-world-preview/blue-review/index.html)

Sources are `assets/animations/NPCs/Eiland/Sprites/Spring/spr_npc_eiland_spring_{idle,walk}_{north,south,east}.png`. The complete local corpus is `extracted/eiland-world-study`; raw sidecars are recorded in `tmp/eiland-world-author-metadata.json`. Fresh exports retain 80×80 frames, the `Default` atlas and `Middle`/54 offset. Idle uses single-frame default timing; walk has four frames at 0.15 seconds each. Native West views mirror the East packs.

The read-only archive SHA-256 is `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`. Every portrait pin remains strict and unchanged; every new sprite is pinned independently.

The world highlight `#E9A980` and fine forehead shadow `#9C5241` already exist in the accepted portrait palette and retain their roles. Three additional shades use existing target roles:

| New source shade | Target role |
| --- | --- |
| `#DE8F5D` | Middle, from preset color 1 |
| `#BA6A4C` | Shadow, from preset color 2 |
| `#7D3B14` | Deepest, from preset color 3 |

Each addition has its own new color group. All nine prior mapping entries and four prior groups remain unchanged. The canonical Debug Blue recipe matches every role in the world preset set, and all 780 accepted portrait original/variant PNG and metadata files remain byte-identical.

The 183 new component seeds select 366 skin pixels per target. All 15 actual frames were inspected in Vanilla and all four targets. Faces, jaw shading, fine warm forehead shading and bare hands change. Pink hair, eye details, white cape, gold trim, belt, uniform, trousers and boots retain their source colors. Sixty pixels of `#BA6A4C` shade the gold shoulders and belt rather than skin; their 45 components are explicitly excluded. The existing deep portrait shade `#6A3126` appears in belt and cape trim in these strips, so it stays original here. No portrait material choices are changed.

| Strip | Selected skin pixels per frame |
| --- | --- |
| Idle East / North / South | 32 / 4 / 38 |
| Walk East | 32, 37, 32, 31 |
| Walk North | 4, 3, 4, 3 |
| Walk South | 38, 36, 38, 34 |

`tests/eiland_world.rs` checks the preserved portrait regions and roles, the three isolated aliases and the canonical Debug Blue mapping. Its opt-in source-corpus test checks 50 literal skin/material landmarks, every source-or-target pixel against independent skin and trim inventories, per-frame counts, alpha, dimensions, metadata, strict source hashes and equal selections across all four targets. It also compares every earlier output file with `generated/characters-reina-juniper-march-wedding-finish-trial/characters/eiland`.

Temporary controls fail at the intended material assertions: removing the new middle-face shade at idle East `[39,38]`, or selecting the shared belt shadow at `[40,46]`. The accepted candidate corpus test, normal data test, formatting and targeted Clippy pass. All 336 final variants strictly validate and match the inspected candidates. Common selection SHA-256: `4d86e2dfa8cfc8edb042cf0fb913d342a550f3e23e56d44c9ee7c3a4967fca84`.

The compact gallery covers all twenty direction/frame cases across three detail pages. Crop `[28,24,24,34]` contains the full artwork bounds `[31,27,48,55]` and is symmetric for West mirroring. Exact preview checks cover source/palette/frame bindings, hashes, raw sidecars, complete crop coverage and every displayed pixel: 3,264,000 gallery pixels and 326,400 summary pixels. Chromium checks all four HTML pages, twenty-one decoded images and their links. Static previews do not establish native timing or natural scheduling. See [shared integration](balor-valen-eiland-world.md) for direct native animation probes and isolated installation; natural schedules and live gameplay remain untested.

```sh
nix-shell --pure --run 'cargo test --test eiland_world'
nix-shell --pure --run 'cargo test --test eiland_world -- --ignored'
nix-shell --pure --run 'cargo clippy --test eiland_world -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/eiland-world-study \
  --presets palettes/sets/eiland-world-trial.json \
  --output generated/eiland-world-trial
```

Local evidence is under `tmp/eiland-world-author-*`: raw metadata, source inventory, all-target sheets, literal landmarks, omission/spill logs, frozen hashes, final validation, prior-file preservation and preview/browser results. Game images and generated artifacts remain ignored. This slice introduces no runtime changes.
