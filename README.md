<img width="730" height="310" alt="image" src="https://github.com/user-attachments/assets/496db0c2-8b60-4930-b887-394ff8f50c3f" />

# VampireCrawlers Save Editor

A small CLI save editor for **Vampire Crawlers** (Nosebleed Interactive), written in Rust.

Allows reading and modifying coin balance and unlocked characters in `SaveProfile*.save` files.

## Features

- Read current profile state (coins + character unlocks).
- Set `TotalCoins` (updates both `ProfileSaveData` and `ProgressionSaveData`).
- Unlock one or more characters by short name.
- Idempotent: re-running with the same arguments does not duplicate entries.
- Auto-detects default save path on Windows (`%USERPROFILE%\AppData\LocalLow\Nosebleed Interactive\Vampire Crawlers\Save\SaveProfile0.save`).

## Build

Requires Rust (edition 2021+).

```bash
cargo build --release
```

The binary will be at `target/release/vamp-crawler.exe`.

## Usage

### Read-only (no modifications)

```bash
cargo run
```

Outputs current coins and a checklist of all characters with their unlock status.

### Set coin balance

```bash
cargo run -- --coins 100000
```

### Unlock characters

Pass one or more short names:

```bash
cargo run -- --add-character Pugnala Giovanna Poe
```

### Combine flags

```bash
cargo run -- --coins 999999 --add-character Concetta MissingNo
```

### Custom save path

```bash
cargo run -- --path "C:\path\to\SaveProfile0.save" --coins 50000
```

### Help

```bash
cargo run -- --help
```

## Supported characters

| Short name | Internal achievement key                |
| ---------- | --------------------------------------- |
| Antonio    | `SimpleAchievement_Character_Antonio`   |
| Pugnala    | `SimpleAchievement_Character_Pugnala`   |
| Giovanna   | `SimpleAchievement_Character_Giovanna`  |
| Concetta   | `SimpleAchievement_Character_Concetta`  |
| Poppea     | `SimpleAchievement_Character_Poppea`    |
| MissingNo  | `SimpleAchievement_Character_MissingNo` |
| Gennaro    | `MetricAchievement_Character_Gennaro`   |
| Arca       | `MetricAchievement_Character_Arca`      |
| Poe        | `MetricAchievement_Character_Poe`       |
| Dommario   | `MetricAchievement_Character_Dommario`  |
| Ramba      | `MetricAchievement_Character_Ramba`     |
| Porta      | `MetricAchievement_Character_Porta`     |
| Mortuccio  | `MetricAchievement_Character_Mortuccio` |
| Cavallo    | `MetricAchievement_Character_Cavallo`   |
| Krochi     | `MetricAchievement_Character_Krochi`    |
| Clerici    | `MetricAchievement_Character_Clerici`   |
| OSole      | `MetricAchievement_Character_OSole`     |
| Christine  | `MetricAchievement_Character_Christine` |

## How it works

The save file is a JSON document containing:

- `Data.ProfileSaveData.TotalCoins` and `Data.ProgressionSaveData.TotalCoins` (mirrored fields).
- `Data.ProgressionSaveData.AchievementsUnlocked`: array of `{ "Key": "...", "Value": true }` objects. Character unlocks live here.
- `Checksum`: a base64-encoded hash over `Data` used by the game to detect tampering.

The game's checksum validation only runs when the field is non-empty. Setting `Checksum` to an empty string bypasses validation, so the editor does not need to recompute the hash after modifying fields.


## Warning

This tool modifies game save files in place. Back up your save before using:

```bash
copy "%USERPROFILE%\AppData\LocalLow\Nosebleed Interactive\Vampire Crawlers\Save\SaveProfile0.save" SaveProfile0.save.backup
```

Use at your own risk.
