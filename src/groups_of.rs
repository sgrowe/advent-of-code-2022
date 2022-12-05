pub fn groups_of<const N: usize, I>(iter: I) -> GroupsOf<N, I> {
    GroupsOf { iter }
}

pub struct GroupsOf<const N: usize, I> {
    iter: I,
}

impl<const N: usize, I, T> Iterator for GroupsOf<N, I>
where
    I: Iterator<Item = T>,
    T: Default,
    T: Copy,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut items = [Default::default(); N];

        for i in 0..N {
            items[i] = self.iter.next()?;
        }

        Some(items)
    }
}
