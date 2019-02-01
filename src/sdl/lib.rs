#![no_std]

extern crate num;
extern crate rand;

pub use sdl::*;

pub mod event;
pub mod keysym;
pub mod mouse;
pub mod video;
pub mod gl;
pub mod wm;

pub mod sdl;

#[repr(C)]
#[repr(C)]
pub struct nSDL_Font {
	pub chars: [*mut video::ll::SDL_Surface; 256usize],
	pub char_width: [cty::uint8_t; 256usize],
	pub hspacing: cty::c_int,
	pub vspacing: cty::c_int,
	pub monospaced: bool,
}

extern "C" {
	pub fn nSDL_LoadFont(font_index: i32, r: u8, g: u8, b: u8) -> *mut nSDL_Font;
	pub fn nSDL_DrawString(surface: *mut video::ll::SDL_Surface, font: *const nSDL_Font, x: i32, y:i32, str: *const cty::c_char);
}
