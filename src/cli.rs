use crate::{cache::load_cache, service::run_service};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::{FuzzyMatcher, clangd::fuzzy_match};
use std::io::{self, Write};

pub async fn run_cli() {
    let entries = if let Some(cached) = load_cache() {
        cached
    } else {
        run_service().await;
        load_cache().unwrap_or_default()
    };

    let matcher = SkimMatcherV2::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        if io::stdin().read_line(&mut query).is_err() {
            break;
        }
        let query = query.trim();
        if query.is_empty() {
            break;
        }

        let mut results: Vec<_> = entries
            .iter()
            .filter_map(|entry| matcher.fuzzy_match(&entry.name, query).map(|s| (entry, s)))
            .collect();
        results.sort_by_key(|(_, s)| -s);

        for (i, (entry, _)) in results.iter().take(8).enumerate() {
            println!("{} : {}", i + 1, entry.name);
        }
    }
}
