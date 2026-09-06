Investigation follow-up: see `INVESTIGATION.md`, `TOOLS.md`, and
`development/verification.md` for the implemented proof of concept and verification
record. The supplied build uses `assets.zip` and NPC Studio's internal engine.
The toolkit is now Rust, with a Fenix dev shell; follow TOOLS.md for current commands.
The original brief below preserves its earlier GameMaker and Python suggestions
for historical context only.

You are investigating whether Fields of Mistria can support a neutral character palette/complexion preset mod.

Goal:
Determine the safest, most maintainable path to build a mod that lets players select alternate palette/complexion presets per character while preserving vanilla assets as the default. This is an investigation/scaffolding task, not a production art pass.

Context:
Fields of Mistria appears to be a GameMaker/GameMaker Studio game. The largest asset file is likely `data.win`. Modding appears to be unofficial. Community tooling may include UndertaleModTool / UndertaleModCli and MOMI / Mods of Mistria Installer. Assume the game may change before 1.0, so prioritize reversible workflows, documentation, and small proofs of concept.

Hard requirements:

* Do not overwrite the original game files without creating backups.
* Prefer mod-loader or patch-based workflows over direct mutation of `data.win`.
* Preserve vanilla/original palettes as the default preset.
* Design the system as general palette customization, not removal/replacement of any protected class representation.
* Do not use inflammatory naming in files, docs, comments, or config.
* Keep the proof of concept limited to one character and one or two assets.
* Make all scripts deterministic and reviewable.
* Do not commit extracted copyrighted game assets to the repo.

Investigation questions:

1. What public modding tools currently work with Fields of Mistria?

   * Check MOMI / Mods of Mistria Installer.
   * Check UndertaleModTool / UndertaleModCli support for the current `data.win`.
   * Identify whether there is an existing documented asset replacement format.

2. Can character portraits and overworld sprites be exported and reimported cleanly?

   * Find where portrait textures/sprites live.
   * Find where overworld sprites live.
   * Determine whether assets are individual sprites, texture pages, or both.
   * Note whether transparency, dimensions, offsets, origins, or frame metadata must be preserved.

3. Can a tiny replacement mod be packaged without directly editing the user’s live install?

   * Replace one harmless test asset or one copied portrait asset.
   * Document the exact install/uninstall process.
   * Verify whether Steam updates would break or overwrite the change.

4. What is the best internal structure for a future per-character preset system?
   Proposed shape:

   ```text
   mod/
     manifest.json
     README.md
     config/
       character-palettes.json
       presets.json
     scripts/
       export-assets.sh
       build-contact-sheets.py
       apply-palette.py
       validate-assets.py
     palettes/
       vanilla/
       light/
       medium/
       deep/
       warm/
       cool/
       stylized/
     generated/
       # ignored by git
   ```

5. What data model should palette replacement use?
   Draft something like:

   ```json
   {
     "characters": {
       "example_character": {
         "portrait_assets": ["..."],
         "sprite_assets": ["..."],
         "presets": {
           "vanilla": {
             "description": "Original game palette; no changes."
           },
           "light": {
             "skin_ramp": {
               "#F7CFAE": "#F2D2BD",
               "#E7A678": "#DCA27F",
               "#B96F54": "#A96750"
             }
           },
           "deep": {
             "skin_ramp": {
               "#F7CFAE": "#B87856",
               "#E7A678": "#8F553B",
               "#B96F54": "#633527"
             }
           }
         }
       }
     }
   }
   ```

6. Can palette replacement be done safely with exact RGBA mapping?

   * Implement or sketch a Python/Pillow script that:

     * loads PNGs
     * applies exact RGBA palette replacements
     * preserves canvas size, transparency, and filename structure
     * writes to a generated output directory
     * emits a summary of changed pixel counts per file
   * Avoid fuzzy HSV replacement initially.
   * Optionally add a separate “candidate unknown skin-like colors” report for manual review.

7. Can contact sheets be generated for QA?

   * Build a script that creates side-by-side original vs modified sheets at 4x or 8x zoom.
   * Include checks for image dimensions and alpha preservation.
   * Include a manifest/report of every processed asset.

8. Is AI-assisted palette generation useful?

   * Do not rely on AI output as final game assets.
   * Treat AI as a way to explore target palettes on one representative image.
   * Final mod output should be deterministic palette replacement plus manual cleanup.
   * Document a safe workflow:

     1. export representative portrait
     2. mask skin region manually
     3. use an image editor/model to suggest a target tone
     4. sample resulting colors into a palette ramp
     5. apply via script to all related assets
     6. review contact sheet

Deliverables:

* A short `INVESTIGATION.md` explaining current modding options and recommended path.
* A `TOOLS.md` with install/run notes for Linux/NixOS if possible.
* A minimal scaffold repo layout.
* A proof-of-concept script for deterministic palette replacement.
* A proof-of-concept contact-sheet generator.
* A `.gitignore` that excludes extracted game assets and generated outputs.
* A risk/unknowns section covering:

  * Fields of Mistria version changes before 1.0
  * direct `data.win` editing risks
  * mod loader maturity
  * asset origin/frame metadata risks
  * copyright/distribution constraints

Preferred implementation:
Use Python 3 and Pillow for image processing unless a better lightweight option is found. Keep scripts small, readable, and easy to run. Do not introduce a large framework. If Nix is useful, add a simple `flake.nix` dev shell with Python, Pillow, ImageMagick if needed, and any CLI tools that are already packaged.

Initial command targets:

* `python scripts/apply-palette.py --input extracted/example_character --palette palettes/light/example_character.json --output generated/example_character-light`
* `python scripts/build-contact-sheet.py --original extracted/example_character --modified generated/example_character-light --output generated/contact-sheet-example_character-light.png`
* `python scripts/validate-assets.py --original extracted/example_character --modified generated/example_character-light`

Definition of done:
The task is successful if we know whether the current game build can be modded this way, have one tiny asset replacement proven or clearly blocked, and have a clean deterministic pipeline ready for a larger post-1.0 art pass. Do not attempt a full-character or full-game recolor yet.
