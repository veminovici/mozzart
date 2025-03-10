//! Mozzart standard library

mod interval;
mod interval_slice;
mod pitch;
mod pitch_slice;

pub use interval::*;
pub use interval_slice::*;
pub use pitch::*;
pub use pitch_slice::*;

pub(crate) struct NamedList<'a, T> {
    title: &'static str,
    items: &'a [T],
}

impl<'a, T: std::fmt::Debug> NamedList<'a, T> {
    pub fn new(title: &'static str, items: &'a [T]) -> Self {
        Self { title, items }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for NamedList<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = debug_list(self.items.iter());
        write!(f, "{}[{items}]", self.title)
    }
}

/// Separator used for formatting lists in debug output
const LIST_SEPARATOR: &str = ", ";

/// Formats a list of items in the format `[item1, item2, item3]`
pub(crate) fn debug_list<T: std::fmt::Debug>(items: impl Iterator<Item = T>) -> String {
    items
        .map(|item| format!("{item:?}"))
        .collect::<Vec<_>>()
        .join(LIST_SEPARATOR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namedlist_debug() {
        let nl = NamedList::new("Test", &[1, 2, 3]);
        assert_eq!(format!("{nl:?}"), "Test[1, 2, 3]");

        let nl: NamedList<i32> = NamedList::new("Empty", &[]);
        assert_eq!(format!("{nl:?}"), "Empty[]");

        let nl = NamedList::new("Single", &[100u8]);
        assert_eq!(format!("{nl:?}"), "Single[100]");
    }

    #[test]
    fn test_list_separator() {
        let items = ["a", "b", "c"].join(LIST_SEPARATOR);
        assert_eq!(items, "a, b, c");
    }

    #[test]
    fn test_debug_list() {
        // Test with numbers
        let numbers = vec![1, 2, 3];
        assert_eq!(debug_list(numbers.iter()), "1, 2, 3");

        // Test with strings
        let strings = vec!["a", "b", "c"];
        assert_eq!(debug_list(strings.iter()), "\"a\", \"b\", \"c\"");

        // Test with empty iterator
        let empty: Vec<i32> = vec![];
        assert_eq!(debug_list(empty.iter()), "");

        // Test with single item
        let single = vec![42];
        assert_eq!(debug_list(single.iter()), "42");

        // Test with custom struct
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Test(i32);

        let custom = vec![Test(1), Test(2)];
        assert_eq!(debug_list(custom.iter()), "Test(1), Test(2)");
    }
}
