# simgps

Interactive CLI tool for controlling iOS Simulator GPS location

## Installation

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

## Usage

```bash
# Basic
simgps 35.660118913959366 139.69933489703908

# Coordinates copied from Google Maps work as-is
simgps "35.660118913959366, 139.69933489703908"

# Change step size (default: 10m)
simgps -s 100 35.66 139.69    # 100m step
simgps --step 1 35.66 139.69  # 1m step
```

## Controls

| Key | Action |
|-----|--------|
| `↑` | Move North |
| `↓` | Move South |
| `→` | Move East |
| `←` | Move West |
| `q` / `Esc` | Quit |

## Requirements

- macOS
- Xcode Command Line Tools (`xcrun simctl`)
- Running iOS Simulator

## License

MIT
