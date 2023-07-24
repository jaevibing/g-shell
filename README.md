# g-shell
g-shell is an open-source rust-based compiled unix-like shell meant for performance, compatability, customisation, modding and community.
#### information about the project
g-shell is currently in a very early pre-alpha to create a working prototype that can be deployed as a proper shell before new features and ideas can be implemented
#### pull requests, forks and issues
any additions or improvements are highly encouraged
#### how to use
in this stage of development, no official releases have been made, if you wish to test g-shell, you should clone the repo and compile it
```
git clone https://github.com/jaevibing/g-shell.git
```
```
cd g-shell
```
```
rustc src/main.rs -o g-shell
```
```
./g-shell
```
#### commands
`help` - prints the helpfile to the terminal
`end` - kills the g-shell process
#### roadmap for development
* implementing mainstay unix commands (man, sudo, kill, etc.)
* make the shell able to be installed as a shell