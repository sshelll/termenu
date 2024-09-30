use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::Menu;

impl<T> Menu<T> {
    pub(crate) fn fuzzy_match(&mut self) {
        // reset
        self.matched_item_indices = Vec::new();
        self.selection_idx = 0;
        self.scroll_offset = 0;

        // match
        let matcher = self.fuzzy_matcher.get_or_init(SkimMatcherV2::default);
        for (i, item) in self.item_list.iter_mut().enumerate() {
            let item = item.as_mut().unwrap();
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

        // reverse sort by score
        self.matched_item_indices.sort_by(|a, b| {
            let a = self.item_list[*a].as_ref().unwrap().score.unwrap();
            let b = self.item_list[*b].as_ref().unwrap().score.unwrap();
            b.cmp(&a)
        });
    }
}
