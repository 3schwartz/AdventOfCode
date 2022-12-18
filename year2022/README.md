# 2019 Benchmarks

* Nanosecond (ns): 10^-9
* Microsecond (μs): 10^-6

## Day 16

### Part 2

#### Go

**Test Data**
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
Benchmark_part2                                   make bench16 
Benchmark_part2/Value
Benchmark_part2/Value-8                      313           3_662_061 ns/op           15_764 B/op        124 allocs/op
Benchmark_part2/Cache
Benchmark_part2/Cache-8                      162           6_926_250 ns/op        2_207_092 B/op        454 allocs/op
Benchmark_part2/Default
Benchmark_part2/Default-8                    294           4_308_237 ns/op           15_772 B/op        124 allocs/op
Benchmark_part2/Array
Benchmark_part2/Array-8                     2595             445_007 ns/op           83_970 B/op      6_755 allocs/op
````
**Question Data**
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
Benchmark_part2
Benchmark_part2/Value
Benchmark_part2/Value-8                        1       21_262_852_600 ns/op            185_336 B/op          570 allocs/op
Benchmark_part2/Cache
Benchmark_part2/Cache-8                        1       29_530_301_600 ns/op      4_430_729_096 B/op      615_034 allocs/op
Benchmark_part2/Default
Benchmark_part2/Default-8                      1       52_353_376_400 ns/op            188_056 B/op          572 allocs/op
Benchmark_part2/Array
Benchmark_part2/Array-8                        1        9_536_394_700 ns/op      7_479_036_488 B/op  132_825_288 allocs/op
````