# conscale

Umm... aiming to something like [`d3-scale`](https://github.com/d3/d3-scale).

## Usage

NYR = Not Yet Released

## Example

```rust
let map = map_fn((0, 100), (-127, 127));
assert_eq!(-127, map(0));
assert_eq!(0, map(50));
assert_eq!(127, map(100));
assert_eq!(127, map(123)); // clamp
```

## License

MIT

## Author

Yuki Kodama / [@kuy](https://twitter.com/kuy)
