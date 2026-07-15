# rbuild

A lightweight Rust-powered build automation system


`rbuild` automatically builds and installs software directly from Git repositories.

---

## Usage

```bash
rbuild install https://github.com/user/project
```

---

## Workflow

Clone
->
Detect
->
Build
->
Install
->
Register
->
Cleanup

--- 

## Supported Build Systems

**Currently supported:**

- Cargo
- CMake
- Make

**Planned:**
- Meson
- Autotools

--- 

## Features

### Automatic project detection

rbuild detects projects automatically:

```
Cargo.toml
CMakeLists.txt
Makefile
```

### Temporary build workspace

Repositories are cloned into:

```
./project
```

### Binary Installation

Compiled programs are installed into:

```
/usr/local/bin
```
* (this requires sudo)

### Package registry

Installed applications are stored in:

```
~/.rbuild/installed.json
```

---

## Install rbuild

```bash
git clone https://github.com/Kiwilus/rbuild.git && cd rbuild && cargo install --path .
```
**This command clones the repo, changes into it and installs rbuild. You can execute rbuild with ```rb```**

---

## Roadmap

### Planned:
- uninstall command
- update command
- list installed packages
- colorful prints in the terminal
