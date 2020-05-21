// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! ListView example
#![feature(proc_macro_hygiene)]

use kas::prelude::*;
use kas::widget::view::{ListView, ListViewMsg};
use kas::widget::Window;

#[layout(single)]
#[derive(Clone, Debug, Widget)]
struct DataModel {
    #[widget_core]
    core: CoreData,
    #[widget]
    view: ListView,
    data: Vec<&'static str>,
}

impl DataModel {
    fn refresh(&mut self) -> TkAction {
        let (ak, msg) = self.view.refresh();
        ak + self.view_request(msg)
    }

    fn view_request(&mut self, msg: ListViewMsg) -> TkAction {
        match msg {
            ListViewMsg::None => TkAction::None,
            ListViewMsg::DataRange => {
                let msg = self.view.data_range(self.data.len());
                self.view_request(msg)
            }
            ListViewMsg::DataRows(begin, end) => {
                let mut action = TkAction::None;
                for i in begin..end {
                    action += self.view.data_row(i, self.data[i]);
                }
                action
            }
        }
    }
}

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let mut model = DataModel {
        core: Default::default(),
        view: Default::default(),
        data: vec![
            // random lines from /usr/share/dict/words
            "calendry",
            "holdingly",
            "sulcal",
            "guatemala",
            "Featherstone",
            "ritzes",
            "megacolon",
            "untensely",
            "mongolia",
            "guillemot",
            "indin",
            "Sello",
            "reorganizing",
            "enrolling",
            "wickerby",
            "langourous",
            "nonvagrantly",
            "mesosome",
            "diebacks",
            "unsorting",
            "Shafiite",
            "slackening",
            "Nantyglo",
            "consolably",
            "longbow",
            "inwreathe",
            "smegmas",
            "acrosphacelus",
            "paranoidism",
            "sau",
        ],
    };
    let _ = model.refresh();

    let window = Window::new("List view", model);

    let theme = kas_theme::ShadedTheme::new();
    let mut toolkit = kas_wgpu::Toolkit::new(theme)?;
    toolkit.add(window)?;
    toolkit.run()
}
