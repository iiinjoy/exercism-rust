#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        use ChessPosition as P;
        match (&self.0, &other.0) {
            (P(r1, _), P(r2, _)) if r1 == r2 => true,
            (P(_, f1), P(_, f2)) if f1 == f2 => true,
            //two points are on the diagonal if |k| = 1 in `y = kx + b`, because tan(45°) = 1
            (P(r1, f1), P(r2, f2)) if ((r1 - r2) / (f1 - f2)).abs() == 1 => true,
            _ => false,
        }
    }
}
