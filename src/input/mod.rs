//! `NcInput` & `NcKey`

// functions manually reimplemented: 4
// ------------------------------------------
// (+) done: 4
// (#) test: 0
// ------------------------------------------
// + ncinput_equal_p
// + ncinput_nomod_p
// + nckey_mouse_p
// + nckey_supppuab_p

use crate::NcDim;

mod keycodes;
pub use keycodes::*;

/// Reads and decodes input events.
///
/// Reads from stdin and decodes the input to stdout, including synthesized
/// events and mouse events. Notcurses provides input from keyboards and mice.
/// Single Unicode codepoints are received from the keyboard, directly encoded
/// as `u32`.
///
/// The input system must deal with numerous keyboard signals which do not map
/// to Unicode code points. This includes the keypad arrows and function keys.
/// These "synthesized" codepoints are enumerated in , and mapped into the
/// Supplementary Private Use Area-B (U+100000..U+10FFFD).
/// Mouse button events are similarly mapped into the SPUA-B.
///
/// All events carry a ncinput structure with them.
/// For mouse events, the x and y coordinates are reported within this struct.
/// For all events, modifiers (e.g. "Alt") are carried as bools in this struct.
pub type NcInput = crate::bindings::ffi::ncinput;

/// New NcInput.
impl NcInput {
    /// New empty NcInput.
    pub const fn new_empty() -> NcInput {
        NcInput {
            id: 0,
            y: 0,
            x: 0,
            alt: false,
            shift: false,
            ctrl: false,
            evtype: NCEVTYPE_UNKNOWN,
        }
    }

    /// New NcInput.
    pub const fn new(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, false, 0)
    }

    /// New NcInput with alt key.
    pub const fn with_alt(id: char) -> NcInput {
        Self::with_all_args(id, None, None, true, false, false, 0)
    }

    /// New NcInput with shift key.
    pub const fn with_shift(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, true, false, 0)
    }

    /// New NcInput with ctrl key.
    pub const fn with_ctrl(id: char) -> NcInput {
        Self::with_all_args(id, None, None, false, false, true, 0)
    }

    /// New NcInput, expecting all the arguments.
    pub const fn with_all_args(
        id: char,
        x: Option<NcDim>,
        y: Option<NcDim>,
        alt: bool,
        shift: bool,
        ctrl: bool,
        evtype: NcEvType,
    ) -> NcInput {
        let (ix, iy);
        if let Some(x) = x {
            ix = x as i32
        } else {
            ix = -1
        };
        if let Some(y) = y {
            iy = y as i32
        } else {
            iy = -1
        };

        NcInput {
            id: id as u32,
            y: ix,
            x: iy,
            alt,
            shift,
            ctrl,
            evtype,
        }
    }
}

/// The type of the event, part of [`NcInput`].
///
/// ## Defined constants
/// - [`NCEVTYPE_UNKNOWN`]
/// - [`NCEVTYPE_PRESS`]
/// - [`NCEVTYPE_REPEAT`]
/// - [`NCEVTYPE_RELEASE`]
pub type NcEvType = u32;

/// *Unknown* type event ([`NcEvType`]).
pub const NCEVTYPE_UNKNOWN: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_UNKNOWN;

/// *Press* type event ([`NcEvType`]).
pub const NCEVTYPE_PRESS: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_PRESS;

/// *Repeat* type event ([`NcEvType`]).
pub const NCEVTYPE_REPEAT: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_REPEAT;

/// *Release* type event ([`NcEvType`]).
pub const NCEVTYPE_RELEASE: NcEvType = crate::bindings::ffi::ncinput_NCTYPE_RELEASE;

/// Compares two NcInput structs for data equality.
///
/// Returns true if the two are data-equivalent.
pub const fn ncinput_equal_p(n1: NcInput, n2: NcInput) -> bool {
    if n1.id != n2.id {
        return false;
    }
    if n1.y != n2.y || n1.x != n2.x {
        return false;
    }
    if n1.alt != n2.alt || n1.shift != n2.shift || n1.ctrl != n2.ctrl {
        return false;
    }
    if n1.evtype != n2.evtype {
        return false
    }
    true
}

/// Are all the modifiers off (alt, control, shift)?
pub const fn ncinput_nomod_p(input: &NcInput) -> bool {
    !input.alt && !input.ctrl && !input.shift
}

/// Is this [char] a Supplementary Private Use Area-B codepoint?
///
/// Links:
/// - <https://en.wikipedia.org/wiki/Private_Use_Areas>
/// - <https://codepoints.net/supplementary_private_use_area-b>
#[inline]
pub const fn nckey_supppuab_p(w: char) -> bool {
    w as u32 >= 0x100000_u32 && w as u32 <= 0x10fffd_u32
}

/// Is the event a synthesized mouse event?
#[inline]
pub const fn nckey_mouse_p(r: char) -> bool {
    r >= NCKEY_BUTTON1 && r <= NCKEY_BUTTON11
}
