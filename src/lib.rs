use graphql_client;


pub struct WikiJs {
    url: String,
    key: String,
}


impl WikiJs {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            key,
        }
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
