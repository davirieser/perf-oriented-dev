
### B) Data Structure Selection

##### B1) Tracing

Performance Improvement from 5-25% depending on test case (only one case where performance regressed).
https://github.com/tokio-rs/tracing/pull/580
The only consideration here was that the original BTreeMap (Map based on a B-Tree) was only created once and then only iterated over.
Here a Vec (Array) is better since it removes the initialization overhead and improves iteration speed.
Also this code path seems to only deal with low values of N.

##### B2) rustc

The data structure change is distributed over an issue and 2 pull requests:
- https://github.com/rust-lang/rust/issues/55514
- https://github.com/rust-lang/rust/pull/56241
- https://github.com/rust-lang/rust/pull/58623

The data structure change is about the standard library hashmap in the rust language.
It starts with the issue that asks the question of whether the `hashbrown` implementation would be faster than the implementation that was used at the time.  
This then lead to some troubles because the new implementation could not be used as a drop-in replacement.
After that they compared the behaviour and API of the implementations and concluded that they were compatible except for an edge case where an item would be dropped if it's Hash implementation panicked during rehashing.
Then they ensured that the new implementation would also be compatible with another [pull request](https://github.com/rust-lang/rust/pull/54043) that was active at the time that would add an `entry`-API to the original implementation.
During performance testing they found out that the initial size of the hashmaps differed resulting in `hashbrown` having way worse performance on small examples because of more allocations.
They discussed the initial size question in a more general context about which applications would benefit from which option and how easy it would be for a user to change into the other option if required.
A lot of time was also spent with compiler problems, like in any rust project.

The most important discussions were about:
- Performance 
  - Especially interesting in respect to the compiler getting a faster hashmap.
    Quote: "rustc is basically a giant hashmap benchmark"
- API compatibility

