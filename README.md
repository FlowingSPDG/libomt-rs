## libomt-rs

Rust bindings and lightweight wrappers around the Open Media Transport (OMT) C API.

### Overview
- Generates FFI bindings from `libomt.h` at build time.
- Links against the native OMT library shipped in the OpenMediaTransport binary release.
- Includes an example `discovery` program that lists discoverable OMT sources on the LAN.

### Prerequisites
- Windows with Visual Studio 2022 Build Tools (MSVC toolchain)
- Rust (stable, MSVC target)
- OpenMediaTransport Binaries for Windows x64
  - Download from: [OpenMediaTransport Binaries Releases](https://github.com/openmediatransport/libomtnet/releases)

### Quick Start (Windows, recommended)
This repo includes a launcher that initializes the VS Developer environment and sets the OMT variables for you.

1) Place or extract OMT binaries, and note the `Winx64` path (e.g. `C:\Users\<you>\Downloads\OpenMediaTransport.Binaries.Release.v1.0.0.12\Libraries\Winx64`).

2) Edit `dev-cargo.cmd` if needed:
   - By default it sets:
     - `LIBOMT_LIB_DIR` to the `Winx64` path
     - Appends that folder to `PATH` (for DLL loading)
   - If your `.lib` is named differently (e.g. `OpenMediaTransport.lib`), uncomment and set `LIBOMT_LIB_NAME` inside `dev-cargo.cmd` accordingly (name without extension).

3) Run via the launcher:
```bat
dev-cargo run --example discovery --verbose
```

Expected output example:
```text
Discovered sources (1):
DESKTOP-XXXXXXX (vMix - Output 1)
```

### Manual Setup (alternatives)

#### cmd.exe
```bat
set "LIBOMT_LIB_DIR=C:\Path\To\OpenMediaTransport\Libraries\Winx64"
set "PATH=%PATH%;C:\Path\To\OpenMediaTransport\Libraries\Winx64"
rem If the import library has a different name, set it without extension:
rem set "LIBOMT_LIB_NAME=OpenMediaTransport"

"%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 && ^
cargo run --example discovery --verbose
```

#### PowerShell
```powershell
$env:LIBOMT_LIB_DIR = "C:\Path\To\OpenMediaTransport\Libraries\Winx64"
$env:Path += ";C:\Path\To\OpenMediaTransport\Libraries\Winx64"
# Optional if the .lib name differs:
# $env:LIBOMT_LIB_NAME = "OpenMediaTransport"

& "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch:x64 -host_arch:x64 | Out-Null
cargo run --example discovery --verbose
```

### How linking is configured
The build script (`build.rs`) supports the following environment variables:
- `LIBOMT_LIB_DIR` (required): Folder containing the OMT import library (`omt.lib` or `libomt.lib`) and the corresponding DLLs.
- `LIBOMT_LIB_NAME` (optional): Library name without extension if your vendor uses a non-standard name. Examples:
  - `OMT.lib` → `LIBOMT_LIB_NAME=OMT`
  - `OpenMediaTransport.lib` → `LIBOMT_LIB_NAME=OpenMediaTransport`

On MSVC, if `LIBOMT_LIB_NAME` is not provided and `LIBOMT_LIB_DIR` is set, the script auto-detects between `omt.lib` and `libomt.lib`. Otherwise, it defaults to `omt`.

### Troubleshooting
- Link error LNK1181 cannot open input file:
  - Ensure `LIBOMT_LIB_DIR` is set and points to the folder containing the `.lib` file.
  - If your library file is not `omt.lib` or `libomt.lib`, set `LIBOMT_LIB_NAME` accordingly.
- Runtime error DLL not found:
  - Ensure the `Winx64` folder is on your `PATH`, or copy the OMT DLLs next to the produced `discovery.exe` (e.g. `target\debug\examples`).
- MSVC toolchain not found:
  - Use the Developer Command Prompt for VS 2022, or run via `dev-cargo.cmd` which initializes it for you.

### Example code
The example program is in `examples/discovery.rs` and prints discovered OMT sources.

### Upstream and references
- OpenMediaTransport Releases (binaries): [https://github.com/openmediatransport/libomtnet/releases](https://github.com/openmediatransport/libomtnet/releases)
- OpenMediaTransport Organization: [https://github.com/openmediatransport](https://github.com/openmediatransport)
- libomt repository: [https://github.com/openmediatransport/libomt](https://github.com/openmediatransport/libomt)

### License
MIT


