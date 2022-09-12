use std::{
    collections::HashMap,
    fs,
    io::BufRead,
    path::{Path, PathBuf},
};

use anyhow::Result;
use gitignored::Gitignore;
use serde::Serialize;

use crate::langs::{init_languages_hashmap, LangsMap};

pub struct GetStatsOptions {
    pub gitignore: bool,
}

// fn init_gitignored(is_gitignore: bool, path: Box<Path>) -> impl FnMut(Box<Path>) -> bool {
//     let mut ig = Gitignore::new(path.to_owned(), true, true);
//     let gitignore_path = path.join(".gitignore");
//     let globs1 = fs::read_to_string(gitignore_path)
//         .unwrap_or_default()
//         .clone();
//     let globs2: Vec<String> = globs1.lines().map(|s| s.to_string()).collect();

//     move |p: Box<Path>| {
//         let globs3 = globs2.clone();
//         let gg: Vec<&str> = globs3.into_iter().collect();
//         if is_gitignore {
//             !ig.ignores(gg.as_slice(), p)
//         } else {
//             true
//         }
//     }
// }

pub fn get_stats(path: &Path, options: &GetStatsOptions) -> Result<Stats> {
    let gitignore = options.gitignore;
    let mut ig = Gitignore::new(path, true, true);
    let gitignore_path = path.join(".gitignore");
    let globs = fs::read_to_string(gitignore_path).unwrap_or_default();
    let globs: Vec<&str> = globs.lines().collect();

    let mut include_file = |p: Box<&Path>| {
        if gitignore {
            !ig.ignores(&globs, p.as_ref())
        } else {
            true
        }
    };

    let langs_map = init_languages_hashmap();
    let mut stats = Stats::new();
    for p in get_file_paths(path, &mut include_file)? {
        if include_file(Box::new(p.as_path())) {
            let line_len = get_file_len(&p)?;
            let lang = get_file_lang(&p, &langs_map).unwrap_or("Other");
            let entry = stats.by_lang.entry(lang).or_insert(0);
            *entry += line_len;
            stats.total_loc += line_len;
        }
    }
    Ok(stats)
}

fn get_file_paths(
    path: &Path,
    include_dir: &mut impl FnMut(Box<&Path>) -> bool,
) -> Result<Vec<PathBuf>> {
    let mut result = vec![];
    if path.is_dir()
        && !path.to_str().unwrap_or_default().contains(".git")
        && include_dir(Box::new(path))
    {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                let mut paths = get_file_paths(p.as_path(), include_dir)?;
                result.append(&mut paths);
            } else {
                result.push(p.to_path_buf());
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

#[derive(Debug, PartialEq, Eq, Serialize)]
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
