/// A utility module that provides a named view over a slice of items.
///
/// This module defines the `NamedSlice` struct which pairs a name with a slice reference,
/// enabling better debugging and display of collections in musical contexts.
use std::fmt;

/// Represents a named view over a slice of items.
///
/// `NamedSlice` pairs a descriptive name with a slice reference, which is useful for
/// representing named collections of musical elements such as scales, chords, or voice groups
/// while providing meaningful debug output.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the referenced slice
/// * `T` - The type of elements in the slice
///
/// # Examples
///
/// ```
/// use mozzart_std::NamedSlice;
///
/// let notes = vec![60, 62, 64, 65, 67, 69, 71, 72];
/// let scale = NamedSlice::new("C Major".to_string(), &notes);
/// println!("{:?}", scale); // Outputs: C Major:[60, 62, 64, 65, 67, 69, 71, 72]
/// ```
pub struct NamedSlice<'a, T> {
    /// The descriptive name of the slice
    pub name: String,
    /// The referenced slice of items
    pub items: &'a [T],
}

/// The separator used when formatting items in a `NamedSlice`
const SEPARATOR: &str = ", ";

impl<'a, T> NamedSlice<'a, T> {
    /// Creates a new `NamedSlice` with the specified name and items.
    ///
    /// # Arguments
    ///
    /// * `name` - A string that describes the slice
    /// * `items` - A reference to the slice of items
    ///
    /// # Returns
    ///
    /// A new `NamedSlice` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use mozzart_std::NamedSlice;
    ///
    /// let chord_notes = [60, 64, 67]; // C Major chord
    /// let named_chord = NamedSlice::new("C Major".to_string(), &chord_notes);
    /// ```
    pub fn new(name: String, items: &'a [T]) -> Self {
        Self { name, items }
    }

    /// Creates a new `NamedSlice` with an unnamed name and the specified items.
    ///
    /// # Arguments
    ///
    /// * `items` - A reference to the slice of items
    ///
    /// # Returns
    ///
    /// A new `NamedSlice` instance with an unnamed name
    ///
    /// # Examples
    ///
    /// ```
    /// use mozzart_std::NamedSlice;
    ///
    /// let chord_notes = [60, 64, 67]; // C Major chord
    /// let named_chord = NamedSlice::new_unnamed(&chord_notes);
    /// ```
    pub fn new_unnamed(items: &'a [T]) -> Self {
        Self {
            name: "".to_string(),
            items,
        }
    }
}

/// Formats a slice of items into a string representation.
///
/// This helper function converts each item in the slice to a string using the provided
/// function, then joins them with the defined separator and wraps them in square brackets.
///
/// # Arguments
///
/// * `items` - The slice of items to format
/// * `f` - A function that converts each item to a string
///
/// # Returns
///
/// A formatted string representation of the slice
///
/// # Examples
///
/// ```
/// // Internal usage example
/// // to_string(&[1, 2, 3], |n| n.to_string()) would return "[1, 2, 3]"
/// ```
#[inline]
fn to_string<T, F>(items: &[T], f: F) -> String
where
    F: Fn(&T) -> String,
{
    let items = items.iter().map(f).collect::<Vec<_>>().join(SEPARATOR);
    format!("[{items}]")
}

impl<T> fmt::Debug for NamedSlice<'_, T>
where
    T: fmt::Debug,
{
    /// Formats the `NamedSlice` for debugging output.
    ///
    /// If the slice contains more than one item, the output format is:
    /// `name:[item1, item2, ...]`
    ///
    /// If the slice contains zero or one item, the output format is:
    /// `[item]` or `[]`
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// A formatting result
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let items = to_string(self.items, |item| format!("{:?}", item));
        if self.items.len() > 1 {
            write!(f, "{}:{items}", self.name)
        } else {
            write!(f, "{items}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_slice_new() {
        // Test that constructor properly initializes the struct
        let name = "Test Slice".to_string();
        let items = vec![1, 2, 3];
        let named_slice = NamedSlice::new(name.clone(), &items);

        assert_eq!(named_slice.name, name);
        assert_eq!(named_slice.items, &items);
    }

    #[test]
    fn test_to_string_function() {
        // Test that to_string correctly formats an empty slice
        let empty: Vec<i32> = Vec::new();
        let result = to_string(&empty, |n| n.to_string());
        assert_eq!(result, "[]");

        // Test that to_string correctly formats a slice with one item
        let single = vec![42];
        let result = to_string(&single, |n| n.to_string());
        assert_eq!(result, "[42]");

        // Test that to_string correctly formats a slice with multiple items
        let multiple = vec![1, 2, 3];
        let result = to_string(&multiple, |n| n.to_string());
        assert_eq!(result, "[1, 2, 3]");

        // Test with a custom formatter function
        let numbers = vec![1, 2, 3];
        let result = to_string(&numbers, |n| format!("Number {}", n));
        assert_eq!(result, "[Number 1, Number 2, Number 3]");
    }

    #[test]
    fn test_debug_format_multiple_items() {
        // Test debug formatting with multiple items (should include name)
        let items = vec![1, 2, 3];
        let named_slice = NamedSlice::new("Numbers".to_string(), &items);
        let debug_str = format!("{:?}", named_slice);
        assert_eq!(debug_str, "Numbers:[1, 2, 3]");
    }

    #[test]
    fn test_debug_format_single_item() {
        // Test debug formatting with a single item (should not include name)
        let items = vec![42];
        let named_slice = NamedSlice::new("Answer".to_string(), &items);
        let debug_str = format!("{:?}", named_slice);
        assert_eq!(debug_str, "[42]");
    }

    #[test]
    fn test_debug_format_empty() {
        // Test debug formatting with an empty slice (should not include name)
        let items: Vec<i32> = Vec::new();
        let named_slice = NamedSlice::new("Empty".to_string(), &items);
        let debug_str = format!("{:?}", named_slice);
        assert_eq!(debug_str, "[]");
    }
}
