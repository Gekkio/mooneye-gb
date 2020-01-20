#!/bin/bash
set -euo pipefail

gpg --quiet --batch --yes --decrypt --passphrase="$BOOTROM_PASSPHRASE" .github/boot_roms.tar.gpg | tar -C core/bootroms -xv
