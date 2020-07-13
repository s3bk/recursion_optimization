# Recursion Optimization

If I have some recursive problem (e.g., common in dynamic programming), I believed that manually managing the stack would be an optimization. However, that doesn't seem to be the case - when benchmarked, the trivial recursive solution is faster. Is there any better way I could be doing this?

I've tested with a few different problems, including a much more complicated one (where I first noticed this). In general, I pretty consistently see a slowdown; in this case, `cargo bench` shows a 10% difference. I have a real situation where the recursion depth is a barrier as well. Ideally, there would be some generator magic to avoid recursion and avoid needing to build the state machine by hand, and those generators could just be stored in a vec (as a stack), since they're all the same size. However, I was having lots of issues getting it to work. Just for fun, I also included an implementation using futures; while this is by far the prettiest, it wastes CPU time heap allocating each generator individually, rather than (for example) putting them all in the same vec.

In src/lib.rs:

- foo1 is the naive implementation, and also the fastest
- foo2 is the manually managed implementation, slightly slower
- foo3 is the implementation using futures - very pretty, but slowest