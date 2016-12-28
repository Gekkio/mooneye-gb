#!/bin/bash
set -euo pipefail

WLA_DX_COMMIT="d1dcfd6142a71702471580e6ddca24652a63d00f"

curl -sSL "https://github.com/vhelin/wla-dx/archive/${WLA_DX_COMMIT}.tar.gz" | tar xzv -C "${HOME}"
cd "${HOME}/wla-dx-${WLA_DX_COMMIT}"
cmake .
make

mkdir -p "${HOME}/bin"
cp binaries/wla-gb binaries/wlalink "${HOME}/bin"
echo -n "${WLA_DX_COMMIT}" > "${HOME}/bin/wla-dx.version"
