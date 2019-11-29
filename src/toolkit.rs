// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Toolkit interface
//!
//! In KAS, the "toolkit" is an external library handling system interfaces
//! (windowing and event translation) plus rendering. This allows KAS's core
//! to remain system-neutral.
//!
//! Note: although the choice of windowing library is left to the toolkit, for
//! convenience KAS is able to use several [winit] types.
//!
//! [winit]: https://github.com/rust-windowing/winit

use crate::layout;
use crate::{event, Widget};

/// Toolkit actions needed after event handling, if any.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TkAction {
    /// No action needed
    None,
    /// Whole window requires redrawing
    ///
    /// Note that [`TkWindow::redraw`] can instead be used for more selective
    /// redrawing, if supported by the toolkit.
    Redraw,
    /// Whole window requires reconfigure *and* redrawing
    Reconfigure,
    /// Window should be closed
    Close,
    /// All windows should close (toolkit exit)
    CloseAll,
}

/// Toolkit-specific window management and style interface.
///
/// This is implemented by a KAS toolkit on a window handle. Since each window
/// is assumed to have uniform styling and this styling is provided by the
/// toolkit, this interface includes widget styling ([`TkWindow::size_rules`]).
///
/// Users interact with this trait in a few cases, such as implementing widget
/// event handling. In these cases the user is *always* given an existing
/// reference to a `TkWindow`. Mostly this trait is only used internally.
pub trait TkWindow {
    /// Read access to the event manager state
    fn data(&self) -> &event::Manager;

    /// Update event manager data with a closure
    ///
    /// The closure should return true if this update may require a redraw.
    fn update_data(&mut self, f: &mut dyn FnMut(&mut event::Manager) -> bool);

    /// Margin sizes
    ///
    /// May be called multiple times during a resize operation.
    ///
    /// See documentation of [`layout::Margins`].
    fn margins(&self, widget: &dyn Widget) -> layout::Margins;

    /// Widget size preferences
    ///
    /// Widgets should expect this to be called at least once for each axis.
    ///
    /// See documentation of [`layout::SizeRules`].
    fn size_rules(&mut self, widget: &dyn Widget, axis: layout::AxisInfo) -> layout::SizeRules;

    /// Notify that a widget must be redrawn
    fn redraw(&mut self, widget: &dyn Widget);

    /// Notify that a toolkit action should happen
    ///
    /// Allows signalling application exit, etc.
    fn send_action(&mut self, action: TkAction);
}
