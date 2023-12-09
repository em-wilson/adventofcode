#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Pos {
    pub x: isize, 
    pub y: isize,
}

impl Pos {
    pub fn from(x:isize, y:isize) -> Pos {
        return Pos{x:x,y:y};
    }

    pub fn is_adjacent_to(&self, pos:Pos) -> bool {
        let compare = *self;
        return pos == compare
        || pos == Pos::from(compare.x-1, compare.y-1)
        || pos == Pos::from(compare.x, compare.y-1)
        || pos == Pos::from(compare.x+1, compare.y-1)
        || pos == Pos::from(compare.x-1, compare.y)
        || pos == Pos::from(compare.x+1, compare.y)
        || pos == Pos::from(compare.x-1, compare.y+1)
        || pos == Pos::from(compare.x, compare.y+1)
        || pos == Pos::from(compare.x+1, compare.y+1)
    }
}

#[cfg(test)]
mod tests {
    use super::Pos;

    #[test]
    fn test_position_is_adjacent() {
        assert_eq!(true, Pos::from(3,3).is_adjacent_to(Pos::from(2,2)));
        assert_eq!(false, Pos::from(3,3).is_adjacent_to(Pos::from(8,6)));
    }
}