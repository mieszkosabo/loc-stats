use std::iter::FromIterator;

use colored::Colorize;

use crate::get_stats::Stats;

fn print_banner() {
    println!(
        r#"
    __                      __        __      
   / /___  _____      _____/ /_____ _/ /______
  / / __ \/ ___/_____/ ___/ __/ __ `/ __/ ___/
 / / /_/ / /__/_____(__  ) /_/ /_/ / /_(__  ) 
/_/\____/\___/     /____/\__/\__,_/\__/____/  
                                              
    "#
    );
}

impl Stats {
    pub fn pretty_output(&self) {
        print_banner();

        let total_loc = format!("{}", self.total_loc).bold().yellow();
        let number_of_files = format!("{}", self.number_of_files).bold().yellow();
        print!("{} {}\t", "Lines of code:".bold().bright_white(), total_loc);
        println!(
            "{} {}",
            "Number of files:".bold().bright_white(),
            number_of_files
        );
        println!();

        let longest_name_len = &self
            .by_lang
            .keys()
            .map(|name| name.len())
            .max()
            .unwrap_or_default();

        let longest_loc_len = &&self
            .by_lang
            .values()
            .map(|stats| format!("{}", stats.loc).len())
            .max()
            .unwrap_or_default();

        let mut langs_vec = Vec::from_iter(&self.by_lang);
        langs_vec.sort_by(|a, b| b.1.loc.cmp(&a.1.loc));

        for entry in langs_vec {
            let lang_name = entry.0;
            let stat = entry.1;
            println!(
                "{:width$}\t{:>loc_width$}\t{:>5}%",
                lang_name,
                stat.loc,
                stat.percent,
                width = longest_name_len,
                loc_width = longest_loc_len
            );
        }
        println!();
    }
}
