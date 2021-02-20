# please

## What is this?
`please` is an incredibly simple, feature-less version of `sudo` or `doas`. I made it 
for myself as I don't have a use for the complexity of `sudo`, and instead wanted to 
opt to make something incredibly simple. 

## Why does it lack x feature
It lacks x feature because I don't need it. 

## Are there any limitations
Yes. Currently you can only run one command at a time. That means no pipes or `&&`.

## How is it configured
A config file should be placed at `/usr/local/etc/please.conf`. Also, you will need 
to manually set a SUID bit as root on the finished binary. 

## What is rs-crypt
`rs-crypt` is a simple Rust wrapper(?)/binding(?) over the `crypt` function of 
`crypt.h`. 
