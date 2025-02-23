# Off-by-one error in Rust vector iteration with element removal

This example demonstrates a common error when removing elements from a vector while iterating over it in Rust. The code attempts to remove even numbers, but it causes unexpected behavior due to incorrect index handling after removal.  The solution shows how to safely modify the vector in this situation.