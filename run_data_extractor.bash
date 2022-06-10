#!/bin/sh

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

# Copy the DataExtracttor.java into the mod sources
cp -v ../DataExtractor.java src/main/java/net/fabricmc/example/ExampleMod.java
# Run the mod and write stderr into data.txt
./gradlew runServer 2> ../data.txt

# Cd back into root
cd .. || _exit 1
# Remove The ./temp folder
rm -rf temp
