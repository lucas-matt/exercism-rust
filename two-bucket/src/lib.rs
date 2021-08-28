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
    bucket: Bucket,
    capacity: u8,
    volume: u8
}

impl BucketState {
    fn available(&self) -> u8 {
        self.capacity - self.volume
    }

    fn is_full(&self) -> bool {
        self.volume == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.volume == 0
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct State {
    one: BucketState,
    two: BucketState,
    goal: u8,
    init: Bucket
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

    fn result(&self) -> (Bucket, u8) {
        if self.one.volume == self.goal {
            (Bucket::One, self.two.volume)
        } else {
            (Bucket::Two, self.one.volume)
        }
    }

    fn allowed(&self) -> bool {
        let (primary, secondary) = match self.init {
            Bucket::One => (&self.one, &self.two),
            Bucket::Two => (&self.two, &self.one)
        };
        !(secondary.is_full() && primary.is_empty())
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum ActionType {
    Fill,
    Empty,
    Pour
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Action(ActionType, Bucket, State);

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
        one, two, goal, init: *start_bucket
    };
    match do_solve(state) {
        Some((state, moves)) => {
            let (bucket, other) = state.result();
            Some(BucketStats {
                goal_bucket: bucket,
                other_bucket: other,
                moves
            })
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

fn do_solve(init:State) -> Option<(State, u8)> {

    let mut tried:HashSet<Action> = HashSet::new();
    let mut actions:Vec<(Action, u8)> = next_actions(init, 1);

    while !actions.is_empty() {

        let (next_action, moves) = actions.remove(0);
        let Action(action, bucket, state) = next_action;

        // if we've hit this point before or state not allowed
        if tried.contains(&next_action) || !state.allowed() {
            continue;
        }
        tried.insert(next_action);

        if state.complete() {
            return Some((state, moves))
        }

        let state = match action {
            ActionType::Pour => pour(bucket, state),
            ActionType::Fill => fill(bucket, state),
            ActionType::Empty => empty(bucket, state),
        };

        actions.append(&mut next_actions(state, moves + 1));

    }

    None
}

fn next_actions(state:State, moves:u8) -> Vec<(Action, u8)> {
    let mut actions = Vec::new();
    for action in [ActionType::Fill, ActionType::Empty, ActionType::Pour] {
        for bucket in [Bucket::One, Bucket::Two] {
            actions.push((Action(action, bucket, state), moves))
        }
    }
    actions
}

fn pour(bucket: Bucket, mut state: State) -> State {
    let (primary, secondary) = state.mut_buckets(bucket);
    let mv = min(secondary.available(), primary.volume);
    primary.volume -= mv;
    secondary.volume += mv;
    state
}

fn empty(bucket: Bucket, mut state: State) -> State {
    let (primary, _) = state.mut_buckets(bucket);
    primary.volume = 0;
    state
}

fn fill(bucket: Bucket, mut state: State) -> State {
    let (primary, _) = state.mut_buckets(bucket);
    primary.volume = primary.capacity;
    state
}


