const N: usize = 128;
pub struct Cache {
    data: Vec<u32>
}
impl Cache {
    fn new() -> Cache {
        Cache { data: vec![0; N * N]}
    }
    fn idx(x: u32, y: u32) -> usize {
        x as usize * N + y as usize
    }
    fn get(&self, (x, y): (u32, u32)) -> Option<u32> {
        match self.data[Cache::idx(x, y)] {
            0 => None,
            n => Some(n)
        }
    }
    fn insert(&mut self, (x, y): (u32, u32), val: u32) {
        self.data[Cache::idx(x, y)] = val
    }
}

// A made-up dynamic programming problem (unimportant).
// This first implementation is a classic recursive solution with memoization.
// While it could be made more efficient by being smarter at transversing
// dependencies, I'm trying to avoid that, since I'm trying to find an
// optimization pattern when that isn't possible.
pub fn foo1(x: u32, y: u32) -> u32 {
    foo1_helper(x, y, &mut Cache::new())
}
fn foo1_helper(x: u32, y: u32, cache: &mut Cache) -> u32 {
    if x == 0 || y == 0 {
        // base case
        1
    } else if let Some(res) = cache.get((x, y)) {
        // check the cache
        res
    } else {
        // make some recursive calls, % 1000 to avoid overflow
        let tr = (foo1_helper(x - 1, y - 1, cache)
            + foo1_helper(x, y - 1, cache)
            + foo1_helper(x - 1, y, cache))
            % 1000;
        // save our result and return
        cache.insert((x, y), tr);
        tr
    }
}

// This second implementation is "optimizing" by manually managing the stack.
// This avoids recursion by storing the stack on the heap in the form of a
// vector, and explicitly stating what to store between recursive calls
pub fn foo2(x: u32, y: u32) -> u32 {
    // store x, y, and any previously computed recursive result
    enum StackState {
        FirstRec(u32, u32),
        SecondRec(u32, u32, u32),
        ThirdRec(u32, u32, u32, u32),
    }
    let mut stack = Vec::with_capacity((x + y) as usize);
    // this return value is used by the child to communicate the result back up
    let mut rv = 0;
    // same cache as before
    let mut cache = Cache::new();

    #[inline(always)]
    fn initial(mut x: u32, mut y: u32, cache: &mut Cache, rv: &mut u32, stack: &mut Vec<StackState>) {
        loop {
            if x == 0 || y == 0 {
                *rv = 1;
                break;
            } else if let Some(res) = cache.get((x, y)) {
                *rv = res;
                break;
            } else {
                // add our next step, and spawn a child
                stack.push(StackState::FirstRec(x, y));
                x -= 1;
                y -= 1;
            }
        }
    }

    initial(x, y, &mut cache, &mut rv, &mut stack);

    // grab the top of the stack til nothing left
    while let Some(state) = stack.pop() {
        match state {
            StackState::FirstRec(x, y) => {
                // save our return value, move to next step, spawn child
                stack.push(StackState::SecondRec(x, y, rv));
                initial(x, y - 1, &mut cache, &mut rv, &mut stack);
            }
            StackState::SecondRec(x, y, res1) => {
                // save our return value, move to next step, spawn child
                stack.push(StackState::ThirdRec(x, y, res1, rv));
                initial(x - 1, y, &mut cache, &mut rv, &mut stack);
            }
            StackState::ThirdRec(x, y, res1, res2) => {
                // all subresults are finished - store result in cache and rv
                let tr = (res1 + res2 + rv) % 1000;
                cache.insert((x, y), tr);
                rv = tr
            }
        }
    }
    // since final call has finished, return value is set to final value
    rv
}

// Doing this all auto-magically with futures to build the generator, and using
// the async_recursion crate to make it easier to handle the boxing.
pub fn foo3(x: u32, y: u32) -> u32 {
    futures::executor::block_on(foo3_helper(
        x,
        y,
        &mut Cache::new(),
    ))
}
#[async_recursion::async_recursion]
pub async fn foo3_helper(x: u32, y: u32, cache: &mut Cache) -> u32 {
    if x == 0 || y == 0 {
        1
    } else if let Some(res) = cache.get((x, y)) {
        res
    } else {
        let tr = (foo3_helper(x - 1, y - 1, cache).await
            + foo3_helper(x, y - 1, cache).await
            + foo3_helper(x - 1, y, cache).await)
            % 1000;
        cache.insert((x, y), tr);
        tr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        // hardcoded, known result
        let n = 100;
        let res = 41;
        // 100, 41
        // 5000, 609
        assert_eq!(super::foo1(n, n), res);
        assert_eq!(super::foo2(n, n), res);
        assert_eq!(super::foo3(n, n), res);
    }
}
