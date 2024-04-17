# Hexen Havoc

Hexen Havoc is a wave based arcade game. Made for the Ludum Dare 55 game jam, unfortunately we did not finish on time.

Written in rust using the bevy game engine.

Art done by Piper McCalman

[Itch.io page](https://oxnh.itch.io/hexen-havoc)

# Keybinds
Key           | Bind
------------- | -------------------------
Esacpe        | Pause / Unpause
WASD          | Move player
Left click    | Move ranged summons to mouse
Right click   | Move melee summons to mouse

# Troubleshooting

## Nvidia Optimus
If you have an NVIDIA Optimus setup on linux you may have some issues.
The game won't start because the GPU it selects is the one which your optimus software has disabled.
I use optimus-manager on my system, to solve this issue I set `pci_remove=yes` in `/etc/optimus-manager/optimus-manager.conf`
The solution should be similiar for alternative optimus software.

## Build Script
The build script builds for targets `x86_64-unknown-linux-gnu` & `x86_64-pc-windows-gnu` by default.

If a target is missing install it with `$ rustup target add`.

`mingw-w64` is required for cross compilation from linux to windows.
