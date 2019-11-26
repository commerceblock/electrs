#!/bin/bash -x

OCEAN_PASS=$(cat /run/secrets/ocean_pass)
OCEAN_USER=$(cat /run/secrets/ocean_user)

case "$1" in
        cargo)
            echo "Running electrs"
            exec "$@" "--cookie=${OCEAN_USER}:${OCEAN_PASS}"
            ;;
        *)
            "$@"

esac
