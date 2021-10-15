//! `NcPlot[F|U]64` widget.

/// A histogram, bound to an [`NcPlane`][crate::NcPlane]
/// (uses non-negative `f64`s)
pub type NcPlotF64 = crate::bindings::ffi::ncdplot;

/// A histogram, bound to an [`NcPlane`][crate::NcPlane] (uses `u64`s)
pub type NcPlotU64 = crate::bindings::ffi::ncuplot;

/// Options struct for
/// [`NcPlotF64`] or [`NcPlotU64`]
pub type NcPlotOptions = crate::bindings::ffi::ncplot_options;

impl NcPlotOptions {
    /// Use domain detection only for max
    pub const DETECTMAXONLY: u32 = constants::NCPLOT_OPTION_DETECTMAXONLY;

    /// Exponential dependent axis
    pub const EXPONENTIALD: u32 = constants::NCPLOT_OPTION_EXPONENTIALD;

    /// Show labels for dependent axis
    pub const LABELTICKSD: u32 = constants::NCPLOT_OPTION_LABELTICKSD;

    /// Use domain detection only for max
    pub const NODEGRADE: u32 = constants::NCPLOT_OPTION_NODEGRADE;

    /// Independent axis is vertical
    pub const VERTICALI: u32 = constants::NCPLOT_OPTION_VERTICALI;
}

pub(crate) mod constants {
    /// Use domain detection only for max
    pub const NCPLOT_OPTION_DETECTMAXONLY: u32 = crate::bindings::ffi::NCPLOT_OPTION_DETECTMAXONLY;

    /// Exponential dependent axis
    pub const NCPLOT_OPTION_EXPONENTIALD: u32 = crate::bindings::ffi::NCPLOT_OPTION_EXPONENTIALD;

    /// Show labels for dependent axis
    pub const NCPLOT_OPTION_LABELTICKSD: u32 = crate::bindings::ffi::NCPLOT_OPTION_LABELTICKSD;

    /// Use domain detection only for max
    pub const NCPLOT_OPTION_NODEGRADE: u32 = crate::bindings::ffi::NCPLOT_OPTION_NODEGRADE;

    /// Independent axis is vertical
    pub const NCPLOT_OPTION_VERTICALI: u32 = crate::bindings::ffi::NCPLOT_OPTION_VERTICALI;
}
