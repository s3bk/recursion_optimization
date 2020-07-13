# Recursion Optimization

If I have some recursive problem (e.g., common in dynamic programming), I believed that explictly managing the stack would be an optimization. However, that doesn't seem to be the case - when benchmarked, the recursive solution is faster, although it is limited by the recursion depth. Is there any better way I could be doing this?

I've tested with a few different prolems, including a much more complicated one (where I first noticed this). In general, pretty consistently see a slowdown; in this case, `cargo bench` shows a 10% difference. Ideally, there'd be some generator magic to avoid recursion and avoid needing to build the state machine by hand, and those generators could just be stored in a vec (as a stack), since they're all the same size. However, I was having lots of issues getting it to work.
