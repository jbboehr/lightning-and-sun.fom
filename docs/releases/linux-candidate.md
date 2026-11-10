# Linux release notes

**Early release.** The supported installation route is x86_64 Linux with Nix and
the packaged MOMI 0.16.4 sandbox. Start with the
[build and installation instructions](../../README.md#build-and-install).
The project is licensed AGPL-3.0-only; see the [license notice](../../README.md#license).

## Release contents

The release contains the tool, text recipes and masks, and mod scripts.
It includes neither original game assets nor recolored, game-derived assets.
Replacement images are generated locally from each user's own game copy.
Exported assets, generated replacements, image galleries, screenshots and clips
are excluded from release files and attachments.

## Recipe coverage

The recipes support 36 characters: 2,201 portrait strips and 4,111 recolorable
world strips across their authored seasons, outfits and actions. These counts
describe assets processed locally from the user's copy, not images shipped.
Thirty ground-shadow strips and eighteen spectral strips intentionally retain
their original appearance.

Each character has fourteen choices: Vanilla, Debug Blue, three retained donor
palettes and nine additional NPC/creator tones, including Deep umber (Player 27),
Muted light (Stillwell) and Muted medium (Terithia).
The [palette controls](../../README.md#palette-controls) list the additions and
character-specific donors. These are representative tones; they do not reproduce
every native skin color exactly.

Portrait refinements address skin transitions around Reina's mouth and facial
boundaries on Adeline, Celine and Holt. Existing material rules preserve clothing,
hair, equipment, scars and markings; recolorable glow retains its adapted shading.
Intentional spectral forms remain unchanged.

## Controls and installation behavior

Open **Settings → Palettes** from the title screen or journal. One character's
choice applies to both portraits and world sprites. All characters default to
Vanilla. **Remember choices** starts Off; enabling it restores saved selections
in later sessions. **Reset all to Vanilla** resets the included characters.
Persistence failures display a warning while leaving session choices usable.

Installing without a selection installs the complete collection. Installation
checks generated pixels, metadata and scripts before replacing the archive.
Removal restores the exact pre-install archive, including earlier compatible
mods. See [removal and recovery](../../README.md#remove-or-change-the-mod)
and [compatibility requirements](../../README.md#compatibility) before changing
recipes, updating the game or managing other mods.

## Tested scope and known limits

Testing used one game build, identified in the
[compatibility instructions](../../README.md#compatibility), on one Linux
machine. Other game builds, arbitrary mod combinations, ARM installation,
Windows, macOS and portable non-Nix bundles are not supported.

Tested in the running game:

- the palette menu from the title screen and journal, with keyboard, mouse and
  a virtual gamepad, including remembered choices, reset and save-failure
  warnings;
- basic movement, dialogue, room changes, day changes and save/load;
- representative samples of a season change, seasonal portraits and sprites,
  Juniper's Beach to Summer change, Elsie's Winter writing, Terithia's fishing
  and special-scene characters.

Known limits:

- Physical controllers are untested.
- Not every form, action and story scene has been seen in play. Some sampled
  scenes were reached with debug tools rather than normal story progression.
- The final frames of some complex animations were checked only in a prepared
  test scene.
- Compatibility with other mods was tested only with simple folder mods.
