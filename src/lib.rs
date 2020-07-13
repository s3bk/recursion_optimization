use std::collections::HashMap;

// A made-up dynamic programming problem (unimportant).
// This first implementation is a classic recursive solution with memoization.
// While it could be made more efficient by being smarter at transversing
// dependencies, I'm trying to avoid that, since I'm trying to find an
// optimization pattern when that isn't possible.
pub fn foo1(x: u32, y: u32) -> u32 {
    foo1_helper(x, y, &mut HashMap::new())
}
fn foo1_helper(x: u32, y: u32, cache: &mut HashMap<(u32, u32), u32>) -> u32 {
    if x == 0 || y == 0 {
        // base case
        1
    } else if let Some(&res) = cache.get(&(x, y)) {
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
// This avoid recursion by storing the stack on the heap in the form of a
// vector, and explicitly stating what to store between recursive calls
pub fn foo2(x: u32, y: u32) -> u32 {
    // store x, y, and any previously computed recursive result
    enum StackState {
        Initial(u32, u32),
        FirstRec(u32, u32),
        SecondRec(u32, u32, u32),
        ThirdRec(u32, u32, u32, u32),
    }
    let mut stack = vec![StackState::Initial(x, y)];
    // this return value is used by the child to communicate the result back up
    let mut rv = 0;
    // same cache as before
    let mut cache = HashMap::new();
    // grab the top of the stack til nothing left
    while let Some(state) = stack.pop() {
        match state {
            StackState::Initial(x, y) => {
                // base case and checking cache - the return value is put in rv
                if x == 0 || y == 0 {
                    rv = 1
                } else if let Some(&res) = cache.get(&(x, y)) {
                    rv = res
                } else {
                    // add our next step, and spawn a child
                    stack.push(StackState::FirstRec(x, y));
                    stack.push(StackState::Initial(x - 1, y - 1));
                }
            }
            StackState::FirstRec(x, y) => {
                // save our return value, move to next step, spawn child
                stack.push(StackState::SecondRec(x, y, rv));
                stack.push(StackState::Initial(x, y - 1));
            }
            StackState::SecondRec(x, y, res1) => {
                // save our return value, move to next step, spawn child
                stack.push(StackState::ThirdRec(x, y, res1, rv));
                stack.push(StackState::Initial(x - 1, y));
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

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        let n = 100;
        // hardcoded, known result
        let res = 41;
        assert_eq!(super::foo1(n, n), res);
        assert_eq!(super::foo2(n, n), res);
    }
}
