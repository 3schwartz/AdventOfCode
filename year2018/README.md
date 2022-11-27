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
```