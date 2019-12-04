#!/bin/bash

if [ "$1" == '-?' ] || [ "$1" == '' ] || [ "$1" == '--help' ]
then
   echo "Sets up a new crate modelled on an old crate in this workspace"
   echo ""
   echo "Usage: $(basename $0) pattern-crate-name new-crate-name"
   echo "  example: $(basename $0) example-day day-11-1"
   echo "  example: $(basename $0) day-11-1 day-11-2"
   exit 0
fi

set -o errexit  # exit on error
set -o nounset  # exit if trying to use uninitialised variable

echo Adding new crate $2 modelled on $1

if [[ ! -d $1 ]]
then
    echo "Pattern directory $1 does not exist"
    exit 1
fi

if [[ -d $2 ]]
then
    echo "Destination directory $1 exists already"
    exit 1
fi

cp -R $1 $2
sed "s/$1/$2/g" $1/Cargo.toml > $2/Cargo.toml
sed -i '' "s/\([ \t]*\)\"$1\",/\1\"$1\",\1\"$2\",/" Cargo.toml
