//! `cell*_*` reimplemented functions.

#![allow(dead_code)]

use libc::strcmp;

use core::ptr::null_mut;

use crate::{
    c_api::{self, nccell_release},
    cstring, rstring, NcAlpha, NcCell, NcChannel, NcChannels, NcComponent, NcIntResult,
    NcIntResultApi, NcPaletteIndex, NcPlane, NcRgb, NcStyle, NcStyleApi,
};

const NCBOXLIGHT: &str = "┌┐└┘─│";
const NCBOXHEAVY: &str = "┏┓┗┛━┃";
const NCBOXROUND: &str = "╭╮╰╯─│";
const NCBOXDOUBLE: &str = "╔╗╚╝═║";
const NCBOXASCII: &str = "/\\\\/-|";
const NCBOXOUTER: &str = "🭽🭾🭼🭿▁🭵🭶🭰";

// Alpha -----------------------------------------------------------------------

/// Extracts the foreground [`NcAlpha`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[fg_alpha()][NcCell#method.fg_alpha].*
#[inline]
pub fn nccell_fg_alpha(cell: &NcCell) -> NcAlpha {
    c_api::ncchannels_fg_alpha(cell.channels)
}

/// Extracts the background [`NcAlpha`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[bg_alpha()][NcCell#method.bg_alpha].*
#[inline]
pub fn nccell_bg_alpha(cell: &NcCell) -> NcAlpha {
    c_api::ncchannels_bg_alpha(cell.channels)
}

/// Sets the foreground [`NcAlpha`] of an [`NcCell`].
///
/// *Method: NcCell.[set_fg_alpha()][NcCell#method.set_fg_alpha].*
#[inline]
pub fn nccell_set_fg_alpha(cell: &mut NcCell, alpha: NcAlpha) {
    c_api::ncchannels_set_fg_alpha(&mut cell.channels, alpha);
}

/// Sets the background [`NcAlpha`] of an [`NcCell`].
///
/// *Method: NcCell.[set_bg_alpha()][NcCell#method.set_bg_alpha].*
#[inline]
pub fn nccell_set_bg_alpha(cell: &mut NcCell, alpha: NcAlpha) {
    c_api::ncchannels_set_bg_alpha(&mut cell.channels, alpha);
}

// NcComponent ---------------------------------------------------------------------

/// Gets the foreground [`NcComponent`]s of an [`NcCell`],
/// and returns the [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcCell.[fg_rgb8()][NcCell#method.fg_rgb8].*
#[inline]
pub fn nccell_fg_rgb8(
    cell: &NcCell,
    red: &mut NcComponent,
    green: &mut NcComponent,
    blue: &mut NcComponent,
) -> NcChannel {
    c_api::ncchannels_fg_rgb8(cell.channels, red, green, blue)
}

/// Gets the background [`NcComponent`]s of an [`NcCell`],
/// and returns the [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcCell.[bg_rgb8()][NcCell#method.bg_rgb8].*
#[inline]
pub fn nccell_bg_rgb8(
    cell: &NcCell,
    red: &mut NcComponent,
    green: &mut NcComponent,
    blue: &mut NcComponent,
) -> NcChannel {
    c_api::ncchannels_bg_rgb8(cell.channels, red, green, blue)
}

/// Sets the foreground [`NcComponent`]s of the [`NcCell`],
/// and marks it as not using the "default color".
///
/// *Method: NcCell.[set_fg_rgb8()][NcCell#method.set_fg_rgb8].*
#[inline]
pub fn nccell_set_fg_rgb8(
    cell: &mut NcCell,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) {
    c_api::ncchannels_set_fg_rgb8(&mut cell.channels, red, green, blue);
}

/// Sets the background [`NcComponent`]s of the [`NcCell`],
/// and marks it as not using the "default color".
///
/// *Method: NcCell.[set_bg_rgb8()][NcCell#method.set_bg_rgb8].*
#[inline]
pub fn nccell_set_bg_rgb8(
    cell: &mut NcCell,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) {
    c_api::ncchannels_set_bg_rgb8(&mut cell.channels, red, green, blue);
}

// NcRgb -----------------------------------------------------------------------

/// Gets the foreground [`NcRgb`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[fg_rgb()][NcCell#method.fg_rgb].*
#[inline]
pub fn nccell_fg_rgb(cell: &NcCell) -> NcRgb {
    c_api::ncchannels_fg_rgb(cell.channels)
}

/// Gets the background [`NcRgb`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[bg_rgb()][NcCell#method.bg_rgb].*
#[inline]
pub fn nccell_bg_rgb(cell: &NcCell) -> NcRgb {
    c_api::ncchannels_bg_rgb(cell.channels)
}

/// Sets the foreground [`NcRgb`] of an [`NcCell`],
/// and marks it as not using the default color.
///
/// *Method: NcCell.[set_fg_rgb()][NcCell#method.set_fg_rgb].*
#[inline]
pub fn nccell_set_fg_rgb(cell: &mut NcCell, rgb: NcRgb) {
    c_api::ncchannels_set_fg_rgb(&mut cell.channels, rgb);
}

/// Sets the background [`NcRgb`] of an [`NcCell`],
/// and marks it as not using the default color.
///
/// *Method: NcCell.[set_bg_rgb()][NcCell#method.set_bg_rgb].*
#[inline]
pub fn nccell_set_bg_rgb(cell: &mut NcCell, rgb: NcRgb) {
    c_api::ncchannels_set_bg_rgb(&mut cell.channels, rgb);
}

// Default ---------------------------------------------------------------------

/// Indicates to use the "default color" for the foreground [`NcChannel`]
/// of an [`NcCell`].
///
/// *Method: NcCell.[set_fg_default()][NcCell#method.set_fg_default].*
#[inline]
pub fn nccell_set_fg_default(cell: &mut NcCell) {
    c_api::ncchannels_set_fg_default(&mut cell.channels);
}

/// Indicates to use the "default color" for the background [`NcChannel`]
/// of an [`NcCell`].
///
/// *Method: NcCell.[set_bg_default()][NcCell#method.set_bg_default].*
#[inline]
pub fn nccell_set_bg_default(cell: &mut NcCell) {
    c_api::ncchannels_set_bg_default(&mut cell.channels);
}

/// Is the foreground [`NcChannel`] of this [`NcCell`] using the
/// "default foreground color"?
///
/// *Method: NcCell.[fg_default_p()][NcCell#method.fg_default_p].*
#[inline]
pub fn nccell_fg_default_p(cell: &NcCell) -> bool {
    c_api::ncchannels_fg_default_p(cell.channels)
}

/// Is the background [`NcChannel`] of this [`NcCell`] using the
/// "default background color"?
///
/// The "default background color" must generally be used to take advantage of
/// terminal-effected transparency.
///
/// *Method: NcCell.[bg_default_p()][NcCell#method.bg_default_p].*
#[inline]
pub fn nccell_bg_default_p(cell: &NcCell) -> bool {
    c_api::ncchannels_bg_default_p(cell.channels)
}

// Palette ---------------------------------------------------------------------

/// Is the foreground [`NcChannel`] of this [`NcCell`] using an
/// [`NcPaletteIndex`] indexed [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcCell.[fg_palindex_p()][NcCell#method.fg_palindex_p].*
#[inline]
pub fn nccell_fg_palindex_p(cell: &NcCell) -> bool {
    c_api::ncchannels_fg_palindex_p(cell.channels)
}

/// Is the background [`NcChannel`] of this [`NcCell`] using an
/// [`NcPaletteIndex`] indexed [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcCell.[bg_palindex_p()][NcCell#method.bg_palindex_p].*
#[inline]
pub fn nccell_bg_palindex_p(cell: &NcCell) -> bool {
    c_api::ncchannels_bg_palindex_p(cell.channels)
}

/// Gets the [`NcPaletteIndex`] of the foreground [`NcChannel`] of the [`NcCell`].
///
/// *Method: NcCell.[fg_palindex()][NcCell#method.fg_palindex].*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn nccell_fg_palindex(cell: &NcCell) -> NcPaletteIndex {
    ((cell.channels & 0xff00000000 as NcChannels) >> 32) as NcPaletteIndex
}

/// Gets the [`NcPaletteIndex`] of the background [`NcChannel`] of the [`NcCell`].
///
/// *Method: NcCell.[bg_palindex()][NcCell#method.bg_palindex].*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn nccell_bg_palindex(cell: &NcCell) -> NcPaletteIndex {
    (cell.channels & 0xff) as NcPaletteIndex
}

/// Sets an [`NcCell`]'s foreground [`NcPaletteIndex`].
///
/// Also sets [`NcChannels::FG_PALETTE`][NcChannels#associatedconstant.FG_PALETTE]
/// and [`NcAlpha::OPAQUE`][NcAlpha#associatedconstant::OPAQUE], and clears out
/// [`NcChannels::FG_DEFAULT_MASK`][NcChannels#associatedconstant.FG_DEFAULT_MASK].
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcCell.[set_fg_palindex()][NcCell#method.set_fg_palindex].*
#[inline]
pub fn nccell_set_fg_palindex(cell: &mut NcCell, index: NcPaletteIndex) {
    c_api::ncchannels_set_fg_palindex(&mut cell.channels, index)
}

/// Sets an [`NcCell`]'s background [`NcPaletteIndex`].
///
/// Also sets [`NcChannels::BG_PALETTE`][NcChannels#associatedconstant.BG_PALETTE]
/// and [`NcAlpha::OPAQUE`][NcAlpha#associatedconstant::OPAQUE], and clears out
/// [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK].
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcCell.[set_bg_palindex()][NcCell#method.set_bg_palindex].*
#[inline]
pub fn nccell_set_bg_palindex(cell: &mut NcCell, index: NcPaletteIndex) {
    c_api::ncchannels_set_bg_palindex(&mut cell.channels, index)
}

// Styles ----------------------------------------------------------------------

/// Extracts the [`NcStyle`] bits from an [`NcCell`].
///
/// *Method: NcCell.[cell_styles()][NcCell#method.cell_styles].*
#[inline]
pub const fn nccell_styles(cell: &NcCell) -> NcStyle {
    cell.stylemask
}

/// Adds the specified [`NcStyle`] bits to an [`NcCell`]'s existing spec.,
/// whether they're actively supported or not.
///
/// *Method: NcCell.[styles_on()][NcCell#method.styles_on].*
#[inline]
pub fn nccell_on_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask |= stylebits & NcStyle::MASK as u16;
}

/// Removes the specified [`NcStyle`] bits from an [`NcCell`]'s existing spec.
///
/// *Method: NcCell.[styles_off()][NcCell#method.styles_off].*
#[inline]
pub fn nccell_off_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask &= !(stylebits & NcStyle::MASK as u16);
}

/// Sets *just* the specified [`NcStyle`] bits for an [`NcCell`],
/// whether they're actively supported or not.
///
/// *Method: NcCell.[styles_set()][NcCell#method.styles_set].*
#[inline]
pub fn nccell_set_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask = stylebits & NcStyle::MASK as u16;
}

// Chars -----------------------------------------------------------------------

/// Returns the number of columns occupied by `cell`.
///
/// See [`ncstrwidth`][c_api::ncstrwidth] for an equivalent for multiple EGCs.
#[inline]
pub const fn nccell_cols(cell: &NcCell) -> u8 {
    if cell.width != 0 {
        cell.width
    } else {
        1
    }
}

/// Returns the number of columns occupied by a string, or -1 if a
/// non-printable/illegal character is encountered.
pub fn ncstrwidth(string: &str) -> NcIntResult {
    unsafe { c_api::ncstrwidth_valid(cstring![string], null_mut(), null_mut()) }
}

/// Does the [`NcCell`] contain an East Asian Wide codepoint?
///
/// *Method: NcCell.[double_wide_p()][NcCell#method.double_wide_p].*
#[inline]
pub const fn nccell_double_wide_p(cell: &NcCell) -> bool {
    cell.width > 0
}

/// Is this the right half of a wide character?
///
/// *Method: NcCell.[wide_right_p()][NcCell#method.wide_right_p].*
#[inline]
pub const fn nccell_wide_right_p(cell: &NcCell) -> bool {
    nccell_double_wide_p(cell) && cell.gcluster == 0
}

/// Is this the left half of a wide character?
///
/// *Method: NcCell.[wide_left_p()][NcCell#method.wide_left_p].*
#[inline]
pub const fn nccell_wide_left_p(cell: &NcCell) -> bool {
    nccell_double_wide_p(cell) && cell.gcluster != 0
}

/// Copies the UTF8-encoded `EGC` out of the [`NcCell`], whether simple or complex.
///
/// The result is not tied to the [NcPlane],
/// and persists across erases and destruction.
///
/// *Method: NcCell.[strdup()][NcCell#method.strdup].*
#[inline]
pub fn nccell_strdup(plane: &NcPlane, cell: &NcCell) -> String {
    rstring![libc::strdup(c_api::nccell_extended_gcluster(plane, cell))].into()
}

// Misc. -----------------------------------------------------------------------

/// Saves the [`NcStyle`] and the [`NcChannels`],
/// and returns the `EGC`, of an [`NcCell`].
///
/// *Method: NcCell.[extract()][NcCell#method.extract].*
#[inline]
pub fn nccell_extract(
    plane: &NcPlane,
    cell: &NcCell,
    stylemask: &mut NcStyle,
    channels: &mut NcChannels,
) -> String {
    *stylemask = cell.stylemask;
    *channels = cell.channels;
    nccell_strdup(plane, cell)
}

/// Returns true if the two cells are distinct `EGC`s, attributes, or channels.
///
/// The actual egcpool index needn't be the same--indeed, the planes needn't even
/// be the same. Only the expanded EGC must be equal. The EGC must be bit-equal;
///
/// *Method: NcCell.[compare()][NcCell#method.compare].*
//
// NOTE: FIXME: it would probably be better to test whether they're Unicode-equal
#[inline]
pub fn nccellcmp(plane1: &NcPlane, cell1: &NcCell, plane2: &NcPlane, cell2: &NcCell) -> bool {
    if cell1.stylemask != cell2.stylemask {
        return true;
    }
    if cell1.channels != cell2.channels {
        return true;
    }
    unsafe {
        strcmp(
            c_api::nccell_extended_gcluster(plane1, cell1),
            c_api::nccell_extended_gcluster(plane2, cell2),
        ) != 0
    }
}

/// Initializes (zeroes out) an [`NcCell`].
///
/// *Method: NcCell.[init()][NcCell#method.init].*
#[inline]
pub fn nccell_init(cell: &mut NcCell) {
    *cell = unsafe { core::mem::zeroed() }
}

/// Same as [`nccell_load`][c_api::nccell_load], plus blasts the styling with
/// `style` and `channels`.
///
/// - Breaks the UTF-8 string in `gcluster` down, setting up the cell `cell`.
/// - Returns the number of bytes copied out of `gcluster`, or -1 on failure.
/// - The styling of the cell is left untouched, but any resources are released.
/// - Blasts the styling with `style` and `channels`.
///
/// *Method: NcCell.[prime()][NcCell#method.prime].*
#[inline]
pub fn nccell_prime(
    plane: &mut NcPlane,
    cell: &mut NcCell,
    gcluster: &str,
    style: NcStyle,
    channels: NcChannels,
) -> NcIntResult {
    cell.stylemask = style;
    cell.channels = channels;
    unsafe { c_api::nccell_load(plane, cell, cstring![gcluster]) }
}

/// Loads up six cells with the `EGC`s necessary to draw a box.
///
/// Returns [`NcIntResult::OK`][NcIntResult#associatedconstant.OK] on success
/// or [`NcIntResult::ERR`][NcIntResult#associatedconstant.ERR] on error.
///
/// On error, any [`NcCell`]s this function might have loaded before the error
/// are [nccell_release]d. There must be at least six `EGC`s in `gcluster`.
///
/// *Method: NcCell.[load_box()][NcCell#method.load_box].*
#[inline]
pub fn nccells_load_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
    gcluster: &str,
) -> NcIntResult {
    assert![gcluster.len() >= 6]; // DEBUG

    // TODO: CHECK: mutable copy for pointer arithmetics:
    let mut gclu = cstring![gcluster];

    let mut ulen: NcIntResult;

    ulen = nccell_prime(plane, ul, gcluster, style, channels);

    if ulen > 0 {
        gclu = unsafe { gclu.offset(ulen as isize) };
        ulen = nccell_prime(plane, ur, gcluster, style, channels);

        if ulen > 0 {
            gclu = unsafe { gclu.offset(ulen as isize) };
            ulen = nccell_prime(plane, ll, gcluster, style, channels);

            if ulen > 0 {
                gclu = unsafe { gclu.offset(ulen as isize) };
                ulen = nccell_prime(plane, lr, gcluster, style, channels);

                if ulen > 0 {
                    gclu = unsafe { gclu.offset(ulen as isize) };
                    ulen = nccell_prime(plane, hl, gcluster, style, channels);

                    if ulen > 0 {
                        let _gclu = unsafe { gclu.offset(ulen as isize) };
                        ulen = nccell_prime(plane, vl, gcluster, style, channels);

                        if ulen > 0 {
                            return NcIntResult::OK;
                        }
                        unsafe {
                            nccell_release(plane, hl);
                        }
                    }
                    unsafe {
                        nccell_release(plane, lr);
                    }
                }
                unsafe {
                    nccell_release(plane, ll);
                }
            }
            unsafe {
                nccell_release(plane, ur);
            }
        }
        unsafe {
            nccell_release(plane, ul);
        }
    }
    NcIntResult::ERR
}

/// [`nccells_load_box`] with ASCII characters.
///
/// *Method: NcCell.[ascii_box()][NcCell#method.ascii_box].*
#[inline]
pub fn nccells_ascii_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
) -> NcIntResult {
    nccells_load_box(plane, style, channels, ul, ur, ll, lr, hl, vl, NCBOXASCII)
}

/// [`nccells_load_box`] with double line box-drawing characters.
///
/// *Method: NcCell.[double_box()][NcCell#method.double_box].*
#[inline]
pub fn nccells_double_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
) -> NcIntResult {
    nccells_load_box(plane, style, channels, ul, ur, ll, lr, hl, vl, NCBOXDOUBLE)
}

/// [`nccells_load_box`] with heavy line box-drawing characters.
///
/// *Method: NcCell.[heavy_box()][NcCell#method.heavy_box].*
#[inline]
pub fn nccells_heavy_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
) -> NcIntResult {
    nccells_load_box(plane, style, channels, ul, ur, ll, lr, hl, vl, NCBOXHEAVY)
}

/// [`nccells_load_box`] with light line box-drawing characters.
///
/// *Method: NcCell.[light_box()][NcCell#method.light_box].*
#[inline]
pub fn nccells_light_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
) -> NcIntResult {
    nccells_load_box(plane, style, channels, ul, ur, ll, lr, hl, vl, NCBOXLIGHT)
}

/// [`nccells_load_box`] with round line box-drawing characters.
///
/// *Method: NcCell.[rounded_box()][NcCell#method.rounded_box].*
#[inline]
pub fn nccells_rounded_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
) -> NcIntResult {
    nccells_load_box(plane, style, channels, ul, ur, ll, lr, hl, vl, NCBOXROUND)
}
