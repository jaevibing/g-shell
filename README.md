# g-shell
[![Source](https://github.com/jaevibing/g-shell/actions/workflows/rust.yml/badge.svg)](https://github.com/jaevibing/g-shell/actions/workflows/rust.yml)

g-shell is an open-source rust-based compiled unix-like shell meant for performance, compatability, customisation, modding and community.
### information about the project
g-shell is currently in a very early pre-alpha to create a working prototype that can be deployed as a proper shell before new features and ideas can be implemented
### pull requests, forks and issues
any additions or improvements are highly encouraged, due to the pre-alpha nature of the current releases, bugs, missing features and issues are expected so report them when found.
### how to use
you can test g-shell by either testing the binary or fully installing the shell

you can test the binary by either downloading the latest stable binary from the releases tab:
```
gsh$(curl -s https://api.github.com/repos/jaevibing/g-shell/releases/latest | grep "gsh" | cut -d : -f 2,3 | tr -d \" | wget -qi -)
```
or compiling the latest version from the source like this:
```
git clone https://github.com/jaevibing/g-shell.git
```
```
cd g-shell
```
```
cargo build
```
```
./target/debug/gsh
```

if you wish to install g-shell you must run the shell script file. this can be done with this command:
```
sh -c "$(curl -fsSL https://raw.githubusercontent.com/jaevibing/g-shell/master/tools/install.sh)"
```
### commands
`help` - prints the helpfile to the terminal

`end` - kills the g-shell process
### roadmap for development
* making it more customizable
* adding plugin support
* adding base themes
* making the update checker *truly* asynchronous
* add environment variables
* cycle through past commands with up and down arrow keys
