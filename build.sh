
set -e

usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  --windows    Build Windows executable bundle (.exe)"
    echo "  --macos      Build macOS application bundle (.app)"
    echo "  --all        Build for all platforms"
    echo "  --help       Display this help message"
    exit 1
}

if ! command -v cargo-bundle &> /dev/null; then
    echo "cargo-bundle is not installed. Installing now..."
    cargo install cargo-bundle
fi

if [ $# -eq 0 ]; then
    usage
fi

BUILD_WINDOWS=false
BUILD_MACOS=false

while [ $# -gt 0 ]; do
    case "$1" in
        --windows)
            BUILD_WINDOWS=true
            ;;
        --macos)
            BUILD_MACOS=true
            ;;
        --all)
            BUILD_WINDOWS=true
            BUILD_MACOS=true
            ;;
        --help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
    shift
done

if [ "$BUILD_WINDOWS" = true ]; then
    echo "Building Windows executable bundle..."
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        cargo bundle --release --format msi
    else
        echo "Cross-compiling for Windows..."
        echo "Note: This requires the appropriate cross-compilation toolchain."
        echo "If this fails, please build on a Windows machine or use a Windows VM."
        cargo bundle --release --target x86_64-pc-windows-gnu --format msi
    fi
    echo "Windows build completed. Check the 'target/release/bundle/msi/' directory."
fi

if [ "$BUILD_MACOS" = true ]; then
    echo "Building macOS application bundle..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        cargo bundle --release --format osx
    else
        echo "Cross-compiling for macOS..."
        echo "Note: This requires the appropriate cross-compilation toolchain."
        echo "If this fails, please build on a macOS machine or use a macOS VM."
        cargo bundle --release --target x86_64-apple-darwin --format osx
    fi
    echo "macOS build completed. Check the 'target/release/bundle/osx/' directory."
fi

echo "Build process completed!"
