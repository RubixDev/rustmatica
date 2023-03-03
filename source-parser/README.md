# rustmatica: minecraft-source-parser

A script to extract information about entities from Minecraft's source code

Getting the list of properties per Entity with their respective type is not as
easy to get as the list of block properties. While the block properties can
easily be accessed from running Java with a simple Mod, the Entity NBT
properties can only really be found programmatically by traversing the
decompiled Minecraft source code.

This script tries its best to do that. It is written in TypeScript, because it
had the best and most active Java parsing library I could find.

Before running this with `npm start`, the Minecraft sources must be provided by
running the `generate_sources.sh` script in the repository root. After running,
a `entityData.json` file will be created, which can then be read by
`make_lists.py`.
