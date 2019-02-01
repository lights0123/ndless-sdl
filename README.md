# Rust-SDL
Bindings for SDL in Rust
# Overview

Rust-SDL is a library for talking to SDL from Rust. Low-level C components are wrapped in Rust code to make them more idiomatic and abstract away inappropriate manual memory management.

In addition, it provides optional APIs to a number of common SDL extension libraries.

Rust-SDL uses the MIT license.

# Requirements

* *Rust* - You can get the Rust compiler from http://www.rust-lang.org
* *SDL 1.2 development libraries* - install through your favourite package management tool, or via http://www.libsdl.org/

**Optional**
* *SDL_Mixer* and *SDL_Image* - also available through most package managers, or through http://www.libsdl.org/projects/SDL_mixer/ and http://www.libsdl.org/projects/SDL_image/

# When things go wrong
Rust, and Rust-SDL, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest Master branch of Rust, check that you've updated Rust-SDL to the latest version, and run `cargo clean`. If that fails, please let us know on the issue tracker.
