#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 || capacity_1 == capacity_2 {
        return None;
    }

    use Bucket::*;
    use FillState::*;

    let impossible_state = if *start_bucket == One {
        AllState {
            one: BucketState {
                fill_state: Empty,
                capacity: capacity_1,
            },
            two: BucketState {
                fill_state: Full,
                capacity: capacity_2,
            },
        }
    } else {
        AllState {
            one: BucketState {
                fill_state: Full,
                capacity: capacity_1,
            },
            two: BucketState {
                fill_state: Empty,
                capacity: capacity_2,
            },
        }
    };

    let mut moves = 0;
    let mut state = AllState {
        one: BucketState {
            fill_state: Empty,
            capacity: capacity_1,
        },
        two: BucketState {
            fill_state: Empty,
            capacity: capacity_2,
        },
    };
    use Move::*;
    let mut last_move = Pouring(One, One);
    loop {
        let mv = match state.status() {
            (Empty, Empty) => {
                if *start_bucket == One {
                    Filling(One)
                } else {
                    Filling(Two)
                }
            }
            (Full, Empty) => {
                if goal == capacity_2 {
                    Filling(Two)
                } else {
                    Pouring(One, Two)
                }
            }
            (Empty, Full) => {
                if goal == capacity_1 {
                    Filling(One)
                } else {
                    Pouring(Two, One)
                }
            }
            (Some(_), Empty) => {
                if let Pouring(_, _) = last_move {
                    Filling(Two)
                } else {
                    Pouring(One, Two)
                }
            }
            (Empty, Some(_)) => {
                if let Pouring(_, _) = last_move {
                    Filling(One)
                } else {
                    Pouring(Two, One)
                }
            }
            (Some(_), Full) => {
                if let Pouring(_, _) = last_move {
                    Emptying(Two)
                } else {
                    Pouring(Two, One)
                }
            }
            (Full, Some(_)) => {
                if let Pouring(_, _) = last_move {
                    Emptying(One)
                } else {
                    Pouring(One, Two)
                }
            }
            _ => break,
        };

        state = apply_move(state, mv);
        last_move = mv;
        moves += 1;

        if state == impossible_state {
            break;
        }

        if state.one.count() as u8 == goal {
            return Option::<BucketStats>::Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: state.two.count() as u8,
            });
        } else if state.two.count() as u8 == goal {
            return Option::<BucketStats>::Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: state.one.count() as u8,
            });
        }
    }

    None
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Move {
    Filling(Bucket),
    Emptying(Bucket),
    Pouring(Bucket, Bucket),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FillState {
    Empty,
    Some(u8),
    Full,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct BucketState {
    capacity: u8,
    fill_state: FillState,
}

impl BucketState {
    fn count(self) -> i32 {
        use FillState::*;
        match self.fill_state {
            Empty => 0,
            Some(count) => count as i32,
            Full => self.capacity as i32,
        }
    }

    fn filled_with(self, diff: i32) -> BucketState {
        use FillState::*;
        let count = self.count() + diff;
        match count {
            count if count <= 0 => BucketState {
                fill_state: Empty,
                ..self
            },
            count if count >= self.capacity as i32 => BucketState {
                fill_state: Full,
                ..self
            },
            count => BucketState {
                fill_state: Some(count as u8),
                ..self
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct AllState {
    one: BucketState,
    two: BucketState,
}

impl AllState {
    fn status(self) -> (FillState, FillState) {
        (self.one.fill_state, self.two.fill_state)
    }
}

fn apply_move(state: AllState, mv: Move) -> AllState {
    use Bucket::*;
    use FillState::*;
    use Move::*;
    match mv {
        Filling(One) => AllState {
            one: BucketState {
                fill_state: Full,
                ..state.one
            },
            ..state
        },
        Filling(Two) => AllState {
            two: BucketState {
                fill_state: Full,
                ..state.two
            },
            ..state
        },
        Emptying(One) => AllState {
            one: BucketState {
                fill_state: Empty,
                ..state.one
            },
            ..state
        },
        Emptying(Two) => AllState {
            two: BucketState {
                fill_state: Empty,
                ..state.two
            },
            ..state
        },
        Pouring(One, Two) => {
            let one_count = state.one.count();
            let two_count = state.two.count();
            let total = one_count + two_count;
            if total <= state.two.capacity as i32 {
                AllState {
                    one: state.one.filled_with(-one_count),
                    two: state.two.filled_with(one_count),
                }
            } else {
                let diff = state.two.capacity as i32 - two_count;
                AllState {
                    one: state.one.filled_with(-diff),
                    two: state.two.filled_with(diff),
                }
            }
        }
        Pouring(Two, One) => {
            let one_count = state.one.count();
            let two_count = state.two.count();
            let total = one_count + two_count;
            if total <= state.one.capacity as i32 {
                AllState {
                    one: state.one.filled_with(two_count),
                    two: state.two.filled_with(-two_count),
                }
            } else {
                let diff = state.one.capacity as i32 - one_count;
                AllState {
                    one: state.one.filled_with(diff),
                    two: state.two.filled_with(-diff),
                }
            }
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_move_filling_one() {
        use Bucket::*;
        use FillState::*;
        use Move::*;
        let state = AllState {
            one: BucketState {
                fill_state: Empty,
                capacity: 3,
            },
            two: BucketState {
                fill_state: Empty,
                capacity: 5,
            },
        };
        assert_eq!(
            apply_move(state, Filling(One)),
            AllState {
                one: BucketState {
                    fill_state: Full,
                    capacity: 3
                },
                two: BucketState {
                    fill_state: Empty,
                    capacity: 5
                }
            }
        );
    }

    #[test]
    fn apply_move_pouring_one_two() {
        use Bucket::*;
        use FillState::*;
        use Move::*;
        let state = AllState {
            one: BucketState {
                fill_state: Full,
                capacity: 3,
            },
            two: BucketState {
                fill_state: Empty,
                capacity: 5,
            },
        };
        assert_eq!(
            apply_move(state, Pouring(One, Two)),
            AllState {
                one: BucketState {
                    fill_state: Empty,
                    capacity: 3
                },
                two: BucketState {
                    fill_state: Some(3),
                    capacity: 5
                }
            }
        );
    }
}
