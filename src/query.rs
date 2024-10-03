use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::Menu;

impl<T: Send + Sync> Menu<T> {
    pub(crate) fn fuzzy_match(&mut self) {
        // reset
        self.selection_idx = 0;
        self.scroll_offset = 0;

        if self.item_list.len() > 10000 {
            self.fuzzy_match_rayon();
        } else {
            self.matched_item_indices = Vec::new();
            // match
            let matcher = self.fuzzy_matcher.get_or_init(SkimMatcherV2::default);
            for (i, item) in self.item_list.iter_mut().enumerate() {
                match matcher.fuzzy_indices(&item.alias, &self.query) {
                    None => continue,
                    Some((score, indices)) => {
                        if score <= 0 {
                            continue;
                        }
                        item.score = Some(score);
                        item.matched_indices = Some(indices);
                        self.matched_item_indices.push(i);
                    }
                }
            }
        }

        // reverse sort by score
        self.matched_item_indices.sort_by(|a, b| {
            let a = self.item_list[*a].score;
            let b = self.item_list[*b].score;
            b.cmp(&a)
        });
    }

    fn fuzzy_match_rayon(&mut self) {
        use rayon::prelude::*;
        use std::sync::Mutex;

        let rayon_pool = self.rayon_pool.get_or_init(|| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .build()
                .unwrap()
        });

        // reset
        let matched_item_indices = Mutex::new(Vec::new());
        self.selection_idx = 0;
        self.scroll_offset = 0;

        // match
        let matcher = self.fuzzy_matcher.get_or_init(SkimMatcherV2::default);
        let chunk_size = 50;
        rayon_pool.install(|| {
            self.item_list
                .par_chunks_mut(chunk_size)
                .enumerate()
                .for_each(|(i, chunk)| {
                    let base_idx = i * chunk_size;
                    chunk.iter_mut().enumerate().for_each(|(i, item)| {
                        match matcher.fuzzy_indices(&item.alias, &self.query) {
                            None => (),
                            Some((score, indices)) => {
                                if score <= 0 {
                                    return;
                                }
                                item.score = Some(score);
                                item.matched_indices = Some(indices);
                                matched_item_indices.lock().unwrap().push(i + base_idx);
                            }
                        }
                    });
                });
        });

        self.matched_item_indices = matched_item_indices.into_inner().unwrap();
    }
}
