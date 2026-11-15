#![allow(unused)]

use citro2d::{Citro2d, Color, RenderTarget};
use ctru::prelude::*;

/// Width of top screen in pixels
pub const WIDTH_TOP_SCREEN: u16 = 800 / 2;

/// Height of top screen in pixels
pub const HEIGHT_TOP_SCREEN: u16 = 240 / 2;

/// Width of bottom screen in pixels
pub const WIDTH_BOTTOM_SCREEN: u16 = 320 / 2;

/// Height of bottom screen in pixels
pub const HEIGHT_BOTTOM_SCREEN: u16 = 240 / 2;

/// Width of top screen in pixels
pub const WIDTH_TOP_SCREEN_F32: f32 = WIDTH_TOP_SCREEN as f32;

/// Height of top screen in pixels
pub const HEIGHT_TOP_SCREEN_F32: f32 = HEIGHT_TOP_SCREEN as f32;

/// Width of bottom screen in pixels
pub const WIDTH_BOTTOM_SCREEN_F32: f32 = WIDTH_BOTTOM_SCREEN as f32;

/// Height of bottom screen in pixels
pub const HEIGHT_BOTTOM_SCREEN_F32: f32 = HEIGHT_BOTTOM_SCREEN as f32;

/// Clears a render target given a `Color` to fill the scene with.
pub fn clear(c2d: &Citro2d<'_>, target: &RenderTarget, clear_color: Color) {
    c2d.frame(|frame| frame.scene(target, clear_color, |_| {}));
}

/// Clears the top screen given a `Color` to fill the scene with.
pub fn clear_top_screen(c2d: &Citro2d<'_>, clear_color: Color) {
    clear(c2d, c2d.top_screen(), clear_color);
}

/// Clears the bottom screen given a `Color` to fill the scene with.
pub fn clear_bottom_screen(c2d: &Citro2d<'_>, clear_color: Color) {
    clear(c2d, c2d.bottom_screen(), clear_color);
}

/// Returns the where the touch screen is being touched given a `KeyPad`.
/// If you got the `KeyPad` via `Hid::keys_down` then the touch screen was
/// only JUST pressed this frame, but wasn't pressed last frame.
/// See `Hid::keys_held` and `Hid::keys_up` as well, since those give additional context.
pub fn touch_screen_position(keys: KeyPad, hid: &Hid) -> Option<(u16, u16)> {
    keys.contains(KeyPad::TOUCH).then(|| hid.touch_position())
}

/// Returns the where the touch screen is being touched given a `KeyPad`.
///
/// Unlike `touch_screen_position`, this returns the position as `f32`s,
/// which can be convenient for c2d and citro2d functions that expect `f32`s.
///
/// If you got the `KeyPad` via `Hid::keys_down` then the touch screen was
/// only JUST pressed this frame, but wasn't pressed last frame.
/// See `Hid::keys_held` and `Hid::keys_up` as well, since those give additional context.
pub fn touch_screen_position_f32(keys: KeyPad, hid: &Hid) -> Option<(f32, f32)> {
    touch_screen_position(keys, hid).map(|(a, b)| (a.into(), b.into()))
}
