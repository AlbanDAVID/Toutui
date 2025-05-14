#!/usr/bin/env bash
# Install Toutui and dependencies automagically.

# For test from AlbDav55 fork
# bash -c 'tmpfile=$(mktemp) && curl -LsSf https://github.com/AlbanDAVID/Toutui/raw/install_with_cargo/hello_toutui.sh -o "$tmpfile" && bash "$tmpfile" install && rm -f "$tmpfile"'

# For test from AlbanDAVID toutui repo (stable branch)
# bash -c 'tmpfile=$(mktemp) && curl -LsSf https://github.com/AlbanDAVID/Toutui/raw/stable/hello_toutui.sh -o "$tmpfile" && bash "$tmpfile" install && rm -f "$tmpfile"'


set -eo pipefail

main() {
    do_not_run_as_root

    # Url variables for tests in AlbDav55 fork
   # url_config_file="https://github.com/AlbDav55/Toutui/raw/main/config.example.toml"
   # url_latest_release="https://api.github.com/repos/AlbDav55/Toutui/releases/latest"
   # url_latest_binary="https://github.com/AlbDav55/Toutui/releases/download"
   # url_cargo_install="https://github.com/AlbDav55/Toutui"
   # url_toutui_desktop="https://raw.githubusercontent.com/AlbanDAVID/Toutui/install_improvement/curl/toutui.desktop"

    # URL variables for production (do not forget to ensure that repo name and branches are correct)
    url_config_file="https://github.com/AlbanDAVID/Toutui/raw/stable/config.example.toml"
    url_latest_release="https://api.github.com/repos/AlbanDAVID/Toutui/releases/latest"
    url_latest_binary="https://github.com/AlbanDAVID/Toutui/releases/download"
    url_cargo_install="https://github.com/AlbanDAVID/Toutui"
    url_toutui_desktop="https://raw.githubusercontent.com/AlbanDAVID/Toutui/stable/linux/toutui.desktop"

    # Grab essential variables
    OS=$(identify_os)
    USER=${USER:-$(grab_username)}
    HOME=${HOME:-$(grab_home_dir)}
    CONFIG_DIR="${XDG_CONFIG_HOME:-$(grab_config_dir)}/toutui"
    INSTALL_DIR="${2:-$(grab_install_dir)}"

    # if gsed is needed on macos
    sed() {
        if [[ "$OS" == "macOS"  ]]; then
            command gsed "$@"
        else
            command sed "$@"
        fi
    }

    load_dependencies
    load_exit_codes

    # Adjust script to OS
    case $OS in
        linux) DISTRO="$(get_distro)";;
        macOS) DISTRO="hungry for apples?";;
        *)     install_from_source;;
    esac

    case $1 in
        --install|install) install_toutui && exit $EXIT_OK || exit $EXIT_FAIL;;
        --update|update) update_toutui && exit $EXIT_OK || exit $EXIT_FAIL;;
        --uninstall|uninstall) uninstall_toutui && exit $EXIT_OK || exit $EXIT_FAIL;;
        *) usage "INCORRECT_ARG";;
    esac
}

load_dependencies() {
    # Hard Coded dependencies here.
    # os:package_to_install(:cmd)?
    HC_DEPS=(
        arch:gnu-netcat:netcat \
        #centos:libsqlite3-dev:no_check \
        centos:nc \
        *centos:epel-release \
        debian:netcat \
        #debian:libsqlite3-dev:no_check \
        #debian:libssl-dev:no_check \
        fedora:nc \
        linux:curl \
        *linux:kitty \
        #linux:pkg-config \
        #linux:sqlite3 \
        linux:vlc  \
        macOS:curl \
        *macOS:kitty \
        macOS:netcat \
        macOS:gsed \
        #%macOS:openssl \
        #macOS:pkg-config \
        #macOS:sqlite3 \
        macOS:vlc \
        opensuse:netcat \
    )
    # Format: <OS|distrubtion>:<package_name>[:<cmd>|no_check]
    #
    # Dependencies starting with '*' are optional
    # Dependencies starting with '%' are forced
    #
    # 'linux:'  = for all linux distros
    # 'macOS:'  = macOS specific
    # 'debian:' = debian distribution specific
    # See also 'arch:', 'fedora:', 'opensuse:', 'centos:'
    #
    # (optional) ':<cmd>'
    # (optional) ':no_check'
    # INFO: Use either ':<cmd>' or ':no_check'
    #
    # By default, this script uses the package name
    # as the program name to check if the package is
    # installed. This is not sound for all packages.
    # For example: verifying "libsqlite3-dev" is
    # installed by launching "libsqlite3-dev" is
    # meaningless.
    #
    # ':<cmd>' = use <cmd> command to check for the
    # package installation on the system.
    #
    # ':no_check' = do not check whether package is
    # installed. Avoids being warned about a missing
    # dependency.
    }

identify_os() {
    case $OSTYPE in
        darwin*) os="macOS";;
        linux*)  os="linux";;
        *) os="unknown";;
    esac
    echo $os
}

grab_username() {
    local user=${USER:-$(whoami 2>/dev/null)}
    user=${user:-$(id -un 2>/dev/null)}
    if [[ -z "$user" ]]; then
        echo "[ERROR] Cannot find username."
        exit 1
    fi
    echo "$user"
}

grab_home_dir() {
    local home=${HOME:-~/$USER}
    if ! [[ -d "$home" ]]; then home=${home:-/home/$USER}; fi
    if ! [[ -d "$home" ]]; then home=${home:-/Users/$USER}; fi
    if ! [[ -d "$home" ]]; then
        echo "[ERROR] Cannot find \"$USER\" home directory."
        exit 1
    fi
    echo $home
}

grab_config_dir() {
    local config="${XDG_CONFIG_HOME}"
    if [[ $OS == "macOS" && ! -d "$config" ]]; then config="${config:-$HOME/Library/Preferences}"; fi
    if [[ $OS == "macOS" && ! -d "$config" ]]; then config="${config:-$HOME/Library/Application Support}"; fi
    if ! [[ -d "$config" ]]; then config="${config:-$HOME/.config}"; fi
    if ! [[ -d "$config" ]]; then
        echo "[ERROR] Cannot find \"$USER\" config directory."
        exit $EXIT_CONFIG
    fi
    echo "${config}"
}

grab_install_dir() {
    local install_dir="${INSTALL_DIR}"
    if [[ $OS == "linux" ]]; then
        case $DISTRO in
            *) install_dir="${install_dir:-/usr/bin}" ;;
        esac
    elif [[ $OS == "macOS" ]]; then
        install_dir="${install_dir:-/usr/local/bin}"
    fi
    if ! [[ -d "$install_dir" ]]; then
        echo "[ERROR] Cannot locate install directory \"$install_dir\"."
        exit $EXIT_INSTALL_DIR
    fi
    echo "${install_dir}"
}

usage() {
    local exit_code=$1
    echo "Usage: $ /bin/bash ./$(basename $0) <install|update> [install_directory]"
    echo "Help:"
    echo " --install: install toutui and dependencies."
    echo " --update: update toutui and dependencies."
    echo " --uninstall: uninstall toutui."
    echo "Example: /bin/bash ./$(basename $0) install /usr/bin"
    eval "exit \$EXIT_${exit_code}"
}

get_distro() {
    local distro=$(head -n1 /etc/os-release 2>/dev/null| sed -E "s%.*\"([^\"]*).*\"%\1%")
    if [[ -z $distro ]]; then distro=$(lsb_release -a 2>/dev/null | grep Description | sed "s/Description:\s*//") ;fi
    if [[ -z $distro ]]; then distro=$(hostnamectl | grep "Operating System" | sed "s/Operating System:\s*//"); fi
    if [[ -z $distro ]]; then distro="unknown"; fi
    # rename distro to a lowercase general name (easier for package handling later)
    case "$distro" in
        Arch*) distro="archlinux";;
        Debian*|Ubuntu*) distro="debian";;
        Fedora*) distro="fedora";;
        CentOS*) distro="centos";;
        OpenSUSE*) distro="opensuse";;
        unknown|*) distro="unknown";;
    esac
    echo "$distro"
}

install_brew() {
    # adapted from https://brew.sh/
    bash -c "$(sudo curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
}

install_from_source() {
    echo "[ERROR] Could not identify OS/Distro."
    echo "Please follow the instructions here:"
    echo "https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#git"
    exit $EXIT_UNKNOWN_OS
}

propose_optional_dependencies() {
    local optionals="$@"
    if [[ $(( ${#optionals[@]} )) == 0 || "${optionals[@]}" =~ ^\ *$ ]]; then return; fi
        echo "[INFO] Toutui's experience could be improved by these optional packages:"
        for opt in "${optionals[@]}"; do
            echo -e "\t- ${opt}"
        done
        local answer=
        while :; do
            read -p "Would you like to install these packages? (y/N) : " answer
            if [[ $answer == "" || $answer =~ (n|N) ]]; then answer=no; break; fi
            if [[ $answer =~ (y|Y) ]]; then answer=yes; break; fi
        done
        case $answer in
            no)
                echo "[INFO] Ignoring optional dependencies.";;
            yes)
                echo "[INFO] Installing optional dependencies."
                install_packages "${optionals[@]}"
                echo "[OK] Optional dependencies installed."
                ;;
        esac
    }

install_rust() {
    if ! command -v rustc >/dev/null 2>&1; then
        echo "[INFO] Cannot find \"rustc\" in your \$PATH. Installing rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source_cargo_env
    else
        echo "[OK] \"rustc\" exists."
    fi
}

source_cargo_env() {
    if [[ $SHELL =~ \/(sh|bash|zsh|ash|pdksh) ]]; then
        if [[ -z "${CARGO_HOME}" ]]; then
            source "$HOME/.cargo/env"
        else
            source "${CARGO_HOME}/env"
        fi
    elif [[ $SHELL =~ \/fish ]]; then
        if [[ -z "${CARGO_HOME}" ]]; then
            source "$HOME/.cargo/env.fish"
        else
            source "${CARGO_HOME}/env.fish"
        fi
    elif [[ $SHELL =~ \/nushell ]]; then
        if [[ -z "${CARGO_HOME}" ]]; then
            source "$HOME/.cargo/env.nu"
        else
            source "${CARGO_HOME}/env.nu"
        fi
    else
        echo "[ERROR] Cannot source cargo environment automatically."
        echo "Open a new terminal and launch \"hello_toutui.sh\" again."
        exit $EXIT_NO_CARGO_PATH
    fi
}

check_toutui_installed() {
    is_installed="false"

    if [[ "$OS" == "linux" ]]; then
        if [[ -n "$XDG_CONFIG_HOME" && ( -e "$XDG_CONFIG_HOME/toutui" || -e "$HOME/.cargo/bin/toutui" || -e "/usr/local/bin/toutui" ) ]]; then
            is_installed="true"
        elif [[ -e "$HOME/.config/toutui" || -e "$HOME/.cargo/bin/toutui" || -e "/usr/local/bin/toutui" ]]; then
            is_installed="true"
        fi
    elif [[ "$OS" == "macOS" ]]; then
        if [[ -n "$XDG_CONFIG_HOME" && ( -e "$XDG_CONFIG_HOME/toutui" || -e "$HOME/.cargo/bin/toutui" || -e "/usr/local/bin/toutui" ) ]]; then
            is_installed="true"
        elif [[ -e "$HOME/Library/Preferences/toutui" || -e "$HOME/.cargo/bin/toutui" || -e "/usr/local/bin/toutui" ]]; then
            is_installed="true"
        fi
    fi

}

confirm_force_install_update() {
    local message_type=$1
    local message

    if [[ "$message_type" == "install" ]]; then
        message="Toutui is already installed. It's recommended to perform an uninstall before. Do you still want to force the installation? (y/N) : "
    elif [[ "$message_type" == "update" ]]; then
        message="Toutui in not installed. Install toutui before perform an update. Do you want to force update (not recommended)? (y/N) :"

    fi

    local answer=
    while :; do
        read -p "$message" answer
        if [[ -z $answer || $answer =~ (n|N) ]]; then answer=no; break; fi
        if [[ $answer =~ (y|Y) ]]; then answer=yes; break; fi
    done
    case $answer in
        no)
            exit 0
            ;;
        yes)
            ;;
    esac
}


install_packages() {
    local dep="$@"
    if (( ${#dep} == 0 )); then return; fi
    case $OS in
        linux)
	    DISTRO=${DISTRO:-$(get_distro)}
    	    case "$DISTRO" in
                arch*) sudo pacman -S ${dep[@]};;
                debian*) sudo apt install -y ${dep[@]};;
                fedora*) sudo dnf install -y ${dep[@]};;
                centos*) sudo yum install -y ${dep[@]};;
                opensuse*) sudo zypper install -y ${dep[@]};;
                *) install_from_source;;
    	    esac ;;
        macOS)
            if command -v brew >/dev/null 2>&1; then
                brew install ${dep[@]}
            else
                install_brew
                #echo "[ERROR] Please install \"brew\"."
                #exit $EXIT_FAIL
                fi ;;
        esac
    echo "[INFO] Packages installed successfully."
}

post_install_msg() {
    if ! [[ -f "$CONFIG_DIR/.env" ]]; then
        echo "[INFO] No secret found in .env. Do this:"
        echo "    $ mkdir -p ~/.config/toutui"
        echo "    $ echo 'TOUTUI_SECRET_KEY=secret' > ~/.config/toutui/.env"
    fi
}

install_config() {
    mkdir -p "$CONFIG_DIR" 2>/dev/null || ( echo "[ERROR] Cannot create config directory \"${CONFIG_DIR}\""; exit $EXIT_CONFIG )

    # .env
    local env="${CONFIG_DIR}/.env"
    local prompt="Please provide a secret key to encrypt the token stored in the database ($env): "
    local key=
    until [[ -f "$env" && $(sed "s/TOUTUI_SECRET_KEY=//g" "$env") != "" ]]; do
        read -p "$prompt: " key
        if ! [[ $key == "" ]]; then echo "TOUTUI_SECRET_KEY=${key}" > "$env"; echo;fi
    done

    # config.
     # create temp directory
    local tmpdir
    tmpdir=$(mktemp -d) # not supported in bash 3.2
    # dl config.example.toml in temp directory
    curl -LsSf "$url_config_file" -o "$tmpdir/config.toml"

    local example_config="$tmpdir/config.toml"
    if ! [[ -f "$example_config" ]]; then
        echo "[ERROR] \"config.example.toml\" not found."
        exit $EXIT_CONFIG
    else
        # If config file exists: consider this a reinstall, and be
        # careful not to remove users configuration (e.g. themes).
        local user_config="${CONFIG_DIR}/config.toml"
        if [[ -f "$user_config" ]]; then
            # If maintainer decides adding options in "config.toml", we have to
            # update user's config file accordingly without breaking things up.
            # Here is an attempt. If you know of any way to simplify this, feel
            # free to PR <|:^)
            local merged_config=

            # Grab sections from config.example.toml AND from user config (e.g. [player])
            local sections=$(grep -Eoh "^\[.+\] *$" "$example_config" "$user_config" | awk '!visited[$0]++' | sed "s/\[//;s/\]//")
            # `awk '!visited[$0]++'` removes duplicate grep outputs, but respect the order: https://superuser.com/a/1480765

            while read section; do
                local section_uppercase=$(echo $section | tr '[:lower:]' '[:upper:]') # macOS bash3.2 doesn't support ${section^^}
                local user_extracted_section=$(sed -En "/^(#### $section_uppercase|\[$section\])/{p;:loop n;/(^####|^\[[^($section)])/q;p;b loop;}" "$user_config")
                local example_extracted_section=$(sed -En "/^(#### $section_uppercase|\[$section\])/{p;:loop n;/(^####|^\[[^($section)])/q;p;b loop;}" "$example_config")
                # Now, merge user_extracted_section with example_extracted_section
                local merged_section=
                while read example_config_line; do
		    local pseudo_escaped_line=$(sed "s/\[//;s/\]//;s/(//;s/)//" <<< "$example_config_line") # avoids trouble with substrings
                    if grep -E "^$pseudo_escaped_line" <<< "$user_extracted_section" >/dev/null; then
			# Keep lines that match in both example and user's config
                        merged_section+="${example_config_line}"$'\n'
                    elif [[ "$example_config_line" =~ ^([^\ ]+)\ *=\ *(.*)$ ]]; then
			# If example_config_line matches "key=value":
                        local key=${BASH_REMATCH[1]}
                        local value=${BASH_REMATCH[2]}
                        if grep -E "^${key} *=" <<< "$user_extracted_section" >/dev/null; then
			    # then if key is in user's config, keep user's value
                            local user_line="$(grep -E "^${key} *=" <<< "$user_extracted_section")"
                            [[ "$user_line" =~ ^[^\ ]+\ *=\ *(.*)$ ]]
                            local user_value="${BASH_REMATCH[1]}"
                            merged_section+="${key} = ${user_value}"$'\n'
                        else
			    # add a non-existent line in user's config from config.example.toml
                            merged_section+="${example_config_line}"$'\n'
                        fi
		    else
			# Else add a potentially commented line
                        merged_section+="${example_config_line}"$'\n'
                    fi
                done <<< "$example_extracted_section"
		# For each user's config line that is not in config.example.toml,
		# add it to new config if not already added previously.
                while read user_config_line; do
		    local pseudo_escaped_line=$(sed "s/\[//;s/\]//;s/(//;s/)//" <<< "$user_config_line") # avoids trouble with substrings
                    if ! grep -E "^$pseudo_escape_line" <<< "$merged_section" >/dev/null; then
                        merged_section+="${user_config_line}"$'\n'
                    fi
                done <<< "$user_extracted_section"
		# Add freshly merged section to future user's config
                merged_config+="${merged_section}"$'\n'
            done <<< "$sections"
	    # Enjoy Toutui's respect for their users' config files <|:^)
            echo -e "$merged_config" > "$user_config"
        else
            cp "$example_config" "$user_config" || (echo "[ERROR] Cannot copy \"config.toml\"."; exit $EXIT_CONFIG)
            rm -rf "$tmpdir"
        fi
    fi

    rm -rf "$tmpdir"
}

dep_already_installed() {
    local pkg_name=$1
    local cmd_check=${2:-$pkg_name}
    local installed="false"
    if [[ $OS == "linux" ]]; then
        case "$DISTRO" in
            arch*)     (pacman -Qq $pkg_name >/dev/null)2>/dev/null && installed="true";;
            debian*)   (dpkg -l | awk '{print $2}' | grep "^${pkg_name}$" >/dev/null)2>/dev/null && installed="true";;
            fedora*)   (rpm -q "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
            centos*)   (yum list installed "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
            opensuse*) (zypper se --installed-only "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
        esac
    elif [[ $OS == "macOS" ]]; then
        (brew list | grep $pkg_name) && installed="true"
    fi
    if [[ $installed == "false" ]]; then
        if [[ $cmd_check != "no_check" && $(command -v $cmd_check 2>/dev/null) ]]; then
            installed="true"
        fi
    fi
    echo $installed
}

install_deps() {
    # Grab dependencies and optional dependencies
    # Optional deps start with "*" (e.g. *cvlc).
    local deps=()
    local optionals=()
    if [[ -f deps.txt ]]; then
        while read -r line; do
            if [[ $line == "" || $line =~ ^\# ]]; then continue; fi
            deps+=( "$line" )
        done < deps.txt
    else
        deps=("${HC_DEPS[@]}")
    fi

    # Ignore already installed deps
    # Keep track of optional deps
    local missing=()
    for dep in "${deps[@]}"; do
        if [[ $dep =~ ^\* ]]; then
            # this is an optional dependency
            deps=("${deps[@]/$dep}") # remove optional from deps
            dep="${dep:1:${#dep}}" # trim
            local optional="true"
        elif [[ $dep =~ ^% ]];then
            # this is a forced dependency
            deps=("${deps[@]/$dep}") # remove from deps
            dep="${dep:1:${#dep}}" # trim
            deps+=( "$dep" ) # add it back
            local optional="false"
        else
            local optional="false"
        fi
        # Check if package is for OS || distro
        # linux:XXX means for all distro
        # debian:XX means specific to debian/ubuntu
        if [[ "$dep" =~ ^($OS):([^:]*)(:(.*))? || "$dep" =~ ^($DISTRO):([^:]*)(:(.*))? ]]; then
            target_sys=${BASH_REMATCH[1]}
            dep=${BASH_REMATCH[2]}
            cmd=${BASH_REMATCH[4]}
            # if OS or DISTRO match, add to optional deps
            if [[ $target_sys == $OS || $target_sys == $DISTRO ]]; then
                # add only if not installed
                if [[ $optional == "true" ]]; then
                    if [[ $(dep_already_installed "$dep" "$cmd") == "false" ]]; then
                        optionals+=( $dep )
                    fi
                else
                    if [[ $(dep_already_installed "$dep" "$cmd") == "false" ]]; then
                        echo "[DEP] Missing dependency \"$dep\""
                        missing+=( $dep )
                    fi
                fi
            fi
        fi
    done
    install_packages "${missing[@]}" && echo "[INFO] Essential dependencies are installed."
    #propose_optional_dependencies "${optionals[@]}"
}

install_message() {
    echo "[INFO]"
    echo "The installation will have these effects:"
    echo "Install dependencies if needed: VLC, Netcat, Rust, for macos: Homebrew and gsed"
    echo "Add the binary in /usr/local/bin (option 1) or ~/.cargo/bin (option 2)"
    echo "For Linux:"
    echo "Add the directory "toutui" in $HOME/.config (or any other path specified in XDG_CONFIG_HOME) with inside the following files: "
    echo ".env, db.sqlite3, config.toml, toutui.log"
    echo "toutui.desktop will be added in $HOME/.local/share/applications"
    echo "For macOS:"
    echo "Add the directory "toutui" in $HOME/Library/Preferences (or any other path specified in XDG_CONFIG_HOME) with inside the following files: "
    echo ".env, db.sqlite3, config.toml, toutui.log"
    echo " "
    echo " You can run "toutui --uninstall" or the official uninstall curl link to remove all these added files."
    echo 'Only dependencies will not be uninstalled (e.g. VLC, Netcat, Rust, gsed, Homebrew)'
    echo " "
}

install_menu() {
    echo "[HELP] Option 1 is the most user-friendly installation. No compilation time, no need to install rust/cargo. However, if it does not work, select option 2."
    ps3="Please enter your choice: "
    options=(
        "Option 1 - Download the binary (recommended)"
        "Option 2 - Compile from source (remotely, no local clone, will install Rust if it is not already installed)"
        "Option 3 - Clone the repo and compile from source locally (manually)"
        "Quit"
    )

    select opt in "${options[@]}"
    do
        case $REPLY in
            1)
                install_method="binary"
                break
                ;;
            2)
                install_method="source"
                break
                ;;
            3)
                echo "requirements:"
                echo "rust, netcat, vlc, (optional : kitty)"
                echo "follow these steps: "
                echo "clone the main branch (might be unstable):"
                echo "git clone https://github.com/albandavid/toutui"
                echo "or clone the last stable release:"
                echo "git clone --branch stable --single-branch https://github.com/albandavid/toutui"
                echo "cd toutui/"
                echo "mkdir -p ~/.config/toutui"
                echo "cp config.example.toml ~/.config/toutui/config.toml"
                echo "token encryption in the database (note: replace secret) : "
                echo "echo toutui_secret_key=secret >> ~/.config/toutui/.env"
                echo "cargo run --release"
                echo "update :"
                echo "git pull {URL}"
                echo "cargo run --release"
                exit 0
                break
                ;;
            4)
                echo "bye!"
                exit 0
                break
                ;;
            *)
                echo "invalid option: $REPLY"
                ;;
        esac
    done
}

check_and_cleanup_binary_install() {

    local temp_dir=$1

    if [[ ! -e "$temp_dir/toutui" ]]; then
        echo "[ERROR] Failed to download the binary. Please try again later."
        EXIT_FAIL
    fi
    if [[ -e "/usr/local/bin/toutui" && -e "$temp_dir/toutui" ]]; then
        sudo rm "/usr/local/bin/toutui"
    fi
    if [[ -e "$HOME/.cargo/bin/toutui" && -e "$temp_dir/toutui" ]]; then
        sudo rm "$HOME/.cargo/bin/toutui"
    fi
}

dl_handle_compressed_binary() {
    local final_url=$1
    local binary_name=$2
    temp_dir=$(mktemp -d)
    echo "[INFO] Downloading the compressed binary from $final_url"
    sudo curl -L "$final_url" -o "$temp_dir/$binary_name"
    sudo tar -xvzf "$temp_dir/$binary_name" -C "$temp_dir"
    check_and_cleanup_binary_install "$temp_dir"
    echo "[INFO] Copying the binary from temp directory to /usr/local/bin"
    sudo cp "$temp_dir/toutui" "/usr/local/bin"
    rm -rf "$temp_dir"
}

setup_launcher() {
    if [[ "$OS" == "linux" ]]; then
        mkdir -p "$HOME/.local/share/applications"
        curl -sSL "$url_toutui_desktop" -o "$HOME/.local/share/applications/toutui.desktop"
    fi
   # elif [[ "$OS" == "macOS" ]]; then
   #     mkdir -p "/Applications/toutui.app/Contents"
   #     mkdir -p "/Applications/toutui.app/Contents/MacOS"
   #     curl -L "https://raw.githubusercontent.com/AlbanDAVID/Toutui/install_with_cargo/curl/Info.plist" -o "/Applications/toutui.app/Contents/Info.plist"
   #     curl -L "https://raw.githubusercontent.com/AlbanDAVID/Toutui/install_with_cargo/curl/launch.command" -o "/Applications/toutui.app/Contents/MacOS/launch.command"
   #     chmod +x "/Applications/toutui.app/Contents/MacOS/launch.command"
   # fi
}

install_binary() {
    # get the architecture
    arch=$(uname -m)

    # get full and latest version on github(e.g: v0.1.0-beta)
    full_version=$(curl -s "$url_latest_release" | grep tag_name | sed -E "s|.*\"([^\"]*)\",|\1|")


    # determine binary to download
    if [[ "$OS" == "linux" && "$arch" == "x86_64" ]]; then
        echo "[INFO] Linux x86_64 detected"
        binary_name="toutui-x86_64-unknown-linux-gnu.tar.gz"
        final_url="$url_latest_binary/$full_version/$binary_name"
        dl_handle_compressed_binary "$final_url" "$binary_name"
    fi
    if [[ "$OS" == "linux" && "$arch" == "aarch64" ]]; then
        echo "[INFO] Linux aarch64 detected"
        binary_name="toutui-aarch64-unknown-linux-gnu.tar.gz"
        final_url="$url_latest_binary/$full_version/$binary_name"
        dl_handle_compressed_binary "$final_url" "$binary_name"
    fi
    if [[ "$OS" == "linux" && "$arch" != "x86_64" && "$arch" != "aarch64" ]]; then
        echo "[ERROR] No binary available for Linux $arch. You might compile from source"
        exit 0
    fi
    if [[ "$OS" == "macOS" && "$arch" == "arm64" ]]; then
        echo "[INFO] macOS arm64 detected"
        binary_name="toutui-universal-apple-darwin.tar.gz" # for intel and sillicon
        final_url="$url_latest_binary/$full_version/$binary_name"
        dl_handle_compressed_binary "$final_url" "$binary_name"
    fi
    if [[ "$OS" == "macOS" && "$arch" == "x86_64" ]]; then
        echo "[INFO] macOS x86_64 detected"
        binary_name="toutui-universal-apple-darwin.tar.gz" # for intel and sillicon
        final_url="$url_latest_binary/$full_version/$binary_name"
        dl_handle_compressed_binary "$final_url" "$binary_name"
    fi
    if [[ "$OS" == "macOS" && "$arch" != "x86_64" && "$arch" != "arm64" ]]; then
        echo "[ERROR] No binary available for macOS $arch. You might compile from source"
        exit 0
    fi
    if [[ "$OS" == "unknown" ]]; then
        echo "unknown os"
        exit 0
        break
    fi

}

confirm_install_deps_macos() {
    local answer=

    if [[ "$OS" == "macOS" ]]; then

        echo "[IMPORTANT] If you select 1, the script will automatically fetch and install the required dependencies (Brew, VLC, Netcat, gsed) if they are missing."
        echo "[IMPORTANT] Please note: package detection via Homebrew can sometimes be unreliable (but it's not risky). If you encounter issues, install the packages by yourself and select 2."

        while :; do
            read -p "Select option: (1/2/Q (to quit the installation)) : " answer
            if [[ $answer =~ (1) ]]; then answer=option1; break; fi
            if [[ $answer =~ (2) ]]; then answer=option2; break; fi
            if [[ $answer =~ (q|Q) ]]; then answer=quit; break; fi

        done
        case $answer in
            option1)
                install_deps # install essential and/or optional deps
                ;;
            option2)
                ;;
            quit)
                echo "Installation aborted. Install required dependencies Brew, VLC, Netcat and gsed and perfom again an install"
                exit 0
                ;;
        esac
    fi

}

install_toutui() {
    check_toutui_installed
    if [[ "$is_installed" == "true" ]]; then
        confirm_force_install_update "install"
    fi
    install_message
    install_menu
    if [[ "$install_method" == "binary" ]]; then
        echo "Install the binary..."
        confirm_install_deps_macos
        if [[ "$OS" == "linux" ]]; then
            install_deps # install essential and/or optional deps
        fi
        install_config # create ~/.config/toutui/ etc.
        install_binary
        setup_launcher
        if [[ "$OS" == "linux" ]]; then
            echo "[DONE] Install complete. Launch toutui from your favorite app launcher or type toutui in your terminal to run it!"
        elif [[ "$OS" == "macOS" ]]; then
            echo "[DONE] Install complete. Type toutui in your terminal to run it!"
        fi
        echo "[ADVICE] Explore and try various themes: https://github.com/AlbanDAVID/Toutui-theme"
        echo "[ADVICE] Best experience with Kitty or Alacritty terminal."
    elif [[ "$install_method" == "source" ]]; then
        echo "Compiling from source..."
        confirm_install_deps_macos
        if [[ "$OS" == "linux" ]]; then
            install_deps # install essential and/or optional deps
        fi
        install_config # create ~/.config/toutui/ etc.
        install_rust # cornerstone! toutui is written by a crab
        #cargo install --git https://github.com/AlbanDAVID/Toutui --branch install_with_cargo
        cargo install --git "$url_cargo_install" --branch stable
        echo "[INFO] Binary placed in ~.cargo/bin"
        setup_launcher
        # copy Toutui binary to system path
        # sudo cp ./target/release/Toutui "${INSTALL_DIR}/toutui" || exit $EXIT_BUILD_FAIL
        if [[ "$OS" == "linux" ]]; then
            echo "[DONE] Install complete. Launch toutui from your favorite app launcher or type toutui in your terminal to run it!"
        elif [[ "$OS" == "macOS" ]]; then
            echo "[DONE] Install complete. Type toutui in your terminal to run it!"
        fi
        echo "[ADVICE] Explore and try various themes: https://github.com/AlbanDAVID/Toutui-theme"
        echo "[ADVICE] Best experience with Kitty or Alacritty terminal."
        post_install_msg # only if .env not found
    fi
}


update_menu() {
    echo "[HELP] Option 1 is the most user-friendly updating method. No compilation time, no need to install rust/cargo. However, if it does not work, select option 2."
    ps3="Please enter your choice: "
    options=(
        "Option 1 - Download the binary (recommended)"
        "Option 2 - Update by compiling from source (no local clone, will install Rust if it is not already installed)"
        "Option 3 - Update from the local clone (manually)"
        "Quit"
    )

    select opt in "${options[@]}"
    do
        case $REPLY in
            1)
                update_method="binary"
                break
                ;;
            2)
                update_method="source"
                break
                ;;
            3)
                echo "cd toutui"
                echo "git pull {URL}"
                echo "cargo run --release"
                exit 0
                break
                ;;
            4)
                echo "bye!"
                exit 0
                break
                ;;
            *)
                echo "invalid option: $REPLY"
                ;;
        esac
    done
}

get_toutui_local_release() {
#    if ! [[ -f Cargo.toml ]]; then
#        echo "[ERROR] Cannot find \"Cargo.toml\"."
#        exit $EXIT_NO_CARGO_TOML
#    fi
#    grep "version" Cargo.toml | head -1 | sed -E "s/^version\s*=\s*\"([^\"]*)\"\s*$/\1/"

toutui --version | cut -d' ' -f2

}

get_toutui_github_release() {
    curl -s "$url_latest_release" | grep tag_name | sed -E "s|.*\"v([^\"]*)\",|\1|"
}

display_changelog() {
    local changelog=$(curl -s "$url_latest_release" | grep "\"body\"" | sed -E "s|^\s*\"body\":\s*\"([^\"]*)\"|\1|")
    echo -e "\x1b[2m### CHANGELOG ###\x1b[0m"
    echo -e "\x1b[2m$changelog\x1b[0m"
    echo -e "\x1b[2m#################\x1b[0m"
}

check_and_cleanup_source_install() {
    if [[ -e "/usr/local/bin/toutui" ]]; then
        sudo rm "/usr/local/bin/toutui"
    fi
}

pull_latest_version() {
    local version=$1
    local answer=
    while :; do
        read -p "Would you like to update to the latest version? (Y/n) : " answer
        if [[ $answer =~ (n|N) ]]; then answer=no; break; fi
        if [[ $answer == "" || $answer =~ (y|Y) ]]; then answer=yes; break; fi
    done
    case $answer in
        no)
            echo "[INFO] Ignoring latest version.";;
        yes)
            if [[ "$OS" == "macOS" ]]; then
                local answer2=
                while :; do
                    read -p "gsed package is needed to correctly perform the update. Please, check if you have it ('brew install gsed' to install it). Continue? (Y/n) : " answer
                    if [[ $answer =~ (n|N) ]]; then answer=no; break; fi
                    if [[ $answer == "" || $answer =~ (y|Y) ]]; then answer=yes; break; fi
                done
                case $answer2 in
                    no)
                        echo "[INFO] Update aborted"
                        exit 0
                        ;;
                    yes)
                        ;;

                esac
            fi
            # echo "[INFO] Pulling latest version..."
            # git fetch && git pull
            update_menu
            if [[ "$update_method" == "binary" ]]; then
                install_binary
            elif [[ "$update_method" == "source" ]]; then
                install_rust
                cargo install --force --git "$url_cargo_install" --branch stable
                echo "[INFO] Binary placed in ~.cargo/bin"
                check_and_cleanup_source_install
            fi
            install_config
            # cargo build --release
            # sudo cp ./target/release/Toutui "${INSTALL_DIR}/toutui" || exit $EXIT_BUILD_FAIL
            echo "[OK] Latest version installed (v$version)."
            ;;
    esac
}

update_toutui() {
    check_toutui_installed
    if [[ "$is_installed" == "false" ]]; then
        confirm_force_install_update "update"
    fi
    local local_release=$(get_toutui_local_release)
    local github_release=$(get_toutui_github_release)
    echo "[INFO] Local:  $local_release"
    echo "[INFO] GitHub: $github_release"
    if [[ $local_release == $github_release ]]; then
        echo "[INFO] Up to date (version $local_release)."
    else
        #echo "TODO: check if is behind or ahead?"
        if [[ "$OS" == "linux" ]]; then
            install_deps # check for new deps
        fi
        display_changelog # display before pulling?
        pull_latest_version $github_release

    fi
}

uninstall_message() {
    echo "Uninstall will do this:"
    echo "Delete the binary in /usr/local/bin or ~/.cargo/bin"
    echo "For Linux:"
    echo "The directory "toutui" in $HOME/.config (or any other path specified in XDG_CONFIG_HOME) wil be deleted: "
    echo "[IMPORTANT] save your config.toml if you need it later"
    echo "[IMPORTANT] XDG_CONFIG_HOME must be the same as it was at the time Toutui was installed. (in case you change it)"
    echo "toutui.desktop will be deleted from $HOME/.local/share/applications"
    echo "For macOS:"
    echo "The directory "toutui" in $HOME/Library/Preferences (or any other path specified in XDG_CONFIG_HOME) will be deleted: "
    echo "[IMPORTANT] save your config.toml if you need it later"
    echo "[IMPORTANT] XDG_CONFIG_HOME must be the same as it was at the time Toutui was installed. (in case you change it)"
    echo " "
    echo 'Only dependencies will not be uninstalled (e.g. VLC, Netcat, Rust, Homebrew, gsed)'
    echo " "
}

uninstall_process() {
    if [[ "$OS" == "linux" ]]; then

        # delete the config folder
        if [[ -n "$XDG_CONFIG_HOME" && -e "$XDG_CONFIG_HOME/toutui" ]]; then
            sudo rm -r "$XDG_CONFIG_HOME/toutui"
            echo "$XDG_CONFIG_HOME/toutui deleted."
        fi

        if [[ -e "$HOME/.config/toutui" ]]; then
            sudo rm -r "$HOME/.config/toutui"
            echo "$HOME/.config/toutui deleted."
        fi

        # delete toutui.desktopp
        if [[ -e "$HOME/.local/share/applications/toutui.desktop" ]] ; then
            sudo rm "$HOME/.local/share/applications/toutui.desktop"
            echo "$HOME/.local/share/applications/toutui.desktop deleted."
        fi

    fi

    if [[ "$OS" == "macOS" ]]; then

        # delete the config folder
        if [[ -n "$XDG_CONFIG_HOME" && -e "$XDG_CONFIG_HOME/toutui" ]]; then
            sudo rm -r "$XDG_CONFIG_HOME/toutui"
            echo "$XDG_CONFIG_HOME/toutui deleted."
        fi

        if [[ -e "$HOME/Library/Preferences/toutui" ]]; then
            sudo rm -r "$HOME/Library/Preferences/toutui"
            echo "$HOME/Library/Preferences/toutui deleted."
        fi
    fi


    # delete the binary
    if [[ -e "/usr/local/bin/toutui" ]]; then
        sudo rm "/usr/local/bin/toutui"
        echo "/usr/local/bin/toutui deleted."
    fi
    if [[ -e "$HOME/.cargo/bin/toutui" ]]; then
        sudo rm "$HOME/.cargo/bin/toutui"
        echo "$HOME/.cargo/bin/toutui deleted."
    fi


}

uninstall_toutui() {
    uninstall_message
    local answer=
    while :; do
        read -p "Are you sure to uninstall toutui? (Y/n) : " answer
        if [[ $answer =~ (n|N) ]]; then answer=no; break; fi
        if [[ $answer == "" || $answer =~ (y|Y) ]]; then answer=yes; break; fi
    done
    case $answer in
        no)
            echo "[INFO] Uninstall aborted";;
        yes)
            echo "[INFO] Starting uninstall..."
            uninstall_process
            echo "[OK] Toutui has been successfully uninstalled."
            ;;
    esac

}

load_exit_codes() {
    # Exit codes for convenience?
    EXIT_OK=0
    EXIT_FAIL=1
    EXIT_ROOT=2
    EXIT_UNKNOWN_OS=3
    EXIT_INCORRECT_ARG=4
    EXIT_NO_CARGO_TOML=5
    EXIT_NO_CARGO_PATH=6
    EXIT_CONFIG=7
    EXIT_BUILD_FAIL=8
    EXIT_INSTALL_DIR=9
}

do_not_run_as_root() {
    # Must not be run as root
    if [[ $EUID == 0 ]]; then
        echo "[ERROR] Do not run this script as root."
        exit $EXIT_ROOT
    fi
}

main "$@"

# TODO:
# - clone repo from here (making this bloated bash script "portable")
# - test automatic dependencies install on more distributions
# - allow calling toutui from outside the terminal (for macOS, available for Linux)
