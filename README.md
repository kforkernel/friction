# Rust Shell (`@kforkernel`)

> **⚠️ Work in Progress:** This shell is currently under active development. It is functional with multi-argument support and basic built-ins, though some bugs and incomplete features remain.

A lightweight, Unix-like command-line shell written in Rust. It supports interactive command execution, environment `PATH` resolution, single-quote parsing (including multiline inputs), and core shell built-ins.

---

## ✨ Features

* **Built-in Commands:**
  * `exit` — Exits the shell with status code `0`.
  * `echo` — Prints arguments to standard output.
  * `type` — Identifies whether a command is a shell built-in or an external executable in `PATH`.
  * `pwd` — Prints the current working directory.
  * `cd` — Changes the current directory (defaults to `$HOME` if no directory is passed).
* **External Program Execution:**
  * Resolves binaries across directories listed in the `PATH` environment variable.
  * Supports direct path execution (e.g., `./script.sh` or `/usr/bin/ls`).
  * Automatically falls back to `/bin/sh` if an executable script lacks a proper binary format (`InvalidData`).
* **Input Parsing:**
  * Handles multiple arguments separated by whitespace.
  * Supports single quotes (`'...'`), preserving inner whitespace and prompting with `> ` across multiple lines until the closing quote is entered.

---

## 🚀 Getting Started

### Prerequisites
* [Rust & Cargo](https://rust-lang.org) (latest stable version recommended)
* A Unix-like environment (Linux / macOS)

### Installation & Running

1. **Clone the repository:**
   ```bash
   git clone https://github.com<repo-name>.git
   cd <repo-name>
   ```

2. **Build and run the shell:**
   ```bash
   cargo run
   ```

---

## 💻 Usage Example

```text
\$ pwd
/home/user/projects/rust-shell
\(cd /tmp\) echo 'Hello,
> multiline world!'
Hello,
multiline world!
\$ type echo
echo is a shell builtin
\$ exit
```

---

## 🗺️ Roadmap / Known Limitations

* [ ] Add support for double quotes (`"..."`) and escape characters (`\`).
* [ ] Implement I/O redirection (`>`, `>>`, `<`) and pipelines (`|`).
* [ ] Improve argument tokenization so quoted strings with spaces are passed as single arguments to external binaries.
* [ ] Add command history and tab auto-completion.

---

## 📄 License

Copyright © 2026 **@kforkernel**. All rights reserved.  
Licensed under the **kforkernel Use License (KUL) v1.0**. See [LICENSE.md](LICENSE.md) for terms and conditions.
