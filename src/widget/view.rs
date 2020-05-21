// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! View widgets
//!
//! View widgets allow separation of data and the view of that data.
//! There is some similarity to Model-View-Controller (MVC) design patterns,
//! but also some differences: the View widget handles (low level) user input,
//! returning (high level) messages to the Model. There is no explicit
//! Controller.
//!
//! This module provides View widgets and the message types used to communicate
//! with the Model. It is up to the user to implement the Model.
//! TODO: complete description.

use kas::prelude::*;
use kas::widget::{Column, Frame, Label, ScrollRegion};

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
#[layout(single)]
#[derive(Clone, Debug, Widget)]
pub struct ListView {
    #[widget_core]
    core: CoreData,
    #[widget]
    w: Frame<ScrollRegion<Column<Label>>>,
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
            w: Frame::new(scroll_region),
        }
    }

    /// Refresh the view
    ///
    /// This rebuilds the model from scratch and should be called on
    /// initialisation and on data model changes not communicated via another
    /// method. The view will appear empty until first refresh.
    #[inline]
    pub fn refresh(&mut self) -> (TkAction, ListViewMsg) {
        (self.w.inner.inner.clear(), ListViewMsg::DataRange)
    }
}

/// Data supply methods
///
/// These methods should only be called in response to a [`ListViewMsg`].
impl ListView {
    /// Provide the data range (number of rows)
    #[inline]
    pub fn data_range(&mut self, len: usize) -> ListViewMsg {
        self.w.inner.inner.reserve(len);
        ListViewMsg::DataRows(0, len)
    }

    /// Provide a single data row
    pub fn data_row<T: Into<CowString>>(&mut self, _index: usize, row: T) -> TkAction {
        // TODO: allow rows to be provided in any order
        self.w.inner.inner.push(Label::new(row.into()))
    }
}
