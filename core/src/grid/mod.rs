// pub mod chunk;
pub mod vec_grid;

pub trait Grid {
    type Item;
    type Index;
    fn get(&self, index: Self::Index) -> Option<&Self::Item>;
    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Item>;

    fn iter(&self) -> impl Iterator<Item = (Self::Index, &Self::Item)>;
    fn iter_mut(&mut self) -> impl Iterator<Item = (Self::Index, &mut Self::Item)>;

    fn iter_around(&self, index: Self::Index) -> impl Iterator<Item = (Self::Index, &Self::Item)>;
    fn iter_around_mut(
        &mut self,
        index: Self::Index,
    ) -> impl Iterator<Item = (Self::Index, &mut Self::Item)>;

    fn iter_around_indexes(
        &self,
        index: Self::Index,
    ) -> impl Iterator<Item = Self::Index> + 'static;
}
