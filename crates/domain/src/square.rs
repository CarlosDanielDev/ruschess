#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub fn new(index: u8) -> Option<Square> {
        if index >= 64 {
            return None;
        }

        Some(Square(index))
    }

    pub fn index(self) -> u8 {
        self.0
    }

    // / 8 is for the rows like: (1, 2, 3, 4, 5, 6, 7, 8)
    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    // % 8 is for the columns like: (a(0), b(1), c(2), d(3), e(4), f(5), g(6), h(7))
    pub fn file(self) -> u8 {
        self.0 % 8
    }
}
