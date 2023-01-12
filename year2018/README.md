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

## Day9

```
All/Hashmap - 1         time:   [8.2452 ms 8.7236 ms 9.6160 ms]
                        change: [-11.213% -4.0011% +5.5184%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
slope  [8.2452 ms 9.6160 ms] R^2            [0.7343627 0.6483170]
mean   [8.2823 ms 9.2870 ms] std. dev.      [95.783 µs 1.3789 ms]
median [8.2167 ms 8.8483 ms] med. abs. dev. [17.899 µs 764.07 µs]

All/VecDeque - 1        time:   [47.781 ms 51.399 ms 54.793 ms]
                        change: [+0.4669% +5.2521% +11.013%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
slope  [47.781 ms 54.793 ms] R^2            [0.8706168 0.8762067]
mean   [48.148 ms 52.521 ms] std. dev.      [814.91 µs 4.6449 ms]
median [47.629 ms 52.991 ms] med. abs. dev. [389.86 µs 5.1264 ms]

All/Vec - 1             time:   [73.698 ms 77.862 ms 80.740 ms]
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
slope  [73.698 ms 80.740 ms] R^2            [0.9171288 0.9367293]
mean   [72.841 ms 77.641 ms] std. dev.      [1.0988 ms 5.3422 ms]
median [72.132 ms 77.792 ms] med. abs. dev. [173.58 µs 5.1052 ms]

All/Hashmap - 5         time:   [44.396 ms 49.288 ms 58.951 ms]
                        change: [-8.7216% +2.8579% +17.489%] (p = 0.68 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
slope  [44.396 ms 58.951 ms] R^2            [0.3766501 0.2754822]
mean   [46.236 ms 55.792 ms] std. dev.      [3.1212 ms 11.739 ms]
median [44.401 ms 53.839 ms] med. abs. dev. [687.32 µs 11.837 ms]

All/VecDeque - 5        time:   [1.5885 s 1.6832 s 1.8280 s]
                        change: [-18.929% -0.4516% +16.208%] (p = 0.97 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
mean   [1.5885 s 1.8280 s] std. dev.      [48.239 ms 327.83 ms]
median [1.5529 s 1.6933 s] med. abs. dev. [16.459 ms 164.30 ms]

All/Vec - 5             time:   [2.5987 s 2.7066 s 2.8548 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
mean   [2.5987 s 2.8548 s] std. dev.      [70.646 ms 327.84 ms]
median [2.5714 s 2.7723 s] med. abs. dev. [2.6398 ms 248.64 ms]

All/Hashmap - 10        time:   [100.37 ms 109.44 ms 119.71 ms]
Found 2 outliers among 10 measurements (20.00%)  2 (20.00%) high severe
slope  [100.37 ms 119.71 ms] R^2            [0.7958484 0.7825261]
mean   [99.845 ms 120.31 ms] std. dev.      [2.8697 ms 25.928 ms]
median [98.531 ms 114.03 ms] med. abs. dev. [1.0920 ms 17.707 ms]

All/VecDeque - 10       time:   [8.0320 s 8.2762 s 8.5808 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
mean   [8.0320 s 8.5808 s] std. dev.      [173.38 ms 667.97 ms]
median [7.9243 s 8.4355 s] med. abs. dev. [68.145 ms 621.05 ms]
```
