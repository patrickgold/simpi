# Build release packages

Current focus on release targets:
* Windows 10: `x86_64-pc-windows-gnu`
* Linux (Debian/Ubuntu focused): `x86_64-unknown-linux-gnu`

To build and pack all necessary files, just run the `build.sh` script.
It will automatically install the necessary target and for Windows the
`gcc-mingw-w64-x86-64` package (needed for linking the Windows target).

Then, it will produce some `zip`/`tar.gz` archives, which contain all
precompiled binaries, license and readme texts and an install/uninstall script,
for each target there is exactly one archive created.
An sha256 check sum will also be calculated and written for every archive.

End users can then take this archive, unpack it on their target system and
execute the install script, which automatically copies all the files and
adds a menu entry.

**This building system is somewhat buggy and still needs some development, but
it is better than nothing for now.**
