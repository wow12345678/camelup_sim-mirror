use crate::color::Color;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CamelStack {
    // stack from bottom to top
    camels: [Option<Color>; 5],
    // number of camels
    size: usize,
}

impl CamelStack {
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// splits the stack at a given index, returns the split off part
    #[inline]
    #[track_caller]
    pub fn split_off(&mut self, index: usize) -> CamelStack {
        let mut result = CamelStack {
            camels: [const { None }; 5],
            size: 0,
        };

        #[cold]
        #[cfg_attr(not(panic = "immediate-abort"), inline(never))]
        #[track_caller]
        #[optimize(size)]
        fn assert_failed(at: usize, len: usize) -> ! {
            panic!("`at` split index (is {at}) should be <= len (is {len})");
        }

        if index > self.size {
            assert_failed(index, self.size);
        }

        let split_len = self.size - index;
        result.camels[0..split_len].swap_with_slice(&mut self.camels[index..self.size]);
        result.size = split_len;
        self.size = index;

        result
    }

    /// iterates over stack values
    /// omits possible trailing Nones
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Color> + '_ {
        self.camels[..self.size].iter().map(|col| col.unwrap())
    }

    pub fn clear(&mut self) {
        self.camels = [const { None }; 5];
        self.size = 0;
    }

    #[track_caller]
    pub fn push(&mut self, new_elem: Color) {
        if self.size >= 5 {
            panic!("CamelStack overflow: size (is {}) should be < 5", self.size);
        }
        self.camels[self.size] = Some(new_elem);
        self.size += 1;
    }

    pub fn replace<T: Into<CamelStack>>(&mut self, replacement: T) {
        let replacement: CamelStack = replacement.into();

        self.camels = replacement.camels;
        self.size = replacement.size();
    }

    #[track_caller]
    pub fn pop(&mut self) {
        if self.size == 0 {
            panic!("CamelStack underflow: cannot pop from empty stack");
        }
        self.camels[self.size - 1] = None;
        self.size -= 1;
    }

    pub fn append<T: Into<CamelStack>>(&mut self, appendage: T) {
        let mut appendage: CamelStack = appendage.into();

        #[cold]
        #[cfg_attr(not(panic = "immediate-abort"), inline(never))]
        #[track_caller]
        #[optimize(size)]
        fn assert_failed(len1: usize, len2: usize) -> ! {
            panic!("sum of `len1` (is {len1}) and `len2` (is {len2}) should be <= 5");
        }

        if self.size() + appendage.size() > 5 {
            assert_failed(self.size(), appendage.size());
        }

        let new_size = self.size() + appendage.size();
        let appendage_size = appendage.size();
        self.camels[self.size..new_size].swap_with_slice(&mut appendage.camels[0..appendage_size]);
        self.size = new_size;
    }

    pub fn prepend<T: Into<CamelStack>>(&mut self, prefix: T) {
        let mut prefix: CamelStack = prefix.into();

        #[cold]
        #[cfg_attr(not(panic = "immediate-abort"), inline(never))]
        #[track_caller]
        #[optimize(size)]
        fn assert_failed(len1: usize, len2: usize) -> ! {
            panic!("sum of `len1` (is {len1}) and `len2` (is {len2}) should be <= 5");
        }

        if self.size + prefix.size > 5 {
            assert_failed(self.size, prefix.size);
        }

        let prefix_size = prefix.size;
        // append prefix elements to the end
        self.camels[self.size..self.size + prefix_size]
            .swap_with_slice(&mut prefix.camels[0..prefix_size]);
        self.size += prefix_size;

        // rotate so the prefix elements move to the front
        self.camels[0..self.size].rotate_right(prefix_size);
    }
}

impl<const N: usize> From<[Color; N]> for CamelStack {
    fn from(value: [Color; N]) -> Self {
        const {
            assert!(N <= 5);
        }

        let mut value = value.map(Some);

        let mut camels = [const { None }; 5];
        camels[0..N].swap_with_slice(&mut value[0..N]);
        CamelStack { camels, size: N }
    }
}
