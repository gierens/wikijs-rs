use std::collections::HashMap;
use wikijs::{page::PageError, page::PageMinimal, Api};

pub(crate) struct PageCache {
    pages: HashMap<u64, PageMinimal>,
}

#[allow(unused)]
impl PageCache {
    pub(crate) fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    pub(crate) fn get(
        &mut self,
        api: &Api,
        id: u64,
    ) -> Result<PageMinimal, PageError> {
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

    pub(crate) fn evict(&mut self, id: u64) {
        self.pages.remove(&id);
    }

    pub(crate) fn refetch(
        &mut self,
        api: &Api,
        id: u64,
    ) -> Result<PageMinimal, PageError> {
        self.pages.remove(&id);
        let page = api.page_get_minimal(id as i64)?;
        self.pages.insert(id, page.clone());
        Ok(page)
    }

    pub(crate) fn update_content(
        &mut self,
        api: &Api,
        id: u64,
        content: String,
    ) -> Result<(), PageError> {
        api.page_update_content(id as i64, content)?;
        self.refetch(api, id)?;
        Ok(())
    }
}
