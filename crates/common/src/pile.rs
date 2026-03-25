use rand::{rng, Rng};

pub struct Pile<T> {
    items: Vec<T>
}

impl<T> Pile<T> {
    pub fn pick(&mut self) -> T {
        let i = rng().random_range(0..self.items.len());
        self.items.swap_remove(i)
    }
}

impl<T: Clone> From<&[T]> for Pile<T> {
    fn from(value: &[T]) -> Self {
        Self {
            items: value.into(),
        }
    }
}

impl<T: Clone, const N: usize> From<&[T; N]> for Pile<T>  {
    fn from(value: &[T; N]) -> Self {
        value.as_slice().into()
    }
}