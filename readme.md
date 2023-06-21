# Traits

Here you have pets - which all implement the `MakeSound` trait.

But a reckless dev forgot to implement the trait for the pets and just
left a `todo!()` for you. 

He may have noticed his mistake, if he would just have
called 

```sh
cargo test
```

as this would have showed him the failed tests.

Can you correct his mistakes? He will be very thankful.

TL;DR: Make the tests pass ;) - And by make them pass we mean not deleting the asserts.