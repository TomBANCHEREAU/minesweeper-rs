pub struct GridChunk<const S: usize, T> {
    tile: [[T; S]; S],
}

impl<const S: usize, T> GridChunk<S, T> {
    pub fn get_width() -> usize {
        S
    }
    pub fn get_height() -> usize {
        S
    }
}
