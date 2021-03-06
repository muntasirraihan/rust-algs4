/// FIXME: UF::connected has a different mutablity from UnionFind trait.

use std::iter;
use std::fmt;

/// Quick union with path compression.
pub struct UnionFind {
    id: Vec<usize>,
    //  number of objects in the tree rooted at i.
    sz: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            id: (0..n).collect(),
            sz: iter::repeat(1).take(n).collect()
        }
    }

    fn root_of(&mut self, p: usize) -> usize {
        let mut rid = self.id[p];
        while rid != self.id[rid] {
            // Simpler one-pass variant
            self.id[rid] = self.id[self.id[rid]];
            rid = self.id[rid];
        }
        rid
    }

    // NOTE: method type is different from other implementations!!
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root_of(p) == self.root_of(q)
    }

    // Link root of smaller tree to root of larger tree.
    //Update the sz[] array.
    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root_of(p);
        let j = self.root_of(q);

        if i == j {
            return ;
        }
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }

    #[allow(unused_variables)]
    pub fn find(&self, p: usize) -> usize {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.id.iter() {
            try!(write!(f, "{} ", i));
        }
        Ok(())
    }
}
