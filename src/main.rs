use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Config, Matcher,
};
use zellij_tile::prelude::*;

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
struct State {
    available_dirs: Vec<String>,
    filtered_dirs: Vec<String>,
    selected_idx: usize,
    search_term: String,
    scrolloff: usize,
    default_dirs: Vec<String>,
    default_layout: Option<String>,
    home_dir: String,
    find_cmd: Vec<String>,
}

impl State {
    fn filter_dirs(&mut self) {
        let pattern = Pattern::new(
            &self.search_term,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        );
        let mut matcher = Matcher::new(Config::DEFAULT);
        let matches = pattern.match_list(self.available_dirs.clone(), &mut matcher);
        self.filtered_dirs = matches.iter().map(|(v, _)| v.to_owned()).collect();
        self.selected_idx = 0;
    }
    fn create_session(&self, cwd: &str, name: Option<&str>, layout: Option<&str>) {
        let cwd = PathBuf::from(cwd);
        let name = match name {
            Some(n) => n,
            None => cwd.file_name().unwrap().to_str().unwrap(),
        };
        match (layout.map(|v| v.to_owned()), self.default_layout.clone()) {
            (Some(l), Some(_)) | (Some(l), None) | (None, Some(l)) => switch_session_with_layout(
                Some(name),
                LayoutInfo::File(l.to_owned()),
                Some(cwd.clone()),
            ),

            (None, None) => switch_session_with_cwd(Some(name), Some(cwd.clone())),
        };
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        self.scrolloff = 4;
        self.default_dirs = vec!["~/".to_string(), "~/.config/".to_string()];
        self.default_layout = Some("simple".to_owned());
        self.find_cmd = vec!["fd", "-d", "1", "-t", "dir", "."]
            .into_iter()
            .map(|v| v.to_owned())
            .collect();
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
                    if self.selected_idx < self.filtered_dirs.len() {
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
                    self.create_session(cwd, None, None);
                    hide_self();
                }
                _ => (),
            },
            Event::RunCommandResult(exit_status, std_out, std_err, c) => {
                if exit_status == Some(0) {
                    let name = c.get("cmd");
                    if let Some(name) = name {
                        if name == "find" {
                            self.available_dirs = String::from_utf8(std_out)
                                .unwrap()
                                .split("\n")
                                .map(|e| e.to_string())
                                .collect();
                            self.filtered_dirs = self.available_dirs.clone();
                            should_render = true;
                            eprintln!("SESSIONIZER: found {:?}", self.available_dirs);
                        } else if name == "get_home" {
                            eprintln!("SESSIONIZER: getting home");
                            self.home_dir = String::from_utf8(std_out)
                                .unwrap()
                                .split("\n")
                                .map(|e| e.to_string())
                                .collect();
                            eprintln!("SESSIONIZER: got home {}", self.home_dir);
                            eprintln!("SESSIONIZER: finding stuff");
                            if let Some(cwd) = c.get("cwd") {
                                self.create_session(
                                    cwd,
                                    c.get("name").map(String::as_str),
                                    c.get("layout").map(String::as_str),
                                );
                            } else {
                                let mut cmd = self.find_cmd.clone();
                                cmd.append(
                                    &mut self
                                        .default_dirs
                                        .clone()
                                        .iter()
                                        .map(|d| d.replace("~", self.home_dir.as_str()))
                                        .collect(),
                                );
                                eprintln!("SESSIONIZER: cmd {:?}", cmd);
                                run_command(
                                    &cmd.iter().map(String::as_str).collect::<Vec<_>>(),
                                    BTreeMap::from([("cmd".to_owned(), "find".to_owned())]),
                                );
                            }
                        }
                    }
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
            let mut pwd_context = BTreeMap::from([("cmd".to_owned(), "get_home".to_owned())]);
            if let Some(name) = pipe_message.args.get("name") {
                pwd_context.insert("name".to_owned(), name.clone());
            }
            if let Some(layout) = pipe_message.args.get("layout") {
                pwd_context.insert("layout".to_owned(), layout.clone());
            }
            if let Some(cwd) = pipe_message.args.get("cwd") {
                pwd_context.insert("cwd".to_owned(), cwd.clone());
            }
            run_command(&["pwd"], pwd_context);
        };
        should_render
    }
    fn render(&mut self, rows: usize, _cols: usize) {
        if self.filtered_dirs.is_empty() {
            hide_self();
        } else {
            show_self(true);
            let prompt = "FILTER: ";
            let text = Text::new(format!("{}{}", prompt, self.search_term))
                .color_range(2, 0..prompt.len())
                .color_range(3, prompt.len()..);
            print_text(text);
            let max_row = (self.selected_idx + self.scrolloff)
                .min(self.filtered_dirs.len())
                .max(rows - 1);
            let min_row = max_row + 1 - rows;
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
}
