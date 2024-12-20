use zellij_tile::prelude::*;

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
struct State { }

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // runs once on plugin load, provides the configuration with which this plugin was loaded
        // (if any)
        //
        // this is a good place to `subscribe` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.subscribe.html)
        // to `Event`s (https://docs.rs/zellij-tile/latest/zellij_tile/prelude/enum.Event.html)
        // and `request_permissions` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.request_permission.html)
        let subscriptions = [EventType::SessionUpdate, EventType::ListClients];
        let permissions = [
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
            PermissionType::ReadApplicationState,
        ];
        subscribe(&subscriptions);
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
