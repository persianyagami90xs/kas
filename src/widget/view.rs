// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! View widgets
//!
//! **Work in progress:** this module is under construction and not stable.
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

#![cfg(feature = "view")]

mod list;

pub use list::{ListView, ListViewMsg};
