#!/bin/bash -e

NAME="wikijs-rs"
DESTDIR=/usr/bin
# DOCDIR=/usr/share/man/

TAG=$(git describe --tags --abbrev=0)
VERSION=${TAG:1}

echo "checkout tag ${TAG}"
git checkout --quiet "${TAG}"

declare -A TARGETS
TARGETS["amd64"]="x86_64-unknown-linux-musl"
# TARGETS["arm64"]="aarch64-unknown-linux-gnu"
# TARGETS["armhf"]="arm-unknown-linux-gnueabihf"

for ARCH in "${!TARGETS[@]}"; do
    echo "building ${ARCH} package:"

    DEB_TMP_DIR="${NAME}_${VERSION}_${ARCH}"
    DEB_PACKAGE="${NAME}_${VERSION}_${ARCH}.deb"

    TARGET=${TARGETS[$ARCH]}
    echo " -> compiling ${TARGET} binary"
    cargo build --target "${TARGET}" --release --features="cli vendored-tls"
    # cross build --target "${TARGET}" --release

    echo " -> creating directory structure"
    mkdir -p "${DEB_TMP_DIR}"
    mkdir -p "${DEB_TMP_DIR}${DESTDIR}"
    # mkdir -p "${DEB_TMP_DIR}${DOCDIR}"
    # mkdir -p "${DEB_TMP_DIR}${DOCDIR}/man1"
    # mkdir -p "${DEB_TMP_DIR}${DOCDIR}/man5"
    mkdir -p "${DEB_TMP_DIR}/DEBIAN"
    mkdir -p "${DEB_TMP_DIR}/usr/share/doc/${NAME}"
    # mkdir -p "${DEB_TMP_DIR}/usr/share/bash-completion/completions/"
    # mkdir -p "${DEB_TMP_DIR}/usr/share/fish/vendor_completions.d/"
    # mkdir -p "${DEB_TMP_DIR}/usr/share/zsh/vendor-completions/"
    chmod 755 -R "${DEB_TMP_DIR}"

    echo " -> copy executable"
    cp "target/${TARGET}/release/wikijs" "${DEB_TMP_DIR}${DESTDIR}/wikijs"
    chmod 755 "${DEB_TMP_DIR}${DESTDIR}/wikijs"

    # echo " -> compress man pages"
    # gzip -cn9 target/man/wikijs.1 > "${DEB_TMP_DIR}${DOCDIR}man1/wikijs.1.gz"
    # chmod 644 "${DEB_TMP_DIR}${DOCDIR}"/**/*.gz

    # echo " -> copy completions"
    # cp completions/bash/wikijs "${DEB_TMP_DIR}/usr/share/bash-completion/completions/"
    # cp completions/fish/wikijs.fish "${DEB_TMP_DIR}/usr/share/fish/vendor_completions.d/"
    # cp completions/zsh/_wikijs "${DEB_TMP_DIR}/usr/share/zsh/vendor-completions/"

    echo " -> create control file"
    touch "${DEB_TMP_DIR}/DEBIAN/control"
    cat > "${DEB_TMP_DIR}/DEBIAN/control" <<EOM
Package: ${NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6
Maintainer: Sandro-Alessio Gierens <sandro@gierens.de>
Description: CLI client for the Wiki.js
 wikijs is a CLI client for the Wiki.js written in Rust.
EOM
    chmod 644 "${DEB_TMP_DIR}/DEBIAN/control"

    # echo " -> copy changelog"
    # cp CHANGELOG.md "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog"
    # gzip -cn9 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog" > "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz"
    # rm "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog"
    # chmod 644 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz"

    echo " -> create copyright file"
    touch "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright"
    cat > "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright" << EOM
Format: http://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: ${NAME}
Upstream-Contact: Sandro-Alessio Gierens <sandro@gierens.de>
Source: https://github.com/gierens/wikijs-rs

Files: *
License: AGPL3
Copyright: 2023 Sandro-Alessio Gierens <sandro@gierens>

License: AGPL3
EOM
    cat LICENSE | sed 's/^/ /g' >> "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright"
    chmod 644 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright"

    echo " -> build ${ARCH} package"
    dpkg-deb --build --root-owner-group "${DEB_TMP_DIR}" > /dev/null

    echo " -> cleanup"
    rm -rf "${DEB_TMP_DIR}" "${ARCH}.tar.gz" "${NAME}"

    # gierens: this does not work on my arch at the moment and
    #          i'm verifying on the repo host anyway thus the || true
    echo " -> lint ${ARCH} package"
    lintian "${DEB_PACKAGE}" || true
done
