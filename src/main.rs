use state::State;
use zellij_tile::prelude::*;

use std::collections::BTreeMap;

mod state;


register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.scrolloff = configuration
            .get("scrolloff")
            .map(|sc| sc.parse().unwrap_or(4))
            .unwrap_or(4);
        self.default_dirs = configuration
            .get("default_dirs")
            .map(|v| v.split(";").map(str::to_owned).collect())
            .unwrap_or(vec![".".to_string(), ".config/".to_string()]);
        self.default_layout = configuration.get("layout").map(String::to_owned);
        self.find_cmd = configuration
            .get("find_cmd")
            .map(|v| v.split(";").map(str::to_owned).collect())
            .unwrap_or(
                vec!["fd", "-d", "1", "-t", "dir", "."]
                    .into_iter()
                    .map(|v| v.to_owned())
                    .collect(),
            );
        let subscriptions = [EventType::RunCommandResult, EventType::Key];
        let permissions = [
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
        ];
        subscribe(&subscriptions);
        request_permission(&permissions);
    }
    fn update(&mut self, event: Event) -> bool {
        let mut should_render = true;
        match event {
            Event::Key(key) => match key.bare_key {
                BareKey::Backspace => {
                    if !self.search_term.is_empty() {
                        self.search_term.pop();
                        self.filter_dirs();
                        should_render = true;
                    }
                }
                BareKey::Char('n') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    if !self.filtered_dirs.is_empty()
                        && self.selected_idx < self.filtered_dirs.len() - 1
                    {
                        self.selected_idx += 1;
                        should_render = true;
                    }
                }
                BareKey::Char('p') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    if self.selected_idx > 0 {
                        self.selected_idx -= 1;
                        should_render = true;
                    }
                }
                BareKey::Char(ch) if key.has_no_modifiers() => {
                    self.search_term.push(ch);
                    self.filter_dirs();
                }
                BareKey::Enter => {
                    let cwd = self.filtered_dirs.get(self.selected_idx).unwrap();
                    hide_self();
                    self.create_session(cwd, None, None);
                }
                _ => (),
            },
            Event::RunCommandResult(exit_status, std_out, std_err, _) => {
                if exit_status == Some(0) {
                    self.parse_find_cmd(std_out);
                    should_render = true;
                } else {
                    eprintln!("SESSIONIZER: {}", String::from_utf8(std_err).unwrap());
                }
            }
            _ => (),
        }
        should_render
    }
    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let should_render = false;
        if pipe_message.name.as_str() == "sessionizer-new" {
            hide_self();
            self.create_or_select_session(pipe_message.args);
        };
        should_render
    }
    fn render(&mut self, rows: usize, _cols: usize) {
        if self.available_dirs.is_empty() {
            self.select_session();
        }
        let prompt = "FILTER: ";
        let text = Text::new(format!("{}{}", prompt, self.search_term))
            .color_range(2, 0..prompt.len())
            .color_range(3, prompt.len()..);
        print_text(text);
        let (min_row, max_row) = self.display_ranges(rows);
        for (i, dir) in self.filtered_dirs.iter().enumerate() {
            if i.ge(&min_row) && i.lt(&max_row) {
                println!();
                let this_dir = Text::new(dir);
                if self.selected_idx == i {
                    print_text(this_dir.selected());
                } else {
                    print_text(this_dir);
                }
            }
        }
    }
}
