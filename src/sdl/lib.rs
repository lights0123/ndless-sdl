#![no_std]
//! # SDL bindings for Ndless
//! Get started with:
//! ```
//! ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
//! let screen = match ndless_sdl::video::set_video_mode(320, 240, 16,
//!                                                      &[SurfaceFlag::SWSurface],
//!                                                      &[VideoFlag::NoFrame]) {
//!     Ok(screen) => screen,
//!     Err(err) => panic!("failed to set video mode: {}", err)
//! };
//! loop {
//!     screen.fill_rect(Some(ndless_sdl::Rect {
//!          x: 0,
//!          y: 0,
//!          w: 320,
//!          h: 240,
//!     }), ndless_sdl::video::RGB(142, 120, 255));
//! }
//! ndless_sdl::quit();
//! ```
//!
//! It is not recommended to use the input methods from this crate. Rather, use the ones built
//! into the nspire crate.
extern crate num;
extern crate rand;

pub use sdl::*;

pub mod event;
pub mod keysym;
pub mod mouse;
pub mod video;
pub mod gl;
pub mod wm;
pub mod nsdl;

pub mod sdl;
