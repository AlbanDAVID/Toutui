# 🦜 Toutui: A TUI Audiobookshelf Client for Linux  
<em>In French, being "tout ouïe" (toutui) means being all ears.</em>

🚀 **Be toutui and enjoy audiobookshelf from your terminal!**  

![🎬 Demo](assets/demo_1.gif)  

## ✨ Features  
✅ **Lightweight & Fast** – A minimalist, terminal-based UI (TUI) written in Rust 🦀  
✅ **Supports Books & Podcasts** – Enjoy both audiobooks and podcasts  
✅ **Sync Progress & Stats** – Keep your listening progress in sync  
✅ **Streaming Support** – Play directly without downloading  
✅ **Customizable Color Theme** – A config file will allow you to customize the color theme  
✅ **VLC Command Line Option** – By default, the VLC GUI will be launched, but you can choose, into the config file, to use VLC in the command line (`cvlc`)


## 🔮 Future Features  
🚧 **Soon: Check the TODO list for upcoming improvements.**  

## ⚠️ Caution: Beta Version  
This beta app is still **hardly in development and contain bugs**.  
**FOR NOW, NOT OPTIMIZED FOR A LIBRARY WITH A LOT OF ITEMS (LONG LOADING TIME)**  
❗Please check [here](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md) for known bugs especially **MAJOR BUGS** before using the app, so you can use it with full awareness of any known issues.  
If you encounter any issues that are **not yet listed** in the Issues section or into [known bugs](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md), please **open a new issue** to report them.  

🔐 Although it's a beta version, you can use this app with **minimal risk** to your Audiobookshelf library.  
At worst, you may experience **sync issues**, but there is **no risk** of data loss, deletion, or irreversible changes (API is just used to retrieve books and sync them).

## 📝 Notes
### 💬 **Discussion**
For general feedback/questions than can't be in issues, use [discussions](https://github.com/AlbanDAVID/Toutui/discussions).

### 🍏 **macOS**
The app is developed and tested on Linux distributions, so it **may be difficult to use on macOS**. However, we are working to make it compatible with macOS. Known issues:  
- `cvlc` option is not available.

### 🎨 **UI**
The **font** and **emojis** may vary depending on the terminal you are using.
To ensure the best experience, it's recommended to use terminals that support **emoji rendering** properly (e.g., Kitty, Alacritty).

### 🎧 **Using `cvlc`**
When using **`cvlc`** (command-line VLC), make sure to use the `shutdown` command when you want to quit the listening session.
This helps ensure that your books are successfully synced and prevents any potential issues.

## 🚨 Installation Instructions

**⚠️ If you follow all the instructions but installation fails, please open an installation issue.**  

### Automatic install
❗It's quite challenging to create a script that is exhaustive for all distributions and OS. If you encounter difficulties, leave an installation issue and install the app [manually](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#git).  
❗This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version).    
❗macOS user: read [this](https://github.com/AlbanDAVID/Toutui/blob/main/README.md#-macos).  

#### **Install**
```bash
git clone https://github.com/AlbanDAVID/Toutui
cd Toutui/
chmod +x hello_toutui.sh
./hello_toutui.sh install
```
Once the installation is complete, type `toutui` in your terminal to launch the app.
#### **Update**
The script will detect if a new release is available and install it if any.
```bash
./hello_toutui.sh update
```

#### **Notes**  

##### Files installed:
in `/usr/bin`
- `toutui` — The binary file (you can execute it from anywhere).

in `~/.config/toutui`
- `.env` — Contains the secret key.
- `config.toml` — Configuration file.
- `toutui.log` — Log file.
- `db.sqlite3` — SQLite database file.

### For Arch Users
🚧 Soon

### Git

❗This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version).  
❗macOS user: read [this](https://github.com/AlbanDAVID/Toutui/blob/main/README.md#-macos).  

#### **Requirements**
- `Rust`
- `VLC`
- `SQLite3`
- `libsqlite3-dev` (for some OS, Debian for example)
- `libssl-dev`
- `Your terminal must support emojis`
- Optional, only if you use cvlc:
- `Netcat`
- `Kitty terminal emulator`

*⚠️ If you had to install a package that is not listed above, please open an installation issue.*

#### **Install**
```bash
git clone https://github.com/AlbanDAVID/Toutui
cd Toutui/
mkdir -p ~/.config/toutui
cp config.example.toml ~/.config/toutui/config.toml
```

Token encryption in the database (<u>**NOTE**</u>: replace `secret`)
```bash
echo TOUTUI_SECRET_KEY=secret >> ~/.config/toutui/.env
```

```bash
cargo run --release
```
#### **Update**

When a new release is available, follow these steps:

The script will detect if a new release is available and install it if any.
```bash
chmod +x hello_toutui.sh
./hello_toutui.sh update
```
OR 
```bash
git pull https://github.com/AlbanDAVID/Toutui
cargo run --release
```

#### **Notes**  
##### Exec the binary:
```bash
cd target/release
./Toutui
```

##### Files installed:
After installation, you will have the following files in `~/.config/toutui`
- `.env` — Contains the secret key.
- `config.toml` — Configuration file.
- `toutui.log` — Log file.
- `db.sqlite3` — SQLite database file.
