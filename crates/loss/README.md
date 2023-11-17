# loss 

LOSS is a set of lo(w)-level tracing [Subscriber]s with minimal
configuration and exported global constructors.

[Subscriber]: https://docs.rs/tracing-core/0.1.32/tracing_core/subscriber/trait.Subscriber.html

## Subscibers



## Example

Subscriber usage is most easily accessible through a set of
exported helper methods.

```rust
 use loss::dead;

dead::set_global_default().unwrap();
```
