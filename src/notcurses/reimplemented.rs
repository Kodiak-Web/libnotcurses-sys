//! `notcurses_*` reimplemented functions.

use core::ptr::{null, null_mut};

use crate::{
    Nc, NcAlign, NcDim, NcError, NcInput, NcIntResult, NcPlane, NcResult, NcTime, NCALIGN_BOTTOM,
    NCALIGN_CENTER, NCALIGN_LEFT, NCALIGN_RIGHT, NCALIGN_TOP, NCRESULT_MAX,
};

/// Returns the offset into `avail_u` at which `u` ought be output given
/// the requirements of `align`.
///
/// Returns `-`[`NCRESULT_MAX`] if [NCALIGN_UNALIGNED][crate::NCALIGN_UNALIGNED]
/// or invalid [NcAlign].
///
/// *Method: Nc.[align()][Nc#method.align].*
#[inline]
pub fn notcurses_align(avail_u: NcDim, align: NcAlign, u: NcDim) -> NcIntResult {
    if align == NCALIGN_LEFT || align == NCALIGN_TOP {
        return 0;
    }
    if align == NCALIGN_CENTER {
        return ((avail_u - u) / 2) as NcIntResult;
    }
    if align == NCALIGN_RIGHT || align == NCALIGN_BOTTOM {
        return (avail_u - u) as NcIntResult;
    }
    -NCRESULT_MAX
}

/// Reads input blocking until an event is processed or a signal is received.
///
/// Will optionally write the event details in `input`.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: Nc.[getc_blocking()][Nc#method.getc_blocking].*
#[inline]
pub fn notcurses_getc_blocking(nc: &mut Nc, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe { crate::notcurses_get(nc, null(), input_ptr) as NcIntResult }
}

/// Reads input without blocking.
///
/// Will optionally write the event details in `input`.
///
/// If no event is ready, returns 0.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: Nc.[getc_nblock()][Nc#method.getc_nblock].*
#[inline]
pub fn notcurses_getc_nblock(nc: &mut Nc, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe {
        let ts = NcTime::new(0, 0);
        crate::notcurses_get(nc, &ts, input_ptr) as NcIntResult
    }
}

/// [notcurses_stdplane()][crate::notcurses_stdplane], plus free bonus
/// dimensions written to non-NULL y/x!
///
/// *Method: Nc.[getc_stddim_yx()][Nc#method.stddim_yx].*
#[inline]
pub fn notcurses_stddim_yx<'a>(
    nc: &'a mut Nc,
    y: &mut NcDim,
    x: &mut NcDim,
) -> NcResult<&'a mut NcPlane> {
    unsafe {
        let sp = crate::notcurses_stdplane(nc);
        if !sp.is_null() {
            crate::ncplane_dim_yx(sp, &mut (*y as i32), &mut (*x as i32));
            return Ok(&mut *sp);
        }
    }
    Err(NcError::new())
}

/// [notcurses_stdplane_const()][crate::notcurses_stdplane_const], plus free
/// bonus dimensions written to non-NULL y/x!
///
/// *Method: Nc.[getc_stddim_yx_const()][Nc#method.stddim_yx_const].*
#[inline]
pub fn notcurses_stddim_yx_const<'a>(
    nc: &'a Nc,
    y: &mut NcDim,
    x: &mut NcDim,
) -> NcResult<&'a NcPlane> {
    unsafe {
        let sp = crate::notcurses_stdplane_const(nc);
        if !sp.is_null() {
            crate::ncplane_dim_yx(sp, &mut (*y as i32), &mut (*x as i32));
            return Ok(&*sp);
        }
    }
    Err(NcError::new())
}

/// Returns our current idea of the terminal dimensions in rows and cols.
///
/// *Method: Nc.[getc_term_yx()][Nc#method.term_yx].*
#[inline]
pub fn notcurses_term_dim_yx(nc: &Nc) -> (NcDim, NcDim) {
    let (mut y, mut x) = (0, 0);
    unsafe {
        crate::ncplane_dim_yx(crate::notcurses_stdplane_const(nc), &mut y, &mut x);
    }
    (y as NcDim, x as NcDim)
}
