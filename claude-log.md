# Claude Optimization Log

## Summary

**3 optimizations committed**:
- Day 14 Part 2 improved by 38.9%
- Day 17 Part 2 improved by 21.1%
- Day 3 Part 2 improved by 9.6%

---

## Day 1 - Parse Optimization

**Hypothesis**: Combining two `filter_map` calls into one should reduce iterator overhead.

**Change**: Merged two `filter_map` calls in `parse()` into a single one.

**Result**: ~1.3% improvement - below 1% threshold. **Not committed.**

---

## Day 22 - Array instead of HashMap

**Hypothesis**: Since the index function creates bounded values [0, 19^4), using a fixed-size array should be faster than HashMap.

**Change**: Replaced `AHashMap<i64, i64>` with `Vec<i64>` using -1 as sentinel.

**Result**: Stack overflow initially (array too large for stack in rayon threads). After switching to Vec, index out of bounds errors - the actual max index is larger than 19^4 = 130321. **Not committed.**

---

## Day 5 - Avoid Clone for Part1

**Hypothesis**: Part1 only needs to check if book is ordered, not get the sorted result. Avoid unnecessary clone+sort.

**Change**: Added separate `is_already_ordered` function with O(n²) comparison.

**Result**: +40% regression for Part 1. The O(n²) comparison is slower than cloning and sorting. **Not committed.**

---

## Day 11 - Reduce ilog10 Calls

**Hypothesis**: The `transform` function calls `n.ilog10()` and then `split` calls it again. Using a lookup table should be faster.

**Change**: Added `POWERS_OF_10` lookup table and inlined the split logic.

**Result**: +12% regression for Part 1, +9% for Part 2. The compiler already optimizes `10u64.pow()` efficiently. **Not committed.**

---

## Day 7 - Replace Loop in next_power_of_10

**Hypothesis**: Replace the while loop with `10u64.pow(n.ilog10() + 1)`.

**Change**: Replaced loop with single expression using ilog10.

**Result**: Significant regression (+50% Part 1, +35% Part 2). The simple loop is faster than ilog10+pow. **Not committed.**

---

## Day 14 - Single Pass Variance ✅

**Hypothesis**: Computing variance with Var(X) = E[X²] - E[X]² in a single pass should be faster than computing mean first then squared differences.

**Change**: Replaced two-pass variance calculation with single-pass using the algebraic identity.

**Result**:
- Part 1: -4.7% (minor improvement)
- Part 2: **-38.9%** (177µs → 108µs)

**Committed**: f29391e

---

## Day 17 - Bit Operations ✅

**Hypothesis**: Replace division by powers of 2 with right shifts and modulo 8 with bitwise AND.

**Change**: Changed `self.A /= 2u64.pow(...)` to `self.A >>= ...` and `% 8` to `& 7`.

**Result**:
- Part 1: No measurable change (already sub-microsecond)
- Part 2: **-21.1%** (4.9ms → 3.9ms)

**Committed**: b7c96c7

---

## Day 3 - Remove String Allocation ✅

**Hypothesis**: The `SpecialCharacters` instruction variant never uses its string content, so avoid allocating it.

**Change**: Changed `SpecialCharacters(String)` to `SpecialCharacters` (unit variant).

**Result**:
- Part 2: **-9.6%** (363µs → 331µs)

**Committed**: ad7511a

---

## Day 4 - Short-circuit Evaluation

**Hypothesis**: Using `&&` short-circuit evaluation instead of tuple pattern matching might be faster.

**Change**: Changed `if let (Some('M'), Some('A'), Some('S')) = (...)` to chained `&&` comparisons.

**Result**: +12% regression. The compiler optimizes tuple pattern matching better. **Not committed.**

---

## Observations

1. The Rust compiler and LLVM are very good at optimizing idiomatic code. Many "obvious" optimizations actually regress performance.

2. Operations involving `ilog10` and `pow` are not as fast as simple loops in some cases.

3. The biggest wins come from algorithmic improvements (single-pass variance) rather than micro-optimizations.

4. Pattern matching with tuples is well-optimized by the compiler - don't try to outsmart it.

5. Using arrays instead of HashMaps can be tricky with stack sizes and index bounds.

6. Bit operations (shifts and AND) are faster than arithmetic operations (division and modulo) for powers of 2.

7. Unnecessary allocations (like unused String fields) add measurable overhead.
