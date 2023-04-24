pub fn is_page_crossed(from: u16, to: u16) -> bool {
    from >> 8 != to >> 8
}