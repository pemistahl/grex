set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ] || [ $TRAVIS_OS_NAME = osx ]; then
        cargo install --force cross
    else
        rustup target install $TARGET
    fi
}

main
