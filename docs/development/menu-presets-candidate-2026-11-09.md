# Menu-presets candidate, 2026-11-09

The owner asked for the release candidate to be rebuilt after the
[palette presets](menu-presets.md) were committed. The private candidate was
resealed from that commit. It also takes in the three earlier changes of the
same day that the previous candidate predated: the end-user README, the
[warning for changed artwork](source-hash-override.md) and the
[install route without Nix](cli-installer.md#target-and-dependency-handling).
No publication or tag occurred.

## Verification

The package built from the commit passed all 473 regular tests in its Nix
build. It is the same store path as the package used for the
[preset live run](menu-presets.md#live-run-2026-11-09), so that run's evidence
applies to this candidate: a bare `install` of the complete collection into the
isolated copy of the game archive passed the installer's pixel and
animation-metadata check, the six preset rows behaved as tabulated in the
running game, and `uninstall` restored the archive and `Maybe.toml` byte for
byte. The local [install log](../../tmp/menu-presets/install.stderr) and
[before](../../tmp/menu-presets/lab-before.sha256) checksums record the
install; the removal was checked against the same file.

The source snapshot was sealed with the unchanged text-only allowlist, extracted
into a fresh tree, and built standalone from its locked flake. This record and
the planning line are outside the package's filtered source, so the sealed tree
builds the same store path as the package that was installed. The external
[summary](../../tmp/menu-presets-candidate/source-candidate-summary.json) and
[standalone receipt](../../tmp/menu-presets-candidate/standalone-verification.json)
own the archive checksum and verdict, since an archive cannot contain its own
checksum.

## Limits

Same machine and store, and no fresh-OS or uncached build. The route without Nix
was not rerun for this candidate; its earlier
[record](cli-installer.md#target-and-dependency-handling) stands. The live run
covered the preset rows from the title screen only. The
[licensed candidate](licensed-candidate-2026-11-08.md) limits on visual and live
acceptance still apply, and the owner playthrough remains open.
