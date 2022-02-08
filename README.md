# Shell Simulator in Rust
This is a recreation of WSU's CS360 (Systems Programming) Lab 3 in Rust. This was done as a way to learn the language and (hopefully) write safe, idiomatic Rust as opposed to occasionally unsafe and volatile C code.

## Requirements
- [ ] Prompt the user for a command line of the form
```
cmd arg1 arg2 arg3 ... argn
```
- [ ] Handle simple commands (cd == `chdir(arg1)` or `chdir(HOME)` if arg1 DNE, exit == `exit(0)`)
- [ ] For all other commands, fork a child, exec in child using PATH, wait for child to terminate
- [ ] I/O redirection (`<`, `>`, and `>>`)
- [ ] Pipes (`cmd1 | cmd2`)
- [ ] Multiple pipes (`cmd1 | cmd2 | cmd3`)
