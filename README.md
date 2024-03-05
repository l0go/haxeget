# haxeget
The *easier* way to install and manage Haxe compiler versions

## Installation
On macOS and Linux, the easiest way to install is to use the meta-installer with this one command
```sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/l0go/haxeget/main/meta-install.sh | bash
```

On Windows and other platforms, you can download the executable from the [releases tab](https://github.com/l0go/haxeget/releases) and add it to your path, or you can install via cargo with the following command.
```sh
cargo install haxeget
```

## Usage
```sh
# Here is how we would install version 4.3.2 of the compiler
$ haxeget install 4.3.2
$ haxeget use 4.3.2
$ haxe # Can now run haxe and haxelib freely
```

If needed, we can install another version and switch freely between them with the ``haxeget use <version>`` command.

## Commands
| Command   | About                                                                                |
| -------   | -----                                                                                |
| Install   | Installs the specified version of Haxe or Neko. ex: ``4.3.3``, ``neko``, ``nightly`` |
| Uninstall | Uninstalls the specified version                                                     |
| Use       | Selects the version of Haxe to use                                                   |
| List      | Lists the installed versions                                                         |
| Rc        | Installs the version of Haxe specified in .haxerc                                    |
| Update    | Updates ``haxeget`` to the latest version                                            |
| Current   | Outputs the currently used Haxe version                                              |


## Why Rust?
I wanted to mess with the Rust programming language and this seemed like a decent opportunity. If I had proper hindsight, I would have written it in a better language like Go, Zig, or even godforbid Haxe itself. This gives us the interesting property of not forcing you to have a pre-existing Haxe compiler set up to install Haxe itself.

## Alternatives
- [haxe-manager](https://github.com/kLabz/haxe-manager/): The original inspiration for this, still a valid option!
- [asdf-haxe](https://github.com/asdf-community/asdf-haxe): If I was aware that asdf had a Haxe plugin, I would probably just have used that. Writing my own is a lot more entertaining though!
