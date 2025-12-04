# simgps

iOS Simulator の GPS 位置を操作するインタラクティブ CLI ツール

## インストール

### Homebrew

```bash
brew install aovoq/tap/simgps
```

### Shell script

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aovoq/simgps/releases/latest/download/simgps-installer.sh | sh
```

### Cargo

```bash
cargo install simgps
```

### From source

```bash
cargo install --git https://github.com/aovoq/simgps
```

## 使い方

```bash
# 基本
simgps 35.660118913959366 139.69933489703908

# Google Maps からコピーした座標もそのまま使える
simgps "35.660118913959366, 139.69933489703908"

# ステップサイズを変更 (デフォルト: 10m)
simgps -s 100 35.66 139.69    # 100m ステップ
simgps --step 1 35.66 139.69  # 1m ステップ
```

## 操作キー

| キー | 動作 |
|------|------|
| `↑` | 北へ移動 |
| `↓` | 南へ移動 |
| `→` | 東へ移動 |
| `←` | 西へ移動 |
| `q` / `Esc` | 終了 |

## 必要条件

- macOS
- Xcode Command Line Tools (`xcrun simctl`)
- 起動中の iOS Simulator

## License

MIT
