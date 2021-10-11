use crate::{c_api, Nc, NcDim, NcOffset, NcPlane, NcPlaneOptions};

/// Helper function for a new NcPlane on C style tests.
#[allow(dead_code)]
pub(crate) unsafe fn ncplane_new_test<'a>(
    nc: &mut Nc,
    y: NcOffset,
    x: NcOffset,
    rows: NcDim,
    cols: NcDim,
) -> &'a mut NcPlane {
    &mut *c_api::ncpile_create(nc, &NcPlaneOptions::new(y, x, rows, cols))
}

/// Helper function for a new bound NcPlane on C style tests.
#[allow(dead_code)]
pub(crate) unsafe fn ncplane_new_bound_test<'a>(
    plane: &mut NcPlane,
    y: NcOffset,
    x: NcOffset,
    rows: NcDim,
    cols: NcDim,
) -> &'a mut NcPlane {
    &mut *c_api::ncplane_create(plane, &NcPlaneOptions::new(y, x, rows, cols))
}
