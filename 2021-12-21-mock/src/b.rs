fn main() {
    println!("use mock! case");
}

trait SearchIndex {
    fn search_by_keyword(&self, keyword: &str) -> Vec<usize>;
}

fn search(index: &dyn SearchIndex, keyword: &str) -> Vec<usize> {
    index.search_by_keyword(keyword)
}

#[cfg(test)]
use mockall::{mock, predicate::*};
#[cfg(test)]
mock! {
    pub MySearchIndex {
    }
    impl SearchIndex for MySearchIndex {
        fn search_by_keyword(&self, keyword: &str) -> Vec<usize>;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seach() {
        let mut mock = MockMySearchIndex::new();
        mock.expect_search_by_keyword()
            .with(eq("test"))
            .times(1)
            .returning(|_keyword| vec![1, 2, 3]);
        assert_eq!(vec![1, 2, 3], search(&mock, "test"));
    }
}
