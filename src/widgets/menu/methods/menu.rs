use core::ptr::null_mut;

use crate::{
    c_api::{self, ncmenu_create},
    cstring, error, error_ref_mut, error_str, rstring,
    widgets::{NcMenu, NcMenuOptions},
    NcInput, NcPlane, NcResult,
};

#[allow(unused_imports)] // for doc comments
use crate::widgets::{NcMenuItem, NcMenuSection};

/// # `NcMenu` constructors & destructors
impl NcMenu {
    /// Creates an [`NcMenu`] with the specified options.
    ///
    /// Menus are currently bound to an overall [`Nc`][crate::Nc]
    /// object (as opposed to a particular plane), and are implemented as
    /// [`NcPlane`]s kept atop other NcPlanes.
    ///
    /// *C style function: [ncmenu_create()][c_api::ncmenu_create].*
    pub fn new<'a>(plane: &mut NcPlane, options: &NcMenuOptions) -> NcResult<&'a mut Self> {
        error_ref_mut![unsafe { ncmenu_create(plane, options) }, "Creating NcMenu"]
    }

    /// Destroys an `NcMenu` created with [`new`][NcMenu#method.new].
    ///
    /// *C style function: [ncmenu_destroy()][c_api::ncmenu_destroy].*
    pub fn destroy(&mut self) {
        unsafe { c_api::ncmenu_destroy(self) }
    }
}

/// # `NcMenu` methods
impl NcMenu {
    /// Disables or enables an [`NcMenuItem`].
    ///
    /// *C style function: [ncmenu_item_set_status()][c_api::ncmenu_item_set_status].*
    pub fn item_set_status(&mut self, section: &str, item: &str, enabled: bool) -> NcResult<()> {
        error![
            unsafe {
                c_api::ncmenu_item_set_status(self, cstring![section], cstring![item], enabled)
            },
            &format!(
                ".item_set_status({:?}, {:?}, {:?}, {})",
                self, section, item, enabled
            )
        ]
    }

    /// Returns the [`NcMenuItem`] description
    /// corresponding to the mouse `click`.
    ///
    /// The `NcMenuItem` must be on an actively unrolled section, and the click
    /// must be in the area of a valid item.
    ///
    /// If `ninput` is provided, and the selected item has a shortcut,
    /// it will be filled in with that shortcut.
    ///
    /// *C style function: [ncmenu_mouse_selected()][c_api::ncmenu_mouse_selected].*
    pub fn mouse_selected(
        &self,
        click: NcInput,
        shortcut: Option<&mut NcInput>,
    ) -> NcResult<String> {
        let ninput;
        if let Some(i) = shortcut {
            ninput = i as *mut _;
        } else {
            ninput = null_mut();
        }
        error_str![
            unsafe { c_api::ncmenu_mouse_selected(self, &click, ninput) },
            "Getting NcMenuItem description"
        ]
    }

    /// Moves to the next item within the currently unrolled section.
    ///
    /// If no section is unrolled, the first section will be unrolled.
    ///
    /// *C style function: [ncmenu_nextitem()][c_api::ncmenu_nextitem].*
    pub fn nextitem(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_nextitem(self) }]
    }

    /// Unrolls the next section (relative to current unrolled).
    ///
    /// If no section is unrolled, the first section will be unrolled.
    ///
    /// *C style function: [ncmenu_nextsection()][c_api::ncmenu_nextsection].*
    pub fn nextsection(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_nextsection(self) }]
    }

    /// Offers the `input` to this `NcMenu`.
    ///
    /// If it's relevant, this function returns true,
    /// and the input ought not be processed further.
    /// If it's irrelevant to the menu, false is returned.
    ///
    /// Relevant inputs include:
    /// - mouse movement over a hidden menu
    /// - a mouse click on a menu section (the section is unrolled)
    /// - a mouse click outside of an unrolled menu (the menu is rolled up)
    /// - left or right on an unrolled menu (navigates among sections)
    /// - up or down on an unrolled menu (navigates among items)
    /// - escape on an unrolled menu (the menu is rolled up)
    ///
    /// *C style function: [ncmenu_offer_input()][c_api::ncmenu_offer_input].*
    pub fn offer_input(&mut self, input: NcInput) -> bool {
        unsafe { c_api::ncmenu_offer_input(self, &input) }
    }

    /// Returns the [`NcPlane`] backing this `NcMenu`.
    ///
    /// *C style function: [ncmenu_plane()][c_api::ncmenu_plane].*
    pub fn plane(&mut self) -> NcResult<&NcPlane> {
        error_ref_mut![
            unsafe { c_api::ncmenu_plane(self) },
            "Getting the backing NcPlane"
        ]
    }

    /// Moves to the previous item within the currently unrolled section.
    ///
    /// If no section is unrolled, the first section will be unrolled.
    ///
    /// *C style function: [ncmenu_previtem()][c_api::ncmenu_previtem].*
    pub fn previtem(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_previtem(self) }]
    }

    /// Unrolls the previous section (relative to current unrolled).
    ///
    /// If no section is unrolled, the first section will be unrolled.
    ///
    /// *C style function: [ncmenu_prevsection()][c_api::ncmenu_prevsection].*
    pub fn prevsection(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_prevsection(self) }]
    }

    /// Rolls up any unrolled [`NcMenuSection`]
    /// and hides this `NcMenu` if using hiding.
    ///
    /// *C style function: [ncmenu_rollup()][c_api::ncmenu_rollup].*
    pub fn rollup(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_rollup(self) }]
    }

    /// Returns the selected item description, if there's an unrolled section.
    ///
    /// If `shortcut` is provided, and the selected item has a shortcut,
    /// it will be filled in with that shortcut--this can allow faster matching.
    ///
    /// *C style function: [ncmenu_selected()][c_api::ncmenu_selected].*
    pub fn selected(&mut self, shortcut: Option<&mut NcInput>) -> Option<String> {
        let ninput;
        if let Some(i) = shortcut {
            ninput = i as *mut _;
        } else {
            ninput = null_mut();
        }
        let res = unsafe { c_api::ncmenu_selected(self, ninput) };
        if !res.is_null() {
            Some(rstring![&*res].to_string())
        } else {
            None
        }
    }

    /// Unrolls the specified [`NcMenuSection`],
    /// making the menu visible if it was invisible
    /// and rolling up any `NcMenuSection` that is already unrolled.
    ///
    /// *C style function: [ncmenu_unroll()][c_api::ncmenu_unroll].*
    pub fn unroll(&mut self, sectionindex: u32) -> NcResult<()> {
        error![unsafe { c_api::ncmenu_unroll(self, sectionindex as i32) }]
    }
}
