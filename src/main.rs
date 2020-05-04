struct Collection<'a> {
    cc: Vec<&'a [u32]>,
}

impl<'a> Collection<'a> {
    pub fn new(c: Vec<&'a [u32]>) -> Self {
        Collection { cc: c }
    }

    pub fn len(&self) -> usize {
        self.cc.len()
    }
}

struct CollectionIntoIterator<'b> {
    collect: &'b Collection<'b>,
    no_arrays: usize,
    index: Vec<usize>,
    length: Vec<usize>,
    count: usize,
    max_count: usize,
}

impl<'b> IntoIterator for &'b Collection<'b> {
    type Item = Vec<u32>;
    type IntoIter = CollectionIntoIterator<'b>;

    fn into_iter(self) -> Self::IntoIter {
        CollectionIntoIterator {
            collect: self,
            no_arrays: self.len(),
            // Set indexes to 0
            index: self.cc.iter().map(|_| 0).collect(),
            // Create an array of the length of each slice
            length: self.cc.iter().map(|x| x.len()).collect(),
            count: 0,
            // max_count is the product of all the lengths of the collections
            max_count: self.cc.iter().map(|x| x.len()).product::<usize>(),
        }
    }
}

impl Iterator for CollectionIntoIterator<'_> {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Vec<u32>> {
        self.count += 1;
        // Check to see if we have got to the end of the iteration
        if self.count > self.max_count {
            return None;
        } else if self.count > 1 {
            // Increment each field starting with the first
            let mut i = 0;
            self.index[i] = (self.index[i] + 1) % self.length[i];

            //  When we get to the end of the first list need to iterate the second
            while self.index[i] % self.length[i] == 0 && i + 1 < self.no_arrays {
                i += 1;
                self.index[i] = (self.index[i] + 1) % self.length[i];
            }
        }

        // Return the results
        Some(
            self.collect
                .cc
                .iter()
                .enumerate()
                .map(|(i, v)| v[self.index[i]])
                .collect(),
        )
    }
}

// Create the cartesian product from the provided Vec of arrays
fn cartesian(c: Vec<&[u32]>) -> Vec<Vec<u32>> {
    let col = Collection::new(c);
    col.into_iter().collect()
}

fn main() {
    let ret = cartesian(vec![&[1, 2], &[3], &[4, 5, 6]]);
    dbg!(&ret);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        assert_eq!(cartesian(vec![&[1]]), vec![vec![1]]);
    }

    #[test]
    fn square() {
        assert_eq!(
            cartesian(vec![&[1, 2], &[3, 4]]),
            vec![vec![1, 3], vec![2, 3], vec![1, 4], vec![2, 4]]
        );
    }

    #[test]
    fn rectangle() {
        assert_eq!(
            cartesian(vec![&[1, 2], &[3], &[4, 5, 6]]),
            vec![
                vec![1, 3, 4],
                vec![2, 3, 4],
                vec![1, 3, 5],
                vec![2, 3, 5],
                vec![1, 3, 6],
                vec![2, 3, 6]
            ]
        );
    }
}
