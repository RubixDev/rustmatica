#!/bin/bash

_exit () {
    if [ "temp" = "$(basename "$(pwd)")" ]; then
        cd .. || exit "$1"
    fi
    rm -rf temp
    exit "$1"
}

git clone https://github.com/FabricMC/fabric-example-mod.git temp || exit 1
cd temp || _exit 1
cp -v ../DataExtractor.java src/main/java/net/fabricmc/example/ExampleMod.java
./gradlew runServer 2> ../blocks.txt
cd .. || _exit 1
rm -rf temp
git reset -- temp
