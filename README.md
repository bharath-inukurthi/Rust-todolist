# 🦀 Rust Todo CLI

A simple **command-line Todo application written in Rust**, built as a learning project to understand real Rust application structure beyond tutorials.

This project started because of my friend **Nikhil**, who got me interested in Rust. To get the *full experience*, we’re learning and building on **Debian Linux**, but the tool works on **Windows** as well.

---

## ✨ Features

- 📌 CLI interface using `clap`
- 🗄 Persistent storage using **SQLite** (`rusqlite`)
- ➕ Add tasks
- 📋 List tasks with completion status
- ✅ Mark tasks as done
- ❌ Remove tasks
- ⚠️ Proper error handling using `anyhow`

---

## 🛠 Tech Stack

- **Rust**
- `clap`
- `rusqlite`
- `anyhow`
- **Debian Linux / Windows**

---

## 🦀 Install Rust

### 🐧 Linux (Debian / Ubuntu)

```bash
sudo apt update
sudo apt install -y curl build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal, then verify:

```bash
rustc --version
cargo --version
```

---

### 🪟 Windows

1. Download and install **Rustup**  
   https://rustup.rs

2. During installation:
   - Choose **MSVC toolchain** (recommended)
   - Install **Visual Studio Build Tools** if prompted

3. Verify installation (PowerShell):

```powershell
rustc --version
cargo --version
```

---

## 🚀 Getting Started

### 🔧 Development Mode (using Cargo)

```bash
git clone https://github.com/bharath-inukurthi/Rust-todolist.git
cd Rust-todolist
cargo run -- <command>
```

Example:

```bash
cargo run -- add "Learn Rust"
```

---

### 🚀 User Mode (Direct `todo` Command)

#### Build the release binary

```bash
cargo build --release
```

Binary location:

```
target/release/todo
```

---

## 📦 Install the Binary

### 🐧 Linux (Debian / Ubuntu)

**User-level install**

```bash
mkdir -p ~/.local/bin
cp target/release/todo ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

(Add the PATH export to `~/.bashrc` or `~/.zshrc`.)

**System-wide install**

```bash
sudo cp target/release/todo /usr/local/bin/
```

---

### 🪟 Windows

After building, the binary will be located at:

```
target\release\todo.exe
```

**Option A: Add to PATH**

1. Copy `todo.exe` to:

```
C:\tools\todo\
```

2. Add this folder to **System PATH**
3. Restart PowerShell or CMD

**Option B: Run directly**

```powershell
cd target\release
.\todo.exe add "Learn Rust"
```

---

## 📖 Usage

```bash
todo add "Learn Rust"
todo list
todo done 1
todo remove 1
```

---

## 📂 Project Structure

```
src/
├── main.rs   # CLI command handling
├── db.rs     # Database operations
└── file.rs   # Initialization / utility logic
```

---

## ⚠️ Notes

- This is a **learning-focused project**, not production-ready.
- The SQLite database (`todo.db`) is created locally on first run.
- Advanced features like concurrency and sync are intentionally out of scope.

---

## 🎯 What I Learned

- Rust ownership and error handling
- Structuring CLI applications
- SQLite integration in Rust
- Linux & Windows Rust workflows

---

## 🙌 Acknowledgements

Thanks to **Nikhil** for sparking the interest and learning Rust together.

---

## 📜 License

MIT License
