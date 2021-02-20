# please

## What is this?
`please` is an incredibly simple, feature-less version of `sudo` or `doas`. I made it 
for myself as I don't have a use for the complexity of `sudo`, and instead wanted to 
opt to make something incredibly simple. 

## Why is the code so bad and messy
This is the first version. With the projects I work on, the first version is always bad and 
messy. I focus on constructing a minimum viable program first, and then cleaning things up 
later. For example, all the code lives in `main.rs` at the moment, all in the `main` 
function. This is very bad and also cringe. Bear with me.

## Is it secure
Not really. Any `sudo` like program functions by having a SUID bit set by root. 
This means the binary runs as root. At the moment, `please` has a debug mode. 
By activating debug mode, you see the contents of `/etc/shadow/`, even if you 
fail to authenticate. Debug mode is going to be removed imminently however. 
After debug mode is removed, it will probably be secure. The program is very 
small, so there's quite a small surface area for bugs. Also, it is written in 
Rust, which adds a safety blanket.

## Are there any limitations
Yes. Currently you can only run one command at a time. That means no pipes or `&&`.

## How is it configured
A config file should be placed at `/usr/local/etc/please.conf`. Also, you will need 
to manually set a SUID bit as root on the finished binary. 

## What is rs-crypt
`rs-crypt` is a simple Rust wrapper(?)/binding(?) over the `crypt` function of 
`crypt.h`. 
