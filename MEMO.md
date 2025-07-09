
# Memo

## Creating a Release

This project uses GitHub Actions to automatically create releases when tags are pushed.

### Steps:

1. **Create a tag:**
   ```bash
   git tag v1.0.0
   ```

2. **Push the tag:**
   ```bash
   git push origin v1.0.0
   ```

3. **Automatic process:**
   - GitHub Actions will automatically trigger the release workflow
   - Binaries will be built for all supported platforms
   - A new Release will be created with the binaries attached

### Generated Artifacts:

- `mypomodoro-linux-x86_64` - Linux x86_64 binary
- `mypomodoro-windows-x86_64.exe` - Windows MSI installer bundle
- `mypomodoro-macos-x86_64` - macOS Intel .app bundle
- `mypomodoro-macos-aarch64` - macOS Apple Silicon .app bundle

### Tag Format:

Use semantic versioning with `v` prefix:
- `v1.0.0` - Major release
- `v1.0.1` - Patch release
- `v1.1.0` - Minor release
- `v2.0.0-beta.1` - Pre-release

### Workflow File:

The release automation is configured in `.github/workflows/release.yml`

## Local Testing with act

You can test the GitHub Actions workflow locally using `act`.

### Usage:

```bash

# Test the entire workflow (dry run)
act push --container-architecture linux/amd64 --dryrun

# Test only the Linux build job
act push -j build --container-architecture linux/amd64

# Note: Windows and macOS jobs will be skipped in local testing
# Only Linux builds can be tested locally with act
```

### Important Notes:

- **GitHub Actions (production)**: Creates all 4 binaries (Windows, macOS Intel, macOS Apple Silicon, Linux) when you push a tag
- **Local testing with act**: Can only test Linux builds due to Docker limitations
- **Windows/macOS builds**: Will work perfectly on GitHub's actual runners, just can't be tested locally