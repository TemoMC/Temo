# Temo
The Temo Minecraft server microkernel, for complete control and extensibility in an effective and fast way.

## Introduction
Temo is a bare-bones but highly extendable Minecraft server written from scratch in Rust and released under the GPLv3+.

It aims to be the microkernel of Minecraft servers, keeping as much in plugin-land as possible, allowing extensions natively in Rust or a custom language, [TemoScript](https://github.com/TemoMC/TemoScript), designed specifically for these purposes.

## Features
Temo, in many ways, hopes to be better than other implementations:
* It is **100% libre software**, allowing complete control over the implementation
* It is extremely minimal, doing little beyond providing an interface between extensions
* It is not restricted to Vanilla minecraft, or any game at all, merely being optimized for that case

However, it is not all benefits:
* None of these features exist yet, and right now this repository is just a place for me to mess around with MC servers
* It will likely be somewhat slow, especially parts written in TemoScript
* It will be implemented poorly, since this is my first Rust project

## Copyright
Temo is libre software released under the GPLv3+ license, see the `LICENCE` file for details.
