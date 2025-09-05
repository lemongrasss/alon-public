# Quicksort Implementation in Rust

A simple and efficient implementation of the quicksort algorithm in Rust.

## Features

- Generic implementation that works with any type implementing `PartialOrd` and `Copy`
- In-place sorting (no additional memory allocation)
- Comprehensive test suite
- Documentation with examples

## Usage

### As a library

```rust
use quicksort::quicksort;

let mut arr = [3, 6, 8, 10, 1, 2, 1];
quicksort(&mut arr);
// arr is now [1, 1, 2, 3, 6, 8, 10]
```

### Running the example

```bash
cargo run
```

## Testing

Run the test suite:

```bash
cargo test
```

## Algorithm

This implementation uses the Lomuto partition scheme:

1. Choose the last element as the pivot
2. Partition the array so elements ≤ pivot are on the left, others on the right
3. Recursively sort the left and right subarrays

**Time Complexity:**
- Best case: O(n log n)
- Average case: O(n log n)
- Worst case: O(n²)

**Space Complexity:** O(log n) due to recursion stack