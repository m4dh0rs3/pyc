pub(crate) trait PathExt {
    fn rotate_to_smallest(self) -> Self;
    fn invert(self) -> Self;
}

impl PathExt for Vec<usize> {
    fn rotate_to_smallest(self) -> Self {
        let (mut min, mut i) = (usize::MAX, 0);

        for (j, node) in self.iter().enumerate() {
            if node < &min {
                i = j;
                min = *node;
            }
        }

        return self[i..]
            .to_vec()
            .into_iter()
            .chain(self[..i].to_vec().into_iter())
            .collect();
    }

    fn invert(self) -> Self {
        let mut p = self;
        p.reverse();
        p.rotate_to_smallest()
    }
}
