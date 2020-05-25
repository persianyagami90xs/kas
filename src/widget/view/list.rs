// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! ListView widget
//! 
//! Rough implementation plan (see `data-views.md`):
//! 
//! 1.  Add a `ListView` widget based on `ScrollRegion<Column<Label>>`.
//!     Build the whole view on configure and refresh.
//! 2.  Use only enough child widgets for the visible window and re-allocate
//!     them when scrolling.
//! 3.  Support selection of items, where selection is a property of the view.
//!     (May require changes to `Layout::draw`.)
//! 4.  Add a `FixedRowLayout` or some such and support multiple columns of text.
//! 5.  Add headers; allow requesting sorting of the data set.
//! 6.  Support user-defined rows over a user-defined (row-based) data model.
//! 7.  Add example with a delay to data requests simulating remote data access.
//!     Tune the view for responsiveness with async data retrieval.
//! 8.  Plan next steps: tree views, flow views, 2D cellular (spreadsheet) views.

use kas::prelude::*;
use kas::widget::{Column, Label, ScrollRegion};


/// Messages returned from a view
///
/// The model is expected to respond these by calling the appropriate
/// [`ListView`] method. The view will tolerate responses being delayed or
/// dropped, though with some impairment to user experience.
/// Responses should not be reordered (except as noted).
///
/// Failing to observe above requirements may cause the view to behave
/// unexpectedly but may not cause violation of memory safety and should not
/// cause a fatal error.
#[must_use]
pub enum ListViewMsg {
    /// No request
    None,
    /// Request call to [`ListView::data_range`]
    DataRange,
    /// Request call to provide a range of row data
    ///
    /// This request may be fulfilled via calls to [`ListView::data_row`].
    DataRows(usize, usize),
}

/// A view over a list of text entries
///
/// This should be initialised with [`ListView::refresh`]. Before this method
/// is called, the view will appear empty. It is recommended to call `refresh`
/// before the UI starts (or before widget is added to the UI).
#[derive(Clone, Debug, Widget)]
pub struct ListView {
    #[widget_core]
    core: CoreData,
    #[widget]
    w: ScrollRegion<Column<Label>>,
    frame_offset: Coord,
    frame_size: Size,
}

impl Default for ListView {
    fn default() -> Self {
        ListView::new()
    }
}

impl ListView {
    /// Construct a list view
    ///
    /// The parent model is expected to call [`ListView::refresh`] to initialise
    /// the view within the parent's [`WidgetConfig::configure`] method.
    #[inline]
    pub fn new() -> Self {
        let scroll_region = ScrollRegion::default().with_auto_bars(true);
        ListView {
            core: Default::default(),
            w: scroll_region,
            frame_offset: Default::default(),
            frame_size: Default::default(),
        }
    }

    /// Refresh the view
    ///
    /// This rebuilds the model from scratch and should be called on
    /// initialisation and on data model changes not communicated via another
    /// method. The view will appear empty until first refresh.
    #[inline]
    pub fn refresh(&mut self) -> (TkAction, ListViewMsg) {
        (self.w.inner.clear(), ListViewMsg::DataRange)
    }
}

/// Data supply methods
///
/// These methods should only be called in response to a [`ListViewMsg`].
impl ListView {
    /// Provide the data range (number of rows)
    #[inline]
    pub fn data_range(&mut self, len: usize) -> ListViewMsg {
        self.w.inner.reserve(len);
        ListViewMsg::DataRows(0, len)
    }

    /// Provide a single data row
    pub fn data_row<T: Into<CowString>>(&mut self, _index: usize, row: T) -> TkAction {
        // TODO: allow rows to be provided in any order
        self.w.inner.push(Label::new(row.into()))
    }
}

impl Layout for ListView {
    fn size_rules(&mut self, size_handle: &mut dyn SizeHandle, axis: AxisInfo) -> SizeRules {
        let frame_sides = size_handle.edit_surround();
        let inner = size_handle.inner_margin();
        let frame_offset = frame_sides.0 + inner;
        let frame_size = frame_offset + frame_sides.1 + inner;

        let margins = size_handle.outer_margins();
        let frame_rules = SizeRules::extract_fixed(axis.is_vertical(), frame_size, margins);
        let content_rules = self.w.size_rules(size_handle, axis);

        let m = content_rules.margins();
        if axis.is_horizontal() {
            self.frame_offset.0 = frame_offset.0 as i32 + m.0 as i32;
            self.frame_size.0 = frame_size.0 + (m.0 + m.1) as u32;
        } else {
            self.frame_offset.1 = frame_offset.1 as i32 + m.0 as i32;
            self.frame_size.1 = frame_size.1 + (m.0 + m.1) as u32;
        }

        content_rules.surrounded_by(frame_rules, true)
    }

    fn set_rect(&mut self, rect: Rect, _: AlignHints) {
        self.core.rect = rect;
        let rect = Rect {
            pos: rect.pos + self.frame_offset,
            size: rect.size.saturating_sub(self.frame_size),
        };
        self.w.set_rect(rect, AlignHints::NONE);
    }

    fn find_id(&self, coord: Coord) -> Option<WidgetId> {
        if !self.rect().contains(coord) {
            return None;
        }
        if let Some(id) = self.w.find_id(coord) {
            return Some(id);
        }
        Some(self.id())
    }

    fn draw(&self, draw_handle: &mut dyn DrawHandle, mgr: &event::ManagerState, disabled: bool) {
        draw_handle.edit_box(self.core.rect, self.input_state(mgr, disabled));
        self.w.draw(draw_handle, mgr, disabled);
    }
}
