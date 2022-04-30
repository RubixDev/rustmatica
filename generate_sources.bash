#!/bin/bash

# Custom exit function to always delete the temp folder
_exit () {
    if [ "temp" = "$(basename "$(pwd)")" ]; then
        cd .. || exit "$1"
    fi
    rm -rf temp
    exit "$1"
}

# Clone the latest fabric-example-mod into ./temp
git clone https://github.com/FabricMC/fabric-example-mod.git temp || exit 1
# Cd into ./temp
cd temp || _exit 1

# Generate mapped minecraft sources with the CFR decompiler
./gradlew genSourcesWithCfr || _exit 2
# Unzip the generated source jar into ./source-parser/mc_src_cfr
unzip -od ../source-parser/mc_src_cfr .gradle/loom-cache/*/net.fabricmc.yarn.*/minecraft-project-@-merged-named-sources.jar

# Generate mapped minecraft sources with the FernFlower decompiler
./gradlew genSourcesWithFernFlower || _exit 2
# Unzip the generated source jar into ./source-parser/mc_src_ff
unzip -od ../source-parser/mc_src_ff .gradle/loom-cache/*/net.fabricmc.yarn.*/minecraft-project-@-merged-named-sources.jar

# Cd back into root
cd .. || _exit 1
# Remove The ./temp folder
rm -rf temp
