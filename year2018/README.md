# 2018 Benchmarks

* https://doc.rust-lang.org/cargo/commands/cargo-bench.html
* https://crates.io/crates/criterion
* https://bheisler.github.io/criterion.rs/book/getting_started.html
* https://bheisler.github.io/criterion.rs/book/faq.html
* https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html


* Nanosecond (ns): 10^-9
* Microsecond (μs): 10^-6

## Day5

#### Rust

```
Simple                  time:   [1.6260 µs 1.6419 µs 1.6584 µs]
                        change: [-3.8302% -1.5297% +0.9886%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
slope  [1.6260 µs 1.6584 µs] R^2            [0.8360279 0.8353723]
mean   [1.6429 µs 1.6975 µs] std. dev.      [73.885 ns 200.17 ns]
median [1.6053 µs 1.6753 µs] med. abs. dev. [50.455 ns 100.27 ns]

Value                   time:   [1.4974 µs 1.5104 µs 1.5248 µs]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
slope  [1.4974 µs 1.5248 µs] R^2            [0.7941627 0.7923550]
mean   [1.5227 µs 1.5747 µs] std. dev.      [89.691 ns 173.46 ns]
median [1.5114 µs 1.5272 µs] med. abs. dev. [56.492 ns 91.484 ns]
```