#!/usr/bin/env bash

# Fail on error
set -e

# Set ${SCRIPT_DIR}
SCRIPT_DIR="$(dirname -- "$(readlink -f -- "${0}")")"

# Set "${FILES}" from first argument or return if none are given
FILES=${@}
[[ "${#FILES[@]}" -eq 0 ]] && {
    printf '%s\n' "ERROR: Please specify at least one file."
    exit 1
}

# Export svgs to pdf
OUTPUT_DIR="${SCRIPT_DIR}"/../out/pdf
mkdir -p ${OUTPUT_DIR}
rsvg-convert -f pdf -o ${OUTPUT_DIR}/output.pdf ${FILES}
