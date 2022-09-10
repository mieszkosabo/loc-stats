use std::{
    collections::HashMap,
    fs,
    io::BufRead,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::langs::{init_languages_hashmap, LangsMap};

pub fn get_stats(path: &Path) -> Result<Stats> {
    let langs_map = init_languages_hashmap();
    let mut stats = Stats::new();
    for p in get_file_paths(path)? {
        let line_len = get_file_len(&p)?;
        if let Some(lang) = get_file_lang(&p, &langs_map) {
            let entry = stats.by_lang.entry(lang).or_insert(0);
            *entry += line_len;
        }
        stats.total_loc += line_len;
    }
    Ok(stats)
}

fn get_file_paths(path: &Path) -> Result<Vec<PathBuf>> {
    let mut result = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                let mut paths = get_file_paths(&p)?;
                result.append(&mut paths);
            } else {
                result.push(p);
            }
        }
    }
    Ok(result)
}

fn get_file_len(path: &Path) -> Result<usize> {
    Ok(fs::read(path)?.lines().count())
}

fn get_file_lang(path: &Path, langs_map: &LangsMap) -> Option<&'static str> {
    let ext = path.extension()?;
    Some(langs_map.get(ext.to_str().unwrap_or_default())?)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Stats {
    pub total_loc: usize,
    pub by_lang: HashMap<&'static str, usize>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_loc: 0,
            by_lang: HashMap::new(),
        }
    }
}
impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
