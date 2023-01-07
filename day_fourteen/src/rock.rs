/*
Pulling out rock into another module is slightly overkill for this toy problem,
but I wanted to practice making tests in rust a little
*/

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Rock {
    pub points: Vec<(i32, i32)>,
}

#[inline(always)]
fn between_points(
    start: (i32, i32),
    end: (i32, i32),
    test: (i32, i32),
) -> bool {
    if (test.0 != start.0 && test.1 != start.1)
        || (test.0 != end.0 && test.1 != end.1)
    {
        // check whether we can end early by seeing if test doesn't match x or y
        false
    } else if (test.0 >= start.0 && test.1 == start.1)
        && (test.0 <= end.0 && test.1 == end.1)
    {
        // check whether test is right of start and left of end
        true
    } else if (test.0 <= start.0 && test.1 == start.1)
        && (test.0 >= end.0 && test.1 == end.1)
    {
        // check whether test is left of start and right of end
        true
    } else if (test.0 == start.0 && test.1 <= start.1)
        && (test.0 == end.0 && test.1 >= end.1)
    {
        // check whether test is below start and above end
        true
    } else if (test.0 == start.0 && test.1 >= start.1)
        && (test.0 == end.0 && test.1 <= end.1)
    {
        // check whether test is above start and below end
        true
    } else {
        false
    }
}

impl Rock {
    /// check if the point is in this rock
    pub fn check_rock(&self, point: (i32, i32)) -> bool {
        let points = &self.points;
        for (index, start_point) in points.into_iter().enumerate() {
            let end_point = match points.get(index + 1) {
                Some(value) => value,
                None => break,
            };

            if between_points(*start_point, *end_point, point) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_horizontal() {
        let rock = Rock { points: vec![(497, 0), (498, 0)] };
        assert!(rock.check_rock((497, 0)));
        assert!(rock.check_rock((498, 0)));
    }

    #[test]
    fn check_horizontal_neg() {
        let rock = Rock { points: vec![(497, 0), (498, 0)] };
        assert!(!rock.check_rock((496, 0)));
        assert!(!rock.check_rock((499, 0)));
    }

    #[test]
    fn check_vert() {
        let rock = Rock { points: vec![(498, 0), (498, 2)] };
        assert!(rock.check_rock((498, 0)));
        assert!(rock.check_rock((498, 1)));
        assert!(rock.check_rock((498, 2)));
    }

    #[test]
    fn check_vert_neg() {
        let rock = Rock { points: vec![(498, 0), (498, 2)] };
        assert!(!rock.check_rock((498, -1)));
        assert!(!rock.check_rock((498, 3)));
    }

    #[test]
    fn check_rock_even() {
        let rock = Rock { points: vec![(498, 0), (498, 2)] };
        assert!(rock.check_rock((498, 0)));
        assert!(rock.check_rock((498, 1)));
        assert!(rock.check_rock((498, 2)));
    }

    #[test]
    fn check_rock_odd() {
        let rock = Rock { points: vec![(498, 0), (498, 2), (498, 6)] };
        assert!(rock.check_rock((498, 0)));
        assert!(rock.check_rock((498, 1)));
        assert!(rock.check_rock((498, 2)));
        assert!(rock.check_rock((498, 3)));
        assert!(rock.check_rock((498, 4)));
        assert!(rock.check_rock((498, 5)));
        assert!(rock.check_rock((498, 6)));
    }

    #[test]
    fn check_bend() {
        let rock = Rock { points: vec![(498, 0), (498, 2), (496, 2)] };
        assert!(rock.check_rock((498, 0)));
        assert!(rock.check_rock((498, 1)));
        assert!(rock.check_rock((498, 2)));
        assert!(rock.check_rock((497, 2)));
        assert!(rock.check_rock((496, 2)));
    }

    #[test]
    fn check_bend_neg() {
        let rock = Rock { points: vec![(498, 0), (498, 2), (496, 2)] };
        assert!(!rock.check_rock((498, -1)));
        assert!(!rock.check_rock((498, 3)));
        assert!(!rock.check_rock((495, 2)));
        assert!(!rock.check_rock((499, 2)));
        assert!(!rock.check_rock((496, 0)));
        assert!(!rock.check_rock((497, 0)));
        assert!(!rock.check_rock((497, 1)));
    }
}
