//! `NcCell`

// functions already exported by bindgen : 5
// -----------------------------------------
// (W) wrap: 6
// (#) test: 3
// ------------------------------------------
//W  nccell_extended_gcluster
//W# nccell_load
//W# nccell_duplicate
//W# nccell_release
//   ncstrwidth_valid
//
// functions manually reimplemented: 48
// ------------------------------------------
// (X) wont:  2
// (+) done: 43
// (W) wrap: 43
// (#) test: 30
// ------------------------------------------
//W# nccell_bg_alpha
//W# nccell_bg_default_p
//W# nccell_bg_palindex
//W# nccell_bg_palindex_p
//W# nccell_bg_rgb
//W# nccell_bg_rgb8
//W+ nccell_cols
//W+ nccell_double_wide_p
//W# nccell_extract
//W# nccell_fchannel
//W# nccell_fg_alpha
//W# nccell_fg_default_p
//W# nccell_fg_palindex
//W# nccell_fg_palindex_p
//W# nccell_fg_rgb
//W# nccell_fg_rgb8
//W# nccell_init
//W# nccell_load_char
//W+ nccell_off_styles
//W+ nccell_on_styles
//W# nccell_prime
//W# nccell_set_bchannel
//W# nccell_set_bg_alpha
//W# nccell_set_bg_default
//W# nccell_set_bg_palindex
//W# nccell_set_bg_rgb
//W# nccell_set_bg_rgb8
// X nccell_set_bg_rgb8_clipped   // unneeded
//W# nccell_set_fchannel
//W# nccell_set_fg_alpha
//W# nccell_set_fg_default
//W# nccell_set_fg_palindex
//W# nccell_set_fg_rgb
//W# nccell_set_fg_rgb8
// X nccell_set_fg_rgb8_clipped   // unneeded
//W+ nccell_set_styles
//W+ nccell_strdup
//W# nccell_styles
//W+ nccell_wide_left_p
//W+ nccell_wide_right_p
//W+ nccellcmp
//W+ nccells_ascii_box
//W+ nccells_double_box
//W+ nccells_rounded_box
//W+ nccells_heavy_box
//W+ nccells_light_box
//W+ nccells_load_box
// + ncstrwidth

#[cfg(test)]
mod test;

mod methods;
pub(crate) mod reimplemented;

// NcCell
/// A coordinate on an [`NcPlane`][crate::NcPlane] storing 128 bits of data.
///
/// # Methods & Associated Functions
///
/// - [Constructors & Destructors](#nccell-constructors-and-destructors)
///
/// - [bg|fg `NcChannel`s manipulation](#nccell-methods-bgfg-ncchannels-manipulation)
/// - [Other components](#nccell-methods-other-components)
/// - [Text](#nccell-methods-text)
///
/// # Description
///
/// An `NcCell` corresponds to a single character cell on some `NcPlane`,
/// which can be occupied by a single `EGC` grapheme cluster (some root
/// spacing glyph, along with possible combining characters, which might span
/// multiple columns).
///
/// An `NcCell` is bounded to an `NcPlane`, but the cell doesn't store anything
/// about the plane.
///
/// At any `NcCell`, we can have a theoretically arbitrarily long UTF-8 string,
/// a foreground color, a background color, and an [`NcStyle`][crate::NcStyle] attribute set.
///
/// Valid grapheme cluster contents include:
///
/// - A NUL terminator,
/// - A single [control character](https://en.wikipedia.org/wiki/Control_character),
///   followed by a NUL terminator,
/// - At most one [spacing
/// character](https://en.wikipedia.org/wiki/Graphic_character#Spacing_character),
///   followed by zero or more nonspacing characters, followed by a NUL terminator.
///
/// ## Diagram
///
/// ```txt
/// `NcCell`: 128 bits structure comprised of the following 5 elements:
///
/// GCLUSTER|GCLUSTER|GCLUSTER|GCLUSTER  1. `EGC`
/// 00000000║WWWWWWWW║11111111|11111111  2. backstop + 3. width + 4. `NcStyle`
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB  5. `NcChannels`
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB     "
///
/// 1. (32b) Extended Grapheme Cluster, presented either as:
///
///     1.1. An EGC of up to 4 bytes:
///     UUUUUUUU|UUUUUUUU|UUUUUUUU|UUUUUUUU
///
///     1.2. A `0x01` in the first byte, plus 3 bytes with a 24b address to an egcpool:
///     00000001|IIIIIIII|IIIIIIII|IIIIIIII
///
/// 2. (8b) backstop (zero)
/// 00000000
///
/// 3. (8b) column width
/// WWWWWWWW
///
/// 4. (16b) `NcStyle`
/// 11111111 11111111
///
/// 5. (64b) `NcChannels`
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB║~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ```
///
/// `type in C: cell (struct)`
///
/// # More `NcCell` Information
///
/// ## Size
///
/// Multi-column characters can only have a single style/color throughout.
/// [`wcwidth()`](https://www.man7.org/linux/man-pages/man3/wcwidth.3.html)
/// is not reliable. It's just quoting whether or not the `EGC`
/// contains a "Wide Asian" double-width character.
/// This is set for some things, like most emoji, and not set for
/// other things, like cuneiform.
///
/// Each cell occupies 16 static bytes (128 bits). The surface is thus ~1.6MB
/// for a (pretty large) 500x200 terminal. At 80x43, it's less than 64KB.
/// Dynamic requirements (the egcpool) can add up to 16MB to an ncplane, but
/// such large pools are unlikely in common use.
///
/// ## Alpha Compositing
///
/// We implement some small alpha compositing. Foreground and background both
/// have two bits of inverted alpha. The actual grapheme written to a cell is
/// the topmost non-zero grapheme.
///
/// - If its alpha is 00
///   ([`NcAlpha::OPAQUE`][crate::NcAlpha#associatedconstant.OPAQUE])
///   its foreground color is used unchanged.
///
/// - If its alpha is 10
///   ([`NcAlpha::TRANSPARENT`][crate::NcAlpha#associatedconstant.TRANSPARENT])
///   its foreground color is derived
///   entirely from cells underneath it.
///
/// - If its alpha is 01
///   ([`NcAlpha::BLEND`][crate::NcAlpha#associatedconstant.BLEND])
///   the result will be a composite.
///
/// Likewise for the background. If the bottom of a coordinate's zbuffer is
/// reached with a cumulative alpha of zero, the default is used. In this way,
/// a terminal configured with transparent background can be supported through
/// multiple occluding ncplanes.
///
/// A foreground alpha of 11
/// ([`NcAlpha::HIGHCONTRAST`][crate::NcAlpha#associatedconstant.HIGHCONTRAST])
/// requests high-contrast text (relative to the computed background).
/// A background alpha of 11 is currently forbidden.
///
/// ## Precedence
///
/// - Default color takes precedence over palette or RGB, and cannot be used with
///   transparency.
/// - Indexed palette takes precedence over RGB. It cannot meaningfully set
///   transparency, but it can be mixed into a cascading color.
/// - RGB is used if neither default terminal colors nor palette indexing are in
///   play, and fully supports all transparency options.
///
/// ## Column width *(WIP)*
///
/// See [USAGE.md][0]
///
/// [0]: https://github.com/dankamongmen/notcurses/blob/master/USAGE.md#cells
///
/// We store the column width in this field. for a multicolumn EGC of N
/// columns, there will be N nccells, and each has a width of N...for now.
/// eventually, such an EGC will set more than one subsequent cell to
/// WIDE_RIGHT, and this won't be necessary. it can then be used as a
/// bytecount. see #1203. FIXME iff width >= 2, the cell is part of a
/// multicolumn glyph. whether a cell is the left or right side of the glyph
/// can be determined by checking whether ->gcluster is zero.
///
pub type NcCell = crate::bindings::ffi::nccell;

// RETHINK:
//
// NcEgc
//
// /// Extended Grapheme Cluster. A unicode string of length 1.
// ///
// /// This 32 bit char, together with the associated plane's associated egcpool,
// /// completely define this cell's `NcEgc`. Unless the `NcEgc` requires more than
// /// four bytes to encode as UTF-8, it will be inlined here:
// ///
// /// ## Diagram 1
// ///
// /// ```txt
// /// UUUUUUUU UUUUUUUU UUUUUUUU UUUUUUUU
// /// extended grapheme cluster <= 4bytes
// /// ```
// ///
// /// `type in C: uint32_t`
// ///
// /// If more than four bytes are required, it will be spilled into the egcpool.
// /// In either case, there's a NUL-terminated string available without copying,
// /// because (1) the egcpool is all NUL-terminated sequences and (2) the fifth
// /// byte of this struct (the backstop field) is guaranteed to be zero, as are
// /// any unused bytes in gcluster.
// ///
// /// A spilled `NcEgc` is indicated by the value `0x01iiiiii`. This cannot alias a
// /// true supra-ASCII NcEgc, because UTF-8 only encodes bytes <= 0x80 when they
// /// are single-byte ASCII-derived values. The `iiiiii` is interpreted as a 24-bit
// /// index into the egcpool (which may thus be up to 16MB):
// ///
// /// ## Diagram 2
// ///
// /// ```txt
// /// 00000001 iiiiiiii iiiiiiii iiiiiiii
// ///   sign     24bit index to egcpool
// /// ```
// /// `type in C: uint32_t`
// ///
// /// The cost of this scheme is that the character 0x01 (`SOH`) cannot be encoded
// /// in a cell, and therefore it must not be allowed through the API.
// ///
// /// -----
// /// Note that even if the `NcEgc` is <= 4 bytes and inlined, is still interpreted as
// /// a NUL-terminated char * (technically, &cell->gcluster is treated as a char*).
// /// If it is more than 4 bytes, cell->gcluster has a first byte of 0x01,
// /// and the remaining 24 bits are an index into the plane's egcpool,
// /// which is carved into NUL-terminated chunks of arbitrary length.
// ///
// /// ## Links
// ///
// /// - [Grapheme Cluster
// /// Boundaries](https://unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
// ///
// ///
// FIXME: should be an utf-8 string len 1 of type &str.
// pub type NcEgc = String;
// pub type NcEgc<'a> = &'a str;
