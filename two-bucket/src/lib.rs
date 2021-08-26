use std::collections::HashSet;
use std::cmp::min;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct BucketState {
    pub bucket: Bucket,
    pub capacity: u8,
    pub volume: u8
}

impl BucketState {
    fn available(&self) -> u8 {
        self.capacity - self.volume
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct State {
    pub one: BucketState,
    pub two: BucketState,
    pub goal: u8
}

impl State {
    fn complete(&self) -> bool {
        self.one.volume == self.goal || self.two.volume == self.goal
    }

    fn mut_buckets(&mut self, bucket:Bucket) -> (&mut BucketState, &mut BucketState) {
        match bucket {
            Bucket::One => (&mut self.one, &mut self.two),
            Bucket::Two => (&mut self.two, &mut self.one)
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let one = init(Bucket::One, capacity_1, start_bucket);
    let two = init(Bucket::Two, capacity_2, start_bucket);
    let state = State {
        one, two, goal
    };
    let mut solver = Solver {
        space: HashSet::new()
    };
    match solver.solve(state) {
        Some((State, moves)) => {
            None
        }
        _ => None
    }
}

fn init(bucket:Bucket, capacity:u8, starting: &Bucket) -> BucketState {
    let volume = match starting {
        x if x == &bucket => capacity,
        _ => 0
    };
    BucketState {
        bucket,
        capacity,
        volume
    }
}

enum Action {
    Fill,
    Empty,
    Pour
}

struct Solver {
    space: HashSet<State>
}

impl Solver {
    fn solve(&mut self, state: State) -> Option<(State, u8)> {
        println!("{:?} {:?}", state.one, state.two);

        // if we've hit this point before, back out
        if self.space.contains(&state) {
            return None;
        }
        self.space.insert(state);

        // base success case
        if state.complete() {
            return Some((state, 1))
        }

        let actions = [
            self.pour(Bucket::One, state),
            self.pour(Bucket::Two, state),
            self.empty(Bucket::One, state),
            self.empty(Bucket::Two, state),
            self.fill(Bucket::One, state),
            self.fill(Bucket::Two, state)
        ];

        for action in actions {
            if let Some((end_state, moves)) = action {
                // unwind on success
                return Some((end_state, moves + 1));
            }
        }

        None
    }

    fn pour(&mut self, bucket: Bucket, mut state: State) -> Option<(State, u8)> {
        let (primary, secondary) = state.mut_buckets(bucket);
        let mv = min(secondary.available(), primary.volume);
        primary.volume -= mv;
        secondary.volume += mv;
        self.solve(state)
    }

    fn empty(&mut self, bucket: Bucket, mut state: State) -> Option<(State, u8)> {
        let (primary, _) = state.mut_buckets(bucket);
        primary.volume = 0;
        self.solve(state)
    }

    fn fill(&mut self, bucket: Bucket, mut state: State) -> Option<(State, u8)> {
        let (primary, _) = state.mut_buckets(bucket);
        primary.volume = primary.capacity;
        self.solve(state)
    }
}


