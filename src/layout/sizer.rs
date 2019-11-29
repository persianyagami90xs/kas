// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Layout solver

use super::{AxisInfo, SizeRules};
use crate::geom::{Rect, Size};
use crate::{Layout, TkWindow};

pub trait Storage {}

/// A [`SizeRules`] solver for layouts
pub trait RulesSolver {
    /// Type of storage
    type Storage: Clone;

    /// Type required by [`RulesSolver::for_child`] (see implementation documentation)
    type ChildInfo;

    /// Called once for each child. For most layouts the order is important.
    fn for_child<C: Layout>(
        &mut self,
        tk: &mut dyn TkWindow,
        storage: &mut Self::Storage,
        child_info: Self::ChildInfo,
        child: &mut C,
    );

    /// Called at the end to output [`SizeRules`].
    ///
    /// Note that this does not include margins!
    fn finish<ColIter, RowIter>(
        self,
        tk: &mut dyn TkWindow,
        storage: &mut Self::Storage,
        col_spans: ColIter,
        row_spans: RowIter,
    ) -> SizeRules
    where
        ColIter: Iterator<Item = (usize, usize, usize)>,
        RowIter: Iterator<Item = (usize, usize, usize)>;
}

/// Dummy implementation
///
/// TODO: make real implementations to solve empty/single/derive cases?
impl RulesSolver for () {
    type Storage = ();
    type ChildInfo = ();

    /// Called once for each child. For most layouts the order is important.
    fn for_child<C: Layout>(
        &mut self,
        _tk: &mut dyn TkWindow,
        _storage: &mut Self::Storage,
        _child_info: Self::ChildInfo,
        _child: &mut C,
    ) {
    }

    /// Called at the end to output [`SizeRules`].
    ///
    /// Note that this does not include margins!
    fn finish<ColIter, RowIter>(
        self,
        _tk: &mut dyn TkWindow,
        _storage: &mut Self::Storage,
        _col_spans: ColIter,
        _row_spans: RowIter,
    ) -> SizeRules
    where
        ColIter: Iterator<Item = (usize, usize, usize)>,
        RowIter: Iterator<Item = (usize, usize, usize)>,
    {
        unimplemented!()
    }
}

/// Tool to solve for a `Rect` over child widgets
pub trait RulesSetter {
    /// Type of storage
    type Storage: Clone;

    /// Type required by [`RulesSolver::for_child`] (see implementation documentation)
    type ChildInfo;

    /// Called once for each child. For most layouts the order is important.
    fn child_rect(&mut self, child_info: Self::ChildInfo) -> Rect;
}

/// Dummy implementation
///
/// TODO: make real implementations to solve empty/single/derive cases?
impl RulesSetter for () {
    type Storage = ();
    type ChildInfo = ();

    fn child_rect(&mut self, _child_info: Self::ChildInfo) -> Rect {
        unimplemented!()
    }
}

/// Solve `widget` for `SizeRules` on both axes, horizontal first.
pub fn solve<L: Layout>(widget: &mut L, tk: &mut dyn TkWindow, size: Size) {
    // We call size_rules not because we want the result, but because our
    // spec requires that we do so before calling set_rect.
    let _w = widget.size_rules(tk, AxisInfo::new(false, None));
    let _h = widget.size_rules(tk, AxisInfo::new(true, Some(size.0)));
}
