#!/bin/bash -x

if [ -f /run/secrets/ocean_pass ]; then
    OCEAN_PASS=$(cat /run/secrets/ocean_pass)
fi

if [ -f /run/secrets/ocean_user ]; then
    OCEAN_USER=$(cat /run/secrets/ocean_user)
fi

case "$1" in
        cargo)
            echo "Running electrs"
            exec "$@" "--cookie=${OCEAN_USER}:${OCEAN_PASS}"
            ;;
        *)
            "$@"

esac
