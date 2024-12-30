use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

use zellij_tile::prelude::*;

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
pub(crate) struct State {
    pub(crate) available_dirs: Vec<String>,
    pub(crate) filtered_dirs: Vec<String>,
    pub(crate) selected_idx: usize,
    pub(crate) search_term: String,
    pub(crate) scrolloff: usize,
    pub(crate) default_dirs: Vec<String>,
    pub(crate) default_layout: Option<String>,
    pub(crate) find_cmd: Vec<String>,
}

impl State {
    pub(crate) fn filter_dirs(&mut self) {
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
    pub(crate) fn create_session(&self, cwd: &str, name: Option<&str>, layout: Option<&str>) {
        let cwd = PathBuf::from(cwd);
        let name = match name {
            Some(n) => n,
            None => cwd.file_name().unwrap().to_str().unwrap(),
        };
        match (layout.map(str::to_owned), self.default_layout.clone()) {
            (Some(l), Some(_)) | (Some(l), None) | (None, Some(l)) => switch_session_with_layout(
                Some(name),
                LayoutInfo::File(l.to_owned()),
                Some(cwd.clone()),
            ),

            (None, None) => switch_session_with_cwd(Some(name), Some(cwd.clone())),
        };
    }
    pub(crate) fn parse_find_cmd(&mut self, std_out: std::vec::Vec<u8>) {
        self.available_dirs = String::from_utf8(std_out)
            .unwrap()
            .split("\n")
            .map(|e| e.to_string())
            .collect();
        self.filtered_dirs = self.available_dirs.clone();
    }
    pub(crate) fn select_session(&self) {
        let mut cmd = self.find_cmd.clone();
        cmd.append(&mut self.default_dirs.clone());
        eprintln!("SESSIONIZER: cmd {:?}", cmd);
        run_command(
            &cmd.iter().map(String::as_str).collect::<Vec<_>>(),
            BTreeMap::new(),
        );
    }

    pub(crate) fn create_or_select_session(&self, context: BTreeMap<String, String>) {
        if let Some(cwd) = context.get("cwd") {
            self.create_session(
                cwd,
                context.get("name").map(String::as_str),
                context.get("layout").map(String::as_str),
            );
        } else {
            self.select_session();
        }
    }
    pub(crate) fn display_ranges(&self, rows: usize) -> (usize, usize) {
        let curr_len = self.filtered_dirs.len();
        if curr_len == 0 {
            (0, rows - 1)
        } else {
            let max_row = (self.selected_idx + self.scrolloff)
                .min(self.filtered_dirs.len())
                .max(rows - 1);
            let min_row = max_row + 1 - rows;
            (min_row, max_row)
        }
    }
}
