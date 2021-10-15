//! `NcReader*` methods and associated functions.

use super::{NcReader, NcReaderOptions};
use crate::{c_api::ncreader_create, error_ref_mut, NcPlane, NcResult};

/// # `NcReaderOptions` Constructors
impl NcReaderOptions {
    /// `NcReaderOptions` simple constructor.
    pub const fn new() -> Self {
        Self {
            // channels used for input
            tchannels: 0,
            // attributes used for input
            tattrword: 0,
            // bitfield of NCREADER_OPTION_*
            flags: 0,
        }
    }
}

/// # `NcReader` Constructors
impl NcReader {
    /// `NcReader` simple constructor.
    pub fn new<'a>(plane: &mut NcPlane) -> NcResult<&'a mut Self> {
        Self::with_options(plane, &NcReaderOptions::new())
    }

    /// `NcReader` constructor with options.
    pub fn with_options<'a>(
        plane: &mut NcPlane,
        options: &NcReaderOptions,
    ) -> NcResult<&'a mut Self> {
        error_ref_mut![unsafe { ncreader_create(plane, options) }]
    }
}
