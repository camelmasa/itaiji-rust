# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage= \
          ext=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ext=so
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ext=dylib
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --target $TARGET --release

    cp target/$TARGET/release/libitaiji.$ext $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
