# 2015 Benchmarks

* https://doc.rust-lang.org/cargo/commands/cargo-bench.html
* https://crates.io/crates/criterion
* https://bheisler.github.io/criterion.rs/book/getting_started.html
* https://bheisler.github.io/criterion.rs/book/faq.html
* https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html

## Day 7

### Rust

```
all/lines               time:   [8.8221 ms 9.0116 ms 9.2856 ms]
                        change: [-5.4005% +1.5484% +8.2856%] (p = 0.69 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
slope  [8.8221 ms 9.2856 ms] R^2            [0.9652900 0.9563799]
mean   [8.8028 ms 9.6195 ms] std. dev.      [223.69 µs 991.84 µs]
median [8.6604 ms 9.5905 ms] med. abs. dev. [101.67 µs 1.0036 ms]

all/vec                 time:   [8.3716 ms 8.6940 ms 9.0739 ms]
                        change: [-6.2690% -2.7104% +0.7085%] (p = 0.17 > 0.05)
                        No change in performance detected.
slope  [8.3716 ms 9.0739 ms] R^2            [0.9253502 0.9155596]
mean   [8.5384 ms 8.9726 ms] std. dev.      [187.91 µs 490.05 µs]
median [8.4832 ms 9.0511 ms] med. abs. dev. [25.338 µs 647.77 µs]

all/lines - only function
                        time:   [8.6589 ms 8.9711 ms 9.4564 ms]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
slope  [8.6589 ms 9.4564 ms] R^2            [0.8694524 0.8391102]
mean   [8.5751 ms 10.868 ms] std. dev.      [491.61 µs 3.0597 ms]
median [8.2353 ms 9.8477 ms] med. abs. dev. [149.94 µs 1.8184 ms]

all/vec - only function time:   [8.3064 ms 8.6570 ms 9.1045 ms]
slope  [8.3064 ms 9.1045 ms] R^2            [0.9039573 0.8881048]
mean   [8.4638 ms 9.3143 ms] std. dev.      [371.15 µs 946.61 µs]
median [8.2966 ms 9.3681 ms] med. abs. dev. [95.701 µs 1.1783 ms]
```
