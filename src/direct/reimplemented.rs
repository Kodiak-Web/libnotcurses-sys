//! `ncdirect_*` reimplemented functions.

use core::ptr::{null, null_mut};

use crate::{
    c_api, cstring, NcCapabilities, NcChannels, NcComponent, NcDim, NcDirect, NcInput, NcIntResult,
    NcRgb, NcTime,
};

/// Can we directly specify RGB values per cell, or only use palettes?
#[inline]
pub fn ncdirect_cantruecolor(ncd: &NcDirect) -> bool {
    ncdirect_capabilities(ncd).rgb
}

/// Can we set the "hardware" palette? Requires the "ccc" terminfo capability.
#[inline]
pub fn ncdirect_canchangecolor(ncd: &NcDirect) -> bool {
    c_api::nccapability_canchangecolor(&ncdirect_capabilities(ncd))
}

/// Can we fade? Fading requires either the "rgb" or "ccc" terminfo capability.
#[inline]
pub fn ncdirect_canfade(ncd: &NcDirect) -> bool {
    ncdirect_canchangecolor(ncd) || ncdirect_cantruecolor(ncd)
}

/// Can we load videos? This requires being built against FFmpeg.
#[inline]
pub fn ncdirect_canopen_videos(_ncd: &NcDirect) -> bool {
    unsafe { c_api::notcurses_canopen_videos(null()) }
}

/// Can we reliably use Unicode halfblocks?
#[inline]
pub fn ncdirect_canhalfblock(ncd: &NcDirect) -> bool {
    unsafe { c_api::ncdirect_canutf8(ncd) }
}

/// Can we reliably use Unicode quadrants?
#[inline]
pub fn ncdirect_canquadrant(ncd: &NcDirect) -> bool {
    (unsafe { c_api::ncdirect_canutf8(ncd) }) && ncdirect_capabilities(ncd).quadrants
}

/// Can we reliably use Unicode 13 sextants?
#[inline]
pub fn ncdirect_cansextant(ncd: &NcDirect) -> bool {
    (unsafe { c_api::ncdirect_canutf8(ncd) }) && ncdirect_capabilities(ncd).sextants
}

/// Can we reliably use Unicode Braille?
#[inline]
pub fn ncdirect_canbraille(_ncd: &NcDirect) -> bool {
    unsafe { c_api::notcurses_canbraille(null()) }
}

/// Returns the detected [`NcCapabilities`].
#[inline]
pub fn ncdirect_capabilities(ncd: &NcDirect) -> NcCapabilities {
    unsafe { *crate::bindings::ffi::ncdirect_capabilities(ncd) }
}

/// Reads input blocking until an event is processed or a signal is received.
///
/// Will optionally write the event details in `input`.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: NcDirect.[getc_blocking()][NcDirect#method.getc_blocking].*
#[inline]
pub fn ncdirect_getc_blocking(ncd: &mut NcDirect, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe { c_api::ncdirect_get(ncd, null(), input_ptr) as NcIntResult }
}

/// Reads input without blocking.
///
/// Will optionally write the event details in `input`.
///
/// If no event is ready, returns 0.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: NcDirect.[getc_nblock()][NcDirect#method.getc_nblock].*
#[inline]
pub fn ncdirect_getc_nblock(ncd: &mut NcDirect, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe {
        let ts = NcTime::new(0, 0);
        c_api::ncdirect_get(ncd, &ts, input_ptr) as NcIntResult
    }
}

/// Sets the foreground [NcComponent] components.
///
/// *Method: NcDirect.[set_fg_rgb8()][NcDirect#method.set_fg_rgb8].*
#[inline]
pub fn ncdirect_set_fg_rgb8(
    ncd: &mut NcDirect,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) -> NcIntResult {
    let rgb = (red as NcRgb) << 16 | (green as NcRgb) << 8 | blue as NcRgb;
    unsafe { c_api::ncdirect_set_fg_rgb(ncd, rgb) }
}

/// Sets the background [NcComponent] components.
///
/// *Method: NcDirect.[set_bg_rgb8()][NcDirect#method.set_bg_rgb8].*
#[inline]
pub fn ncdirect_set_bg_rgb8(
    ncd: &mut NcDirect,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) -> NcIntResult {
    let rgb = (red as NcRgb) << 16 | (green as NcRgb) << 8 | blue as NcRgb;
    unsafe { c_api::ncdirect_set_bg_rgb(ncd, rgb) }
}

/// Draws horizontal lines using the specified [NcChannels]s, interpolating
/// between them as we go.
///
/// The string at `egc` may not use more than one column.
///
/// All lines start at the current cursor position.
///
/// For a horizontal line, `len` cannot exceed the screen width minus the
/// cursor's offset.
// TODO:MAYBE saturate the `len` value
///
/// *Method: NcDirect.[hline_interp()][NcDirect#method.hline_interp].*
#[inline]
pub fn ncdirect_hline_interp(
    ncd: &mut NcDirect,
    egc: &str,
    len: NcDim,
    h1: NcChannels,
    h2: NcChannels,
) -> NcIntResult {
    #[cfg(any(target_arch = "armv7l", target_arch = "i686"))]
    let egc_ptr = cstring![egc] as *const i8;
    #[cfg(not(any(target_arch = "armv7l", target_arch = "i686")))]
    let egc_ptr = cstring![egc];

    unsafe { crate::bindings::ffi::ncdirect_hline_interp(ncd, egc_ptr, len as i32, h1, h2) }
}

/// Draws horizontal lines using the specified [NcChannels]s, interpolating
/// between them as we go.
///
/// The string at `egc` may not use more than one column.
///
/// All lines start at the current cursor position.
///
/// For a vertical line, `len` may be as long as you'd like; the screen
/// will scroll as necessary.
///
/// *Method: NcDirect.[vline_interp()][NcDirect#method.vline_interp].*
#[inline]
pub fn ncdirect_vline_interp(
    ncd: &mut NcDirect,
    egc: &str,
    len: NcDim,
    h1: NcChannels,
    h2: NcChannels,
) -> NcIntResult {
    #[cfg(any(target_arch = "armv7l", target_arch = "i686"))]
    let egc_ptr = cstring![egc] as *const i8;
    #[cfg(not(any(target_arch = "armv7l", target_arch = "i686")))]
    let egc_ptr = cstring![egc];

    unsafe { crate::bindings::ffi::ncdirect_vline_interp(ncd, egc_ptr, len as i32, h1, h2) }
}
