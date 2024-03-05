#!/usr/bin/env bash
# Installs haxeget
# Code copied from a variety of sources, ty!

version="v0.3.1"

is_command() {
  command -v "$1" >/dev/null
}

http_download_curl() {
    local_file=$1
    source_url=$2
    header=$3
    if [ -z "$header" ]; then
        code=$(curl -w '%{http_code}' -sL -o "$local_file" "$source_url")
    else
        code=$(curl -w '%{http_code}' -sL -H "$header" -o "$local_file" "$source_url")
    fi
    if [ "$code" != "200" ]; then
        echo "http_download_curl received HTTP status $code"
        return 1
    fi
    return 0
}

http_download_wget() {
    local_file=$1
    source_url=$2
    header=$3
    if [ -z "$header" ]; then
        wget -q -O "$local_file" "$source_url"
    else
        wget -q --header "$header" -O "$local_file" "$source_url"
    fi
}

http_download() {
    echo "http_download $2"
    if is_command curl; then
        http_download_curl "$@"
        return
    elif is_command wget; then
        http_download_wget "$@"
        return
    fi
    echo "http_download unable to find wget or curl"
    return 1
}

extract_tar() {
    tar xzf $1 -C $2
}

get_dir() {
    case $OSTYPE in 
        "darwin"*)
            echo "$HOME/.haxeget/"
            return 0;;
        "linux"*)
            echo "$HOME/.local/bin/haxeget"
            return 0;;
    esac
    echo "Unsupported OS: $OSTYPE, please send a pull request"
    return 1
}

install() {
    tmpdir=$(mktemp -d)
    output_dir=$(get_dir)
    
    mkdir -p $output_dir

    case $OSTYPE in
        "darwin"*)
            http_download "$tmpdir/haxeget.tar.gz" "https://github.com/l0go/haxeget/releases/download/$version/haxeget-x86_64-apple-darwin.tar.gz";;
        "linux"*)
            http_download "$tmpdir/haxeget.tar.gz" "https://github.com/l0go/haxeget/releases/download/$version/haxeget-x86_64-unknown-linux-gnu.tar.gz";;
    esac
    extract_tar "$tmpdir/haxeget.tar.gz" $tmpdir
    
    mv "$tmpdir/haxeget" "$output_dir/haxeget"
    rm "$tmpdir/haxeget.tar.gz"
}

add_to_path_zsh() {
    read -p "Do you want to add \"path+=$(get_dir)\" to your ~/.zshrc (this is recommended) (y/n)? " choice
    case "$choice" in 
        y|Y|yes|YES|Yes )
            echo "path+=$(get_dir)" >> ~/.zshrc
            echo "export HAXE_STD_PATH=\"$(get_dir)std/\"" >> ~/.zshrc;;
    esac
    echo "Run \"path+=$(get_dir)\" in order to use haxeget in your current shell"
}

add_to_path_bash() {
    read -p "Do you want to add \"PATH=$PATH:$(get_dir)\" to your ~/.bash_profile (this is recommended) (y/n)? " choice
    case "$choice" in 
        y|Y|yes|YES|Yes ) 
            echo "PATH=$PATH:$(get_dir)" >> ~/.bash_profile
            echo "export HAXE_STD_PATH=$(get_dir)std/" >> ~/.bash_profile;;
    esac
    echo "Run \"PATH=$PATH:$(get_dir)\" in order to use haxeget in your current shell"
}

add_to_path() {
    case $SHELL in
        "/bin/zsh"|"/usr/bin/zsh"|"/usr/zsh")
            add_to_path_zsh;;
        "/bin/bash"|"/usr/bin/bash"|"/usr/bash")
            add_to_path_bash;;
        *)
            echo "Run \"PATH=$PATH:$(get_dir)\" in order to use haxeget in your current shell";;
    esac
}

echo "Installing haxeget.."
install
add_to_path
echo "Done :)"
