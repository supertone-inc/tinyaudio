#!/usr/bin/env bash

set -e

if [[ -z $1 ]]; then
    cat VERSION
    exit 0
fi

VERSION=$1
echo $VERSION > VERSION
git commit -a --message "v$VERSION"
