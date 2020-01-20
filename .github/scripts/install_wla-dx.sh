#!/bin/bash
set -euo pipefail

curl -sSL "https://github.com/${WLA_DX_REPO}/archive/${WLA_DX_COMMIT}.tar.gz" | tar xzv -C "${HOME}"
mv "${HOME}/wla-dx-${WLA_DX_COMMIT}" "${HOME}/wla-dx"
cd "${HOME}/wla-dx"
cmake .
make
