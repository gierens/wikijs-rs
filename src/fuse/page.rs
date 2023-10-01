use std::collections::HashMap;
use wikijs::{page::PageError, page::PageMinimal, Api};

pub(crate) struct PageCache {
    pages: HashMap<u64, PageMinimal>,
}

#[allow(dead_code)]
impl PageCache {
    fn get(&mut self, api: &Api, id: u64) -> Result<PageMinimal, PageError> {
        if let Some(page) = self.pages.get(&id) {
            let updated_at = api.page_get_updated_at(id as i64)?;
            if updated_at != page.updated_at {
                let page = api.page_get_minimal(id as i64)?;
                self.pages.insert(id, page.clone());
                Ok(page)
            } else {
                Ok(page.clone())
            }
        } else {
            let page = api.page_get_minimal(id as i64)?;
            self.pages.insert(id, page.clone());
            Ok(page)
        }
    }

    fn evict(&mut self, id: u64) {
        self.pages.remove(&id);
    }

    fn update(&mut self, api: &Api, id: u64) -> Result<PageMinimal, PageError> {
        self.pages.remove(&id);
        let page = api.page_get_minimal(id as i64)?;
        self.pages.insert(id, page.clone());
        Ok(page)
    }
}
