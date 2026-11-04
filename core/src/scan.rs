use pyo3::prelude::*;

pub fn find_all(hay: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > hay.len() { return vec![]; }
    hay.windows(needle.len()).enumerate()
        .filter(|(_, w)| *w == needle).map(|(i, _)| i).collect()
}

/// Find all offsets of `needle` in `hay`.
#[pyfunction]
pub fn scan_bytes(hay: Vec<u8>, needle: Vec<u8>) -> Vec<usize> { find_all(&hay, &needle) }

#[cfg(test)]
mod tests {
    #[test]
    fn finds() { assert_eq!(super::find_all(b"abcabc", b"bc"), vec![1, 4]); }
}
