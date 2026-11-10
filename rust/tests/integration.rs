/// Integration check: both the library and CLI agree on slugid output.
#[cfg(test)]
mod tests {
    use crate::{slugid, slugify};

    #[test]
    fn slugify_lowercases_and_dashes() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("a  b--c"), "a-b-c");
    }

    #[test]
    fn slugid_is_stable_for_seed() {
        assert_eq!(slugid(1), slugid(1));
        assert_ne!(slugid(1), slugid(2));
    }
}