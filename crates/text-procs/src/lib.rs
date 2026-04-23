#[must_use]
pub fn indent_of(text: &str) -> usize {
    text.lines()
        .filter(|it| !it.trim().is_empty())
        .map(|it| it.len() - it.trim_start().len())
        .min()
        .unwrap_or(0)
}

#[must_use]
pub fn trim_indent(mut text: &str) -> String {
    if text.starts_with('\n') {
        text = &text[1..];
    }
    let indent = indent_of(text);
    text.split_inclusive('\n')
        .map(|line| {
            if line.len() <= indent {
                line.trim_start_matches(' ')
            } else {
                &line[indent..]
            }
        })
        .collect()
}

/// Extract substring that contains the offset, left biased, where each character matches a predicate.
/// This will stop matching at whitespace characters.
pub fn match_in_offset(
    contents: &str,
    offset: usize,
    pred: impl Fn(u8) -> bool,
) -> Option<(u32, &str)> {
    if !contents.is_char_boundary(offset) {
        return None;
    }
    let bytes = contents.as_bytes();
    let len = contents.len();
    // Case 1: offset is inside a match
    if offset < len && pred(bytes[offset]) {
        let mut start = offset;
        while start > 0 && pred(bytes[start - 1]) {
            start -= 1;
        }
        let mut end = offset;
        while end + 1 < len && pred(bytes[end + 1]) {
            end += 1;
        }
        return Some((start as u32, &contents[start..=end]));
    }

    // Case 2: offset is not inside a match

    // Search left: find the last match before offset
    let mut left_match = None;
    let mut left_distance = usize::MAX;

    if offset > 0 {
        let mut i = offset - 1;
        // Skip non-matching chars to the left
        while i > len && !(pred(bytes[i])) && bytes[i].is_ascii_whitespace() {
            i -= 1;
        }
        if pred(bytes[i]) {
            // Found the end of a left match
            let end = i;
            // Find its start
            let mut start = end;
            while start > 0 && pred(bytes[start - 1]) {
                start -= 1;
            }
            left_match = Some((start, end));
            left_distance = offset - end;
        }
    }

    // Search right: find the first match after offset
    let mut right_match = None;
    let mut right_distance = usize::MAX;

    if offset < len {
        let mut i = offset;
        // Skip non-matching chars to the right
        while i < len && !(pred(bytes[i])) && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < len && pred(bytes[i]) {
            // Found the start of a right match
            let start = i;
            // Find its end
            let mut end = start;
            while end + 1 < len && pred(bytes[end + 1]) {
                end += 1;
            }
            right_match = Some((start, end));
            right_distance = start - offset;
        }
    }

    // Choose the closer match (left preferred on tie)
    match (left_match, right_match) {
        (Some((l_start, l_end)), Some((r_start, r_end))) => {
            let (start, end) = if left_distance <= right_distance {
                (l_start, l_end)
            } else {
                (r_start, r_end)
            };
            Some((start as u32, &contents[start..=end]))
        }
        (Some((start, end)), None) | (None, Some((start, end))) => {
            Some((start as u32, &contents[start..=end]))
        }
        (None, None) => None,
    }
}
