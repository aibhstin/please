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

## What is rs-crypt
`rs-crypt` is a simple Rust wrapper(?)/binding(?) over the `crypt` function of 
`crypt.h`. 
