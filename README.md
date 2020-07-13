# Recursion Optimization

If I have some recursive problem (e.g., common in dynamic programming), I believed that explictly managing the stack would be an optimization. However, that doesn't seem to be the case - when benchmarked, the recursive solution is faster, although it is limited by the recursion depth. Is there any better way I could be doing this?