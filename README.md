# UE4SS for Brickadia

This tool patches Brickadia to add support for UE4SS, allowing you to run lua/blueprint mods and custom scripts.

## NOTICE

1. You will not make unsafe systems that can be used to distribute malware automatically.
2. You will not use this to compete with current or future Brickadia Business.
3. You will not create or help people make cheats, trolling tools, or nuisance mods.
4. You will not post content that do not work for the vanilla game to the gallery or host servers that do not work with vanilla clients to server list.
5. You will not complain if this tool stops working in a future game version.
6. You will not submit crash reports that are caused by this tool or mods.

## Usage

1. [Download the latest release](https://github.com/brickadia-community/br-lua-patcher/releases)
2. Re-read 👀 the [NOTICE](#notice)
3. Run `br_patcher.exe`
4. Enjoy!

<!--
TODO: Finish setting up CI based on https://github.com/UE4SS-RE/RE-UE4SS/blob/main/.github/workflows/release.yml
TODO: CI builds ue4ss, rust, and builds the cli to a release binary.
TODO: only run on tags
-->
<!-- Download and run the binary from releases to patch your brickadia game/server steam binaries. -->

Setup is manual at the moment...

## Manual Setup

This project depends on compiling UE4SS from source for the dlls. The CI will automatically do this but due to licensing, the complete source cannot be provided here.

### 1. Setup Accounts

Create a Github and Epic account. Link them at [epicgames.com](https://www.epicgames.com/account/connections).

This is required to access [Re-UE4SS/UEPseudo](https://github.com/Re-UE4SS/UEPseudo/), a fork of [EpicGames/UnrealEngine](https://github.com/EpicGames/UnrealEngine), required to compile the UE4SS binaries.

After you've linked your account, [generate an ssh-key](https://www.ssh.com/academy/ssh/keygen) (I recommend `-t ed25519`) and [add it to your github account](https://github.com/settings/keys). This is required to git-clone-submodules from the private repositories.

### 2. Compile UE4SS

Skip this step by downloading the `dwmapi.dll` and `UE4SS.dll` from the [latest release](https://github.com/brickadia-community/br-lua-patcher/releases) and copying them to the `assets` directory.

1. Download [Visual Studio 2022](https://visualstudio.microsoft.com/vs/) and install a version of MSVC that supports C++23 (Desktop C++)
2. Install [Rust](https://www.rust-lang.org/tools/install)
3. Download [xmake](https://xmake.io/guide/quick-start.html#windows) and add to a folder in the a directory in your PATH [environment variable](https://www.howtogeek.com/787217/how-to-edit-environment-variables-on-windows-10-or-11/)
4. Download [git](https://git-scm.com/downloads/win)
5. `git clone git@github.com:UE4SS-RE/RE-UE4SS.git` (and `cd` to this dir)
7. `git submodule update --init --recursive`
<!-- This is supposed to work too -->
<!-- `xmake f -m "Game__Shipping__Win64" -y` -->
<!-- `xmake build` -->
8. (Optional?) Copy [`raw_pbd_xmake.lua`](./raw_pdb_xmake.lua) to `RE-UE4SS/deps/third-repo/packages/r/raw_pdb/xmake.lua`
9. `xmake project -k vsxmake2022 -m "Game__Shipping__Win64`
10. Open `vsxmake2022/RE-UE4SS.sln` in Visual Studio
11. In the "Build" menu, select "Build UE4SS" to compile the UE4SS binaries (This may take a while)
12. Copy `RE-UE4SS/Binaries/Game__Shipping__Win64/proxy/dwmapi.dll` to `assets/dwmapi.dll`
13. Copy `RE-UE4SS/Binaries/Game__Shipping__Win64/UE4SS/UE4SS.dll` to `assets/UE4SS.dll`

### 3. Compile and Run

1. `cargo run` (You should already have Rust installed from step 2)
2. Re-read 👀 the [NOTICE](#notice)
3. select Y to patch Brickadia and copy the files