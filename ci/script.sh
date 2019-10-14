set -ex

main() {
    local cargo=cargo
    if [ $TRAVIS_OS_NAME = linux ] || [ $TRAVIS_OS_NAME = osx ]; then
      cargo=cross
    fi

    $cargo build --target $TARGET
    $cargo build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    $cargo test --target $TARGET
    $cargo test --target $TARGET --release

    $cargo run --target $TARGET
    $cargo run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
