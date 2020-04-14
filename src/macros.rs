// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Library macros
//!
//! This documentation is provided as a reference. The macros may be easier to
//! understand from the example apps provided with `kas-wgpu`.
//!
//! This module provides two important macros:
//!
//! -   [`derive(Widget)`] implements the [`WidgetCore`] trait and optionally
//!     also [`Layout`], [`Widget`] and [`Handler`]
//! -   [`make_widget`] is a convenience macro to create a single instance of a
//!     custom widget type
//! -   [`derive(VoidMsg)`] is a convenience macro to implement
//!     `From<VoidMsg>` for the deriving type
//!
//! Note that these macros are defined in the external crate, `kas-macros`, only
//! because procedural macros must be defined in a special crate. The
//! `kas-macros` crate should not be used directly.
//!
//! Note further that these macros require gated functionality only available
//! in nightly `rustc` builds:
//! ```
//! #![feature(proc_macro_hygiene)]
//! ```
//!
//! [`make_widget`]: #the-make_widget-macro
//! [`derive(Widget)`]: #the-derivewidget-macro
//! [`derive(VoidMsg)`]: #the-derivevoidmsg-macro
//!
//!
//! ## The `derive(Widget)` macro
//!
//! The [`Widget`] trait requires the base traits [`WidgetCore`] and [`Layout`]
//! be implemented; additionally, widgets should implement [`Handler`].
//! This macro can generate implementations for all of these traits or only
//! for [`WidgetCore`] as required.
//!
//! For parent widgets, the [`make_widget`] macro is even more concise.
//!
//! ### Type attributes
//!
//! This `derive` attribute may only be used on structs. Example:
//!
//! ```
//! use kas::macros::Widget;
//! use kas::event::VoidMsg;
//! use kas::{CoreData, Layout, LayoutData, Widget};
//!
//! #[layout(single)]
//! #[handler(generics = <> where W: Widget<Msg = VoidMsg>)]
//! #[derive(Clone, Debug, Widget)]
//! struct WrapperWidget<W: Widget> {
//!     #[widget_core] core: CoreData,
//!     #[widget] child: W,
//! }
//! ```
//!
//! #### WidgetCore
//!
//! The [`WidgetCore`] trait is always implemented by this macro. The
//! `#[widget]` attribute may be used to parameterise this implementation,
//! for example:
//! ```
//! use kas::draw::{DrawHandle, SizeHandle};
//! use kas::layout::{AxisInfo, SizeRules};
//! use kas::macros::Widget;
//! use kas::{event, CoreData, Layout};
//!
//! #[widget(config(key_nav = true))]
//! #[handler]
//! #[derive(Clone, Debug, Default, Widget)]
//! struct MyWidget {
//!     #[widget_core] core: CoreData,
//! }
//!
//! impl Layout for MyWidget {
//!     fn size_rules(&mut self, size_handle: &mut dyn SizeHandle, axis: AxisInfo) -> SizeRules {
//!         todo!()
//!     }
//!     fn draw(&self, draw_handle: &mut dyn DrawHandle,
//!         mgr: &event::ManagerState, disabled: bool)
//!     {
//!         todo!()
//!     }
//! }
//! ```
//!
//! The `#[widget]` attribute supports the following syntax:
//!
//! -   `config = noauto`: opt out of deriving the [`WidgetConfig`] trait; use
//!     this to implement the trait manually (e.g. to use [`WidgetConfig::configure`])
//! -   `confg(noauto)`: same as above
//! -   `confg(PARAMS)`: derive [`WidgetConfig`] but with the following `PARAMS`
//!     specified:
//!
//!     -   `key_nav = false`: a boolean, describing whether the widget supports
//!         keyboard navigation (see [`WidgetConfig::key_nav`])
//!     -   `cursor_icon = kas::event::CursorIcon::Default`: the cursor icon to use
//!         when the mouse hovers over this widget (see [`WidgetConfig::cursor_icon`])
//!
//! #### Widget
//!
//! If the `#[widget]` attribute is present, the [`Widget`] trait is derived
//! (with default method implementations).
//!
//! #### Layout
//!
//! If the `#[layout(..)]` attribute is present, the [`Layout`] trait is
//! derived. Derivation is only supported for parent widgets.
//!
//! The following attribute parameters are expected:
//!
//! -   (first position): one of `single`, `grid`,
//!     `right`, `left`, `down`, `up`, `col`, `column`, `row`
//! -   (optional): `area=FIELD` where `FIELD` is a child widget; if specified,
//!     the area of self is considered to refer to child `FIELD`. This causes
//!     the [`kas::Layout::find_id`] function to directly return the child's Id.
//!
//! Child widgets are arranged as specified by the first parameter:
//!
//! -   `single` — the widget wraps a single child, with no border or margin
//! -   `col`, `column` or `down` — child widgets are arranged in a vertical
//!     column, top-to-bottom
//! -   `up` — reversed column
//! -   `row` or `right` — child widgets are arranged in a horizontal row,
//!     left-to-right
//! -   `left` — reversed row
//! -   `grid` — child widgets are arranged in a grid; position is specified
//!     via parameters to the `#[widget]` attribute on child fields
//!
//! Derivation of [`Layout`] for non-single layouts requires a data storage
//! field as follows; for the `single` layout this field is optional:
//! ```none
//! #[layout_data] layout_data: <Self as kas::LayoutData>::Data,
//! ```
//! This field is supports `Default` and `Clone`, thus may be constructed with
//! `layout_data: Default::default()`.
//! (Note: the [`LayoutData`] trait is also implemented by this macro.)
//!
//! #### Handler
//!
//! If one or more `#[handler]` attributes are present, then the [`Handler`]
//! trait is implemented (potentially multiple times with different
//! substitutions of generic parameters).
//! This attribute accepts the following comma separated arguments:
//!
//! -   (optional) `none`, `all`, `action`, or `send` — derive only [`Handler`]
//!     (`action`), [`SendEvent`] (`send`), neither, or both
//! -   (optional) `msg = TYPE` — the [`Handler::Msg`] associated type; if not
//!     specified, this type defaults to [`kas::event::VoidMsg`]
//! -   (optional) `substitutions = LIST` — a list subsitutions for type
//!     generics, for example: (T1 = MyType, T2 = some::other::Type`
//! -   (optional): `generics = ...`; this parameter must appear last in the
//!     list and allows extra type parameters and/or restrictions to appear on
//!     the implementations of [`Handler`], [`SendEvent`] and [`Widget`].
//!     It accepts any of the following:
//!
//!     -   `<TYPE_PARAMS>` where `TYPE_PARAMS` is the usual list of type
//!         parameters (e.g. `T, W: Widget`)
//!     -   `<TYPE_PARAMS> where CONDS` where `CONDS` are extra restrictions on
//!         type parameters (these restrictions may be on type parameters used
//!         in the struct signature as well as those in the `TYPE_PARAMS` list)
//!     -   `SUBS` where `SUBS` is a list of substitutions; e.g. if `M` is a
//!         type parameter of the struct, then `M => MyMsg` will substitute the
//!         parameter `M` for concrete type `MyMsg`
//!     -   `SUBS <TYPE_PARAMS> where CONDS`; e.g. if `M` is a type parameter
//!         of the struct, one might use `M => <W as Handler>::Msg, <W: Widget>`
//!
//! Commonly, implementations of the [`Handler`] and [`Layout`] traits require
//! extra type bounds on the
//! `impl` which do not appear on the struct, for example a struct may be
//! parametrised with `W: Widget`, but the [`Handler`] impl may require
//! `W: Layout`. This may be achieved as follows:
//! ```
//! # use kas::macros::Widget;
//! # use kas::{CoreData, Layout, Widget, event::Handler};
//! #[layout(single)]
//! #[handler(msg = <W as Handler>::Msg, generics = <> where W: Layout)]
//! #[derive(Clone, Debug, Default, Widget)]
//! pub struct Frame<W: Widget> {
//!     #[widget_core]
//!     core: CoreData,
//!     #[widget]
//!     child: W,
//! }
//! ```
//!
//! (Note that ideally we would use equality constraints in `where` predicates
//! instead of adding special parameter substitution support, but type equality
//! constraints are not supported by Rust yet: #20041.)
//!
//! ### Fields
//!
//! One struct field with specification `#[widget_core] core: CoreData` is required.
//! When deriving layouts a `#[layout_data]` field is also required (see above).
//!
//! Other fields may be child widgets or simply data fields. Those with a
//! `#[widget]` attribute are interpreted as child widgets, affecting the
//! implementation of derived [`WidgetCore`], [`Layout`] and [`Handler`]
//! methods.
//!
//! The `#[widget]` attribute accepts several parameters affecting both layout
//! and event-handling. All are optional.
//!
//! The first four affect positioning are only used by the `grid` layout:
//!
//! -   `col = ...` or `column = ...` — grid column, from left (defaults to 0)
//! -   `row = ...` — grid row, from top (defaults to 0)
//! -   `cspan = ...` — number of columns to span (defaults to 1)
//! -   `rspan = ...` — number of rows to span (defaults to 1)
//!
//! These two affect alignment in the case that a widget finds itself within a
//! cell larger than its ideal size. Application of alignment is determined by
//! the child widget's implementation of [`Layout::set_rect`], which may simply
//! ignore these alignment hints.
//!
//! -   `halign = ...` — one of `begin`, `centre`, `end`, `stretch`
//! -   `valign = ...` — one of `begin`, `centre`, `end`, `stretch`
//!
//! Finally, a parent widget may handle event-responses from a child widget
//! (see [`Handler`]). The parent widget should implement a utility method
//! with signautre `fn f(&mut self, mgr: &mut Manager, msg: M) -> R` where
//! `M` is the type [`Handler::Msg`] in the child widget's implementation,
//! then reference this method:
//!
//! -   `handler = f` — the name `f` of a utility method defined on this type
//!
//! If there is no `handler` parameter, the child widget's [`Handler::Msg`] type
//! should convert into the parent's [`Handler::Msg`] type via `From`.
//!
//!
//! ### Examples
//!
//! A simple example is included [above](#type-attributes).
//! The example below includes multiple children and custom event handling.
//!
//! ```
//! use kas::event::{Manager, VoidResponse, VoidMsg};
//! use kas::macros::Widget;
//! use kas::widget::Label;
//! use kas::{CoreData, Layout, LayoutData, Widget};
//!
//! #[derive(Debug)]
//! enum ChildMessage { A }
//!
//! #[layout(column)]
//! #[handler(generics = <> where W: Widget<Msg = ChildMessage>)]
//! #[derive(Debug, Widget)]
//! struct MyWidget<W: Widget> {
//!     #[widget_core] core: CoreData,
//!     #[layout_data] layout_data: <Self as LayoutData>::Data,
//!     #[widget] label: Label,
//!     #[widget(handler = handler)] child: W,
//! }
//!
//! impl<W: Widget> MyWidget<W> {
//!     fn handler(&mut self, mgr: &mut Manager, msg: ChildMessage) -> VoidResponse {
//!         match msg {
//!             ChildMessage::A => { println!("handling ChildMessage::A"); }
//!         }
//!         VoidResponse::None
//!     }
//! }
//! ```
//!
//!
//! ## The `make_widget` macro
//!
//! This macro allows easy creation of "layout" widgets (those whose purpose is
//! to house one or more child widgets) by introducing syntax for a struct
//! literal and adding the additional fields and implementations required by all
//! widgets.
//!
//! Syntax is similar to a Rust type definition, but with most of the types and
//! identifiers omitted. It's easiest to study an example:
//!
//! ```rust
//! # #![feature(proc_macro_hygiene)]
//! # use kas::event::{VoidResponse, VoidMsg, Manager};
//! # use kas::macros::make_widget;
//! # use kas::widget::Label;
//! # let inner_widgets = Label::new("");
//! #[derive(Clone, Copy, Debug)]
//! enum Item {
//!     Button,
//!     Check(bool),
//! }
//! let widget = make_widget! {
//!     #[layout(column)]
//!     #[handler(msg = VoidMsg)]
//!     struct {
//!         #[widget] _ = Label::new("Widget Gallery"),
//!         #[widget(handler = activations)] _ = inner_widgets,
//!         last_item: Item = Item::Button,
//!     }
//!     impl {
//!         fn activations(&mut self, mgr: &mut Manager, item: Item)
//!             -> VoidResponse
//!         {
//!             match item {
//!                 Item::Button => println!("Clicked!"),
//!                 Item::Check(b) => println!("Checkbox: {}", b),
//!             };
//!             self.last_item = item;
//!             VoidResponse::None
//!         }
//!     }
//! };
//! ```
//!
//! ### Struct and fields
//!
//! Starting from the middle, we have a `struct` definition, though two things
//! are unusual here: (1) the type is anonymous (unnamed), and (2) fields are
//! simultaneously given both type and value.
//!
//! Field specifications can get more unusual too, since both the field name and
//! the field type are optional. For example, all of the following are equivalent:
//!
//! ```nocompile
//! #[widget] l1: Label = Label::new("label 1"),
//! #[widget] _: Label = Label::new("label 2"),
//! #[widget] l3 = Label::new("label 3"),
//! #[widget] _ = Label::new("label 4"),
//! ```
//!
//! Omitting field names is fine, so long as you don't need to refer to them.
//! Omitting types, however, comes at a small cost: Rust does not support fields
//! of unspecified types, thus this must be emulated with generics. The macro
//! deals with the necessary type arguments to implementations, however macro
//! expansions (as sometimes seen in error messages) are ugly and, perhaps worst
//! of all, the field will have opaque type (making methods and inner fields
//! inaccessible). The latter can be partially remedied via trait bounds:
//!
//! ```nocompile
//! #[widget] display: impl HasText = EditBox::new("editable"),
//! ```
//!
//! Alternatively, generics can be introduced explicitly:
//!
//! ```nocompile
//! #[widget] display: for<W: Widget<Msg = VoidMsg>> Frame<W> =
//!     Frame::new(Label::new("example")),
//! ```
//!
//! ### Implementations
//!
//! Now, back to the example above, we see attributes and an `impl` block:
//!
//! ```nocompile
//! let widget = make_widget! {
//!     #[widget]
//!     #[layout(column)]
//!     #[handler(msg = VoidMsg)]
//!     struct {
//!         ...
//!     }
//!     impl {
//!         fn on_tick(&mut self, mgr: &mut Manager) {
//!             ...
//!         }
//!     }
//! };
//! ```
//!
//! Attributes may be applied to the anonymous struct like normal, with two
//! exceptions:
//!
//! 1.  `#[derive(Clone, Debug, kas::macros::Widget)]` is implied
//! 2.  `#[handler(msg = ..)]` is required and most only have an `msg` parameter
//!
//! `impl` blocks work like usual except that the struct name and type
//! parameters are omitted. Traits may also be implemented this way:
//!
//! ```nocompile
//! impl Trait { ... }
//! ```
//!
//! ### Example
//!
//! ```
//! #![feature(proc_macro_hygiene)]
//!
//! use kas::macros::{make_widget};
//! use kas::widget::TextButton;
//!
//! #[derive(Copy, Clone, Debug)]
//! enum OkCancel {
//!     Ok,
//!     Cancel,
//! }
//!
//! let button_box = make_widget!{
//!     #[layout(row)]
//!     #[handler(msg = OkCancel)]
//!     struct {
//!         #[widget] _ = TextButton::new("Ok", OkCancel::Ok),
//!         #[widget] _ = TextButton::new("Cancel", OkCancel::Cancel),
//!     }
//! };
//! ```
//!
//!
//! ## The `derive(VoidMsg)` macro
//!
//! This macro implements `From<VoidMsg>` for the given type (see [`VoidMsg`]).
//!
//! [`VoidMsg`]: crate::event::VoidMsg
//!
//! ### Example
//!
//! ```
//! use kas::macros::VoidMsg;
//!
//! #[derive(VoidMsg)]
//! enum MyMessage { A, B };
//! ```

// Imported for doc-links
#[allow(unused)]
use crate::{
    event::{Handler, SendEvent},
    CoreData, Layout, LayoutData, Widget, WidgetConfig, WidgetCore,
};

pub use kas_macros::{make_widget, VoidMsg, Widget};
