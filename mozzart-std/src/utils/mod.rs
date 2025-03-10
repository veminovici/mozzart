use std::fmt;

/// Separator used for formatting lists in debug output
pub(crate) const LIST_SEPARATOR: &str = ", ";

/// A list with a title for debug formatting
pub(crate) struct NamedList<'a, T> {
    title: &'static str,
    items: &'a [T],
}

impl<'a, T: fmt::Debug> NamedList<'a, T> {
    /// Creates a new `NamedList` with a title and items
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::NamedList;
    ///
    /// let list = NamedList::new("Numbers", &[1, 2, 3]);
    /// assert_eq!(format!("{:?}", list), "Numbers[1, 2, 3]");
    /// ```
    pub fn new(title: &'static str, items: &'a [T]) -> Self {
        Self { title, items }
    }
}

impl<T: fmt::Debug> fmt::Debug for NamedList<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items = format_iterator(self.items.iter(), |item| format!("{item:?}"));
        write!(f, "{}{items}", self.title)
    }
}

/// Formats an iterator of items into a comma-separated string wrapped in square brackets
///
/// # Arguments
/// * `items` - An iterator over any type
/// * `f` - A function that converts each item to a String
///
/// # Examples
/// ```ignore
/// use mozzart_std::format_iterator;
/// let numbers = vec![1, 2, 3];
/// assert_eq!(
///     format_iterator(numbers.iter(), |n| format!("num({n})")),
///     "[num(1), num(2), num(3)]"
/// );
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(
///     format_iterator(empty.iter(), |n| n.to_string()),
///     "[]"
/// );
/// ```
pub(crate) fn format_iterator<T, F>(items: impl Iterator<Item = T>, f: F) -> String
where
    F: FnMut(T) -> String,
{
    let items = items.map(f).collect::<Vec<_>>().join(LIST_SEPARATOR);
    format!("[{items}]")
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
    fn test_format_iterator() {
        // Test with custom formatting function
        let numbers = vec![1, 2, 3];
        assert_eq!(
            format_iterator(numbers.iter(), |n| format!("num({n})")),
            "[num(1), num(2), num(3)]"
        );

        // Test with simple toString conversion
        let strings = vec!["a", "b", "c"];
        assert_eq!(
            format_iterator(strings.iter(), |s| s.to_string()),
            "[a, b, c]"
        );

        // Test with empty iterator
        let empty: Vec<i32> = vec![];
        assert_eq!(format_iterator(empty.iter(), |n| n.to_string()), "[]");

        // Test with single item
        let single = vec![42];
        assert_eq!(format_iterator(single.iter(), |n| n.to_string()), "[42]");

        // Test with custom type and complex formatting
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let points = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
        assert_eq!(
            format_iterator(points.iter(), |p| format!("({}, {})", p.x, p.y)),
            "[(1, 2), (3, 4)]"
        );
    }
}
