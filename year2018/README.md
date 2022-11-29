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
All/Simple - All        time:   [3.7360 s 4.0107 s 4.3430 s]
                        change: [-8.5298% +1.1140% +11.829%] (p = 0.84 > 0.05)
                        No change in performance detected.
mean   [3.7360 s 4.3430 s] std. dev.      [177.41 ms 687.27 ms]
median [3.6148 s 4.3179 s] med. abs. dev. [41.832 ms 816.61 ms]

All/Value - All         time:   [4.1732 s 4.4632 s 4.7286 s]
                        change: [-11.387% -1.3553% +8.8693%] (p = 0.81 > 0.05)
                        No change in performance detected.
mean   [4.1732 s 4.7286 s] std. dev.      [225.44 ms 557.32 ms]
median [3.9195 s 4.8863 s] med. abs. dev. [88.877 ms 745.28 ms]
```