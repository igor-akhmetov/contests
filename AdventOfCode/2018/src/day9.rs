struct LinkedList<T> {
    data: Vec<T>,
    next: Vec<usize>,
    prev: Vec<usize>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { data: vec![], next: vec![], prev: vec![] }
    }

    pub fn insert(&mut self, item: T, after: usize) -> usize {
        if self.data.len() == 0 {
            self.data.push(item);
            self.next.push(0);
            self.prev.push(0);
            return 0;
        }
        let next = self.next[after];
        self.data.push(item);
        self.prev.push(after);
        self.next.push(next);
        let cur = self.data.len() - 1;
        self.next[after] = cur;
        self.prev[next] = cur;
        cur
    }

    pub fn next(&self, idx: usize) -> usize {
        self.next[idx]
    }

    pub fn prev(&self, idx: usize) -> usize {
        self.prev[idx]
    }

    pub fn remove(&mut self, idx : usize) -> (&T, usize) {
        let next = self.next[idx];
        let prev = self.prev[idx];
        self.next[prev] = next;
        self.prev[next] = prev;
        (&self.data[idx], next)
    }
}

fn solve(players : usize, marbles: usize) -> usize {
    let mut v = LinkedList::<usize>::new();
    let mut current = v.insert(0, 0);
    let mut player = 0;
    let mut scores = vec![0; players];
    for m in 1..marbles + 1 {
        if (m % 23) == 0 {
            scores[player] += m;
            for _ in 0..7 {
                current = v.prev(current);
            }
            let (removed_marble, new_current) = v.remove(current);
            scores[player] += removed_marble;
            current = new_current;
        } else {
            current = v.next(current);
            current = v.insert(m, current);
        }
        player = (player + 1) % players;
    }
    *scores.iter().max().unwrap()
}

pub fn day9_1() -> usize {
    solve(466, 71436)
}

pub fn day9_2() -> usize {
    solve(466, 7143600)
}