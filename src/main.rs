use zellij_tile::prelude::*;

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        let permissions = [
            PermissionType::ChangeApplicationState,
        ];
        request_permission(&permissions);
    }
    fn update(&mut self, _event: Event) -> bool {
        false
    }
    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let should_render = false;
        if pipe_message.name.as_str() == "sessionizer-new" {
            let name = pipe_message.args.get("name").map(|n| n.as_str());
            let cwd = pipe_message.args.get("cwd").map(PathBuf::from);
            if let Some(layout) = pipe_message.args.get("layout") {
                switch_session_with_layout(name, LayoutInfo::File(layout.to_owned()), cwd);
            } else {
                switch_session_with_cwd(name, cwd);
            }
        };
        should_render
    }
    fn render(&mut self, _rows: usize, _cols: usize) {
        hide_self();
    }
}
