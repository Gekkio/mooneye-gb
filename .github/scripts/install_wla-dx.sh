#!/bin/bash
set -euo pipefail

CACHED_VERSION=`cat ${HOME}/bin/wla-dx.version || true`

if [ "${CACHED_VERSION}" = "${WLA_DX_COMMIT}" ]; then
  echo "Using cached wla-dx binaries"
  exit 0
fi

curl -sSL "https://github.com/${WLA_DX_REPO}/archive/${WLA_DX_COMMIT}.tar.gz" | tar xzv -C "${HOME}"
cd "${HOME}/wla-dx-${WLA_DX_COMMIT}"
cmake .
make

mkdir -p "${HOME}/bin"
cp binaries/wla-gb binaries/wlalink "${HOME}/bin"
echo -n "${WLA_DX_COMMIT}" > "${HOME}/bin/wla-dx.version"
