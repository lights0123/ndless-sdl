//! # Module for managing nSDL fonts
//! Example:
//! ```
//! let font = Font::new(FontOptions::Thin, 255, 255, 255);
//! screen.draw_str(&font, "message", 0, 0);
//! ```

pub mod ll {
	use crate::video;

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
		pub fn nSDL_DrawString(surface: *mut video::ll::SDL_Surface, font: *const nSDL_Font, x: i32, y: i32, str: *const cty::c_char);
		pub fn nSDL_FreeFont(font: *mut nSDL_Font);
	}
}

#[repr(i32)]
pub enum FontOptions {
	Thin = 0,
	Space,
	VGA,
	Fantasy,
	ThinType,
}

pub struct Font {
	pub font: *mut ll::nSDL_Font,
}

impl Font {
	pub fn new(font: FontOptions, r: u8, g: u8, b: u8) -> Self {
		let font = unsafe { ll::nSDL_LoadFont(font as i32, r, g, b) };
		if font.is_null() { panic!("Error loading font") }
		Self {
			font,
		}
	}
}

impl Drop for Font {
	fn drop(&mut self) {
		unsafe { ll::nSDL_FreeFont(self.font) }
	}
}
