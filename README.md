[![GitHub release](https://img.shields.io/github/v/release/AlbanDAVID/Toutui?label=Latest%20Release&cacheSeconds=3600)](https://github.com/AlbanDAVID/Toutui/releases/latest)
# 🦜 Toutui: A TUI Audiobookshelf client for Linux and macOS

<p align="center">
    <em>In French, being "tout ouïe" (toutui) means being all ears.</em>
</p>

<p align="center">
    🚀 <strong>Be toutui and enjoy audiobookshelf from your terminal!</strong>
</p>

<p align="center">
    <img src="assets/demo_3.gif" alt="🎬 Demo">
</p>

<div align="center">
🎨 Explore and try various themes <a href="https://github.com/AlbanDAVID/Toutui-theme">here.</a>
</div>

## ✨ Features  
✅ **Cross-platform** – <img src=".github/tux.png" align="top" width="24" alt="Tux (Linux)"/>  Linux and <img src=".github/apple.png" align="top" width="24" alt="Apple (macOS)"/> macOS    
✅ **Lightweight & Fast** – A minimalist terminal user interface (TUI) written in Rust 🦀  
✅ **Supports Books & Podcasts** – Enjoy both audiobooks and podcasts  
✅ **Sync Progress & Stats** – Keep your listening progress in sync  
✅ **Streaming Support** – Play directly without downloading  
✅ **Customizable Color Theme** – A config file will allow you to customize the color theme. Explore and try various themes [here](https://github.com/AlbanDAVID/Toutui-theme).

## 📰 Media
<img src=".github/korben.png" align="top" width="50" alt="Korben"/> Featured on [Korben](https://korben.info/toutui-client-terminal-audiobookshelf.html), a well-known French tech blog covering open source and technology.


## 🛠️ Roadmap  
**Short-term Goals**  
- Since this is a beta version, the main focus is on tracking and fixing bugs.
- Currently working on the next release: [v0.3.6-beta]

**Mid-term Goals**  
- Put the app on yay    
- CI/CD Implementation  
- Add future features described bellow.

## 🔮 Future features
Here are some features that could be added in future releases:
- Ability to add new podcasts from the app
- Add stats
- Offline mode
  
## ⚠️ Caution: Beta Version  
This beta app is still in **heavy development and contains bugs**.  
❗Please check [here](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md) for known bugs especially **MAJOR BUGS** before using the app, so you can use it with full awareness of any known issues.  
If you encounter any issues that are **not yet listed** in the Issues section or into [known bugs](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md), please **open a new issue** to report them.  

🔐 Although it's a beta version, you can use this app with **minimal risk** to your Audiobookshelf library.  
At worst, you may experience **sync issues**, but there is **no risk** of data loss, deletion, or irreversible changes (API is just used to retrieve books and sync them).

## 📝 Notes
### 🤝 **Contributing**  
Do not hesitate to contribute to this project by submitting your code, ideas, or feedback. Please make sure to read the [contributing guidelines](https://github.com/AlbanDAVID/Toutui/blob/main/CONTRIBUTING.md) first.

### 🎨 **UI**
Explore and share themes [here](https://github.com/AlbanDAVID/Toutui-theme).    
The **font** and **emojis** may vary depending on the terminal you are using.    
To ensure the best experience, it's recommended to use **Kitty** or **Alacritty** terminal.



## 🚨 Installation Instructions

### Easy installation from source

>[!WARNING]
> ❗ For now, Toutui is installed and compiled from source by cloning the stable branch, but the process is simplified thanks to the hello_toutui.sh script.
> - **This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version).**    
> - If you follow all the instructions but installation fails:
>     - Check first the [wiki](https://github.com/AlbanDAVID/Toutui/wiki/Installation-issues).
>     - Otherwise, open an issue.
>     - You can also install the app [manually](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#git) from source.

>[!IMPORTANT]
> <img src=".github/apple.png" align="top" width="24" alt="Apple (macOS)"/> **macOS users**: before install, make sure to have `Homebrew` and `openssl` installed.
> 
> Install Homebrew:    
> ```
>bash /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
> ```
> Install openssl:
>```  
> brew install openssl
>```   

**Proceed with the installation by simply copying and pasting the following code block into your terminal:**    

[![GitHub release](https://img.shields.io/github/v/release/AlbanDAVID/Toutui?label=Latest%20Release&cacheSeconds=3600)](https://github.com/AlbanDAVID/Toutui/releases/latest)
```bash
git clone --branch stable --single-branch https://github.com/AlbanDAVID/Toutui
cd Toutui/
chmod +x hello_toutui.sh
./hello_toutui.sh install
```
>[!TIP]
> - Once the installation is complete, type `toutui` in your terminal to launch the app.
> - Explore themes [here.](https://github.com/AlbanDAVID/Toutui-theme)       
> - Best experience with Kitty or Alacritty terminal.    
  
>[!IMPORTANT]
> If you encounter issues, check the [wiki](https://github.com/AlbanDAVID/Toutui/wiki).

#### **Update**
The script will detect if a new release is available and install it if any.
```bash
./hello_toutui.sh update
```

#### **Notes**  

##### Files installed:
In `/usr/bin` for Linux, or `/usr/local/bin` for macOS:
- `toutui` — The binary file (you can execute it from anywhere).
>[!TIP]
> You can choose a custom location: `./hello_toutui.sh install /example/custom/location`

In `~/.config/toutui` for Linux or `~/Library/Preferences` for macOS:    
**Note**: This is the default path if `XDG_CONFIG_HOME` is empty. 
- `.env` — Contains the secret key.
- `config.toml` — Configuration file.
- `toutui.log` — Log file.
- `db.sqlite3` — SQLite database file.

### Git

>[!WARNING]
> This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version).  

#### **Requirements**
- `Rust`
- `Netcat`
- `VLC`
- `SQLite3`
- `openssl`
- Optional:
- `Kitty terminal emulator` (for a better experience or if you want to use cvlc in the terminal).

*⚠️ If you had to install a package that is not listed above, please open an installation issue.*

#### **Install**    

[![GitHub release](https://img.shields.io/github/v/release/AlbanDAVID/Toutui?label=Latest%20Release&cacheSeconds=3600)](https://github.com/AlbanDAVID/Toutui/releases/latest)
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
>[!TIP] 
> - Best experience with Kitty or Alacritty terminal.    

>[!IMPORTANT]
> If you encounter issues, check the [wiki](https://github.com/AlbanDAVID/Toutui/wiki).

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
