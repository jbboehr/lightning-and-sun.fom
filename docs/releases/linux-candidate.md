# Linux candidate notes

**Unpublished candidate.** The supported installation route is x86_64 Linux
with Nix and the packaged MOMI 0.16.4 sandbox. Start with the
[build and installation instructions](../../README.md#build-and-install).
The project is licensed AGPL-3.0-only; see the [license notice](../../README.md#license).
No release tag or public download is assigned by these notes.

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

The candidate includes fixes for title-screen palette initialization, palette
application to native cameo actors, and the lifetime of menu notifications.
The current fourteen-choice build passes native title/journal keyboard/mouse and
virtual-gamepad navigation, wrapping, independent selections, remembering/restart,
reset and save-failure feedback. Both additions have sampled portraits and world
sprites rendered in the normal prologue. Earlier basic movement, dialogue and
save/load results remain recorded separately. Virtual-gamepad results do not
establish physical-controller behavior.

Installation checks generated pixels, metadata and scripts before replacing
the archive. Removal restores the exact pre-install archive, including earlier
compatible mods. See [removal and recovery](../../README.md#remove-or-change-the-mod)
and [compatibility requirements](../../README.md#compatibility) before changing
recipes, updating the game or managing other mods.

## Tested scope and remaining limits

Verification used the preserved game archive identified in the
[compatibility instructions](../../README.md#compatibility) and supplied Linux
executable. Other game builds, arbitrary mod combinations, ARM installation,
Windows, macOS and portable non-Nix bundles have no supported route here.
Same-machine Nix builds do not establish a fresh operating-system test or
compatibility with every Linux distribution and hardware configuration.

Representative live samples exercised a season boundary, seasonal portraits and
world actors, Juniper's Beach→Summer change, Elsie's Winter writing, Terithia's
fishing and native West-facing rendering, and special-scene actors. Calendar,
clock and travel preparation used built-in debug tools. Normal controls performed
the observed gameplay after preparation; special scenes used debug-triggered
access, which does not establish ordinary story eligibility or progression.
The live Priestess sample was intentionally unchanged spectral art, not
recolorable Flashback Priestess.

Displayed artwork, representative Player 27 portraits and the displayed live
samples have qualified owner acceptance, with inspection depth unspecified.
This is not an exhaustive artwork or gameplay review. Physical-controller
acceptance, distinct complex-animation end-stage dispatch, unsampled forms and
actions, and broader approximate-tone coverage remain open. The usable Linux
release milestone and publication authorization remain open.

The refreshed private source snapshot includes these notes and current
verification documentation. Its [finalization record](../development/linux-candidate-finalization.md)
links the archive, per-file manifest and external checksums/validation verdict.
It records the current twelve-choice collection alongside two synthetic folder
mods, bounded refusal checks and exact removal; arbitrary mod compatibility is
not inferred. Older source snapshots remain preserved as historical evidence.

The [Stillwell/Terithia rollout](../development/muted-tone-rollout.md) is
implemented and technically verified across all 36 characters, preserving every
previous choice and generated image. Its current private source snapshot passes
standalone-build, strict installation, bounded compatibility/refusal, native
input/persistence and exact-removal checks. The displayed rollout review has
provisional owner acceptance with "looks fine for now"; inspection depth is
unspecified. Broader tone coverage assessment remains pending, and the earlier
finalized candidate above stays frozen. Live checks used a separately bound copy
omitting exactly three synthetic CPU-test probe payloads; all palette assets and
other entries match the installed
archive. This supplies no live compatibility claim for those synthetic mods.
