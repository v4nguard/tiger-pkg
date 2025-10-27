# Tiger PKG Library

[![Latest version](https://img.shields.io/crates/v/tiger-pkg.svg)](https://crates.io/crates/tiger-pkg)
[![Documentation](https://docs.rs/tiger-pkg/badge.svg)](https://docs.rs/tiger-pkg)
![Discord](https://img.shields.io/discord/948590455715684393?label=v4nguard%20discord&color=%2377aaff)

You need an oo2core DLL to be able to decompress packages.
When using tiger-pkg with a Destiny 2/Marathon installation, PackageManager will automatically search for oo2core
under `bin\x64`.

In any other case, you will need to get oo2core_3_win64.dll from somewhere (an old game for example), and place it in
the
directory where you run tiger-pkg from. Check below for the version of oo2core that is required for your game.

On Linux, liblinoodle is used for oo2core_3 support (which requires the respective Windows DLL), while liboodle-data-shared.so is used for oo2core_9 support (which does not require the Windows DLL and can be used on it's own). `liboodle-data-shared.so` can be downloaded [here](https://github.com/WorkingRobot/OodleUE/releases/latest) (you will want to download clang.zip or gcc.zip)

## Package format support

| Version                         | Platform          | Works? | Oodle Library |
| ------------------------------- | ----------------- | ------ | --------- |
| Destiny Internal Alpha          | X360              | ✅      | `oo2core_3_win64.dll` |
| Destiny Legacy (The Taken King) | PS3/X360/PS4/XONE | ✅      | `oo2core_3_win64.dll` |
| Destiny (Rise of Iron)          | PS4/XONE          | ✅      | `oo2core_3_win64.dll` |
| Destiny 2 (Beta)                | Any               | ✅      | `oo2core_3_win64.dll` |
| Destiny 2 (Pre-BL)              | Any               | ✅      | `oo2core_3_win64.dll` |
| Destiny 2 (Post-BL)             | Any               | ✅      | `oo2core_9_win64.dll` / `liboodle-data-shared.so` |
| Marathon                        | Any               | ✅      | `oo2core_9_win64.dll` / `liboodle-data-shared.so` |

## Game version codes

### Destiny

| Version               | Code          |
| --------------------- | ------------- |
| DestinyInternalAlpha  | `d1_devalpha` |
| DestinyFirstLookAlpha | `d1_fla`      |
| DestinyTheTakenKing   | `d1_ttk`      |
| DestinyRiseOfIron     | `d1_roi`      |
| Destiny2Beta          | `d2_beta`     |
| Destiny2Forsaken      | `d2_fs`       |
| Destiny2Shadowkeep    | `d2_sk`       |
| Destiny2BeyondLight   | `d2_bl`       |
| Destiny2WitchQueen    | `d2_wq`       |
| Destiny2Lightfall     | `d2_lf`       |
| Destiny2TheFinalShape | `d2_tfs`      |
| Destiny2TheEdgeOfFate | `d2_eof`      |

### Marathon

| Version       | Code       |
| ------------- | ---------- |
| MarathonAlpha | `ma_alpha` |

## Platform codes

| Platform      | Code      |
| ------------- | --------- |
| Xbox 360      | `x360`    |
| Xbox One      | `xboxone` |
| PlayStation 3 | `ps3`     |
| PlayStation 4 | `ps4`     |
| Windows (x64) | `w64`     |
