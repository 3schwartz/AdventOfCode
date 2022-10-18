# 2019 Benchmarks

## Day 4

#### Go

````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Benchmark_mapIntegersFromAsciiBytes/Pointers-8 | 693726696 | 1.815 ns/op |  0 B/op | 0 allocs/op |
| Benchmark_mapIntegersFromAsciiBytes/Converter-8 | 1000000000 |  0.8019 ns/op | 0 B/op	| 0 allocs/op |

## Day 7
### Part 2

#### C#
``` ini

BenchmarkDotNet=v0.13.1
Intel Core i7-8650U CPU 1.90GHz (Kaby Lake R), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.300

IterationCount=2  LaunchCount=2  WarmupCount=1  

```
|        Method |      Mean |     Error |    StdDev |    Gen 0 |  Gen 1 | Allocated |
|-------------- |----------:|----------:|----------:|---------:|-------:|----------:|
|      Channels | 15.149 ms | 3.3503 ms | 0.5185 ms | 812.5000 |      - |      3 MB |
| SortOfSubject |  1.863 ms | 0.5272 ms | 0.0816 ms | 394.5313 | 3.9063 |      2 MB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| ChannelCoder | 187 | 6.390699 ms/op |  3_020_306 B/op | 5_520 allocs/op |
| ReactiveCoder | 2_378 |  0.933929 ms/op | 2_549_760 B/op	| 3_720 allocs/op |

## Day 10
### Part 1

#### C#
``` ini

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19043.1826 (21H1/May2021Update)
Intel Core i7-8565U CPU 1.80GHz (Whiskey Lake), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.302
  [Host]     : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT
  DefaultJob : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT


```
|                 Method |      Mean |     Error |    StdDev |     Gen 0 |   Gen 1 | Allocated |
|----------------------- |----------:|----------:|----------:|----------:|--------:|----------:|
| DetectedAsteroidsAsync |  4.469 ms | 0.0759 ms | 0.0710 ms | 2335.9375 | 15.6250 |      8 MB |
|  DetectedAsteroidsSync | 10.812 ms | 0.2835 ms | 0.7948 ms | 1937.5000 |       - |      8 MB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Trigonometric-8 | 54 | 23_788_604 ns/op (23.78 ms/op) |  11_312_955 B/op ~ 11 MB | 10_112 allocs/op |
| Slopes-8 | 24 |  43_277_388 ns/op (43.27 ms/op) | 26_828_667 B/op ~ 26 MB	| 21_284 allocs/op |

## Day 12
### Part 2

#### C#

Need to subtract SetupWithinEachBenchmark from the others. Can't use `[IterationSetup]` [since test below 100ms](https://benchmarkdotnet.org/articles/samples/IntroSetupCleanupIteration.html).

``` ini

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19043.1889 (21H1/May2021Update)
Intel Core i7-8565U CPU 1.80GHz (Whiskey Lake), 1 CPU, 8 logical and 4 physical cores
.NET SDK=6.0.302
  [Host]     : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT
  DefaultJob : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT


```
|                          Method |         Mean |       Error |      StdDev |    Ratio | RatioSD |     Gen 0 | Allocated |
|-------------------------------- |-------------:|------------:|------------:|---------:|--------:|----------:|----------:|
|        SetupWithinEachBenchmark |     5.390 μs |   0.1072 μs |   0.1468 μs |     1.00 |    0.00 |    1.6327 |      7 KB |
|         StepsToGetBackToInitial | 8,578.604 μs (8.579 ms) | 150.4749 μs | 234.2713 μs | 1,587.32 |   62.53 | 1875.0000 |  7,704 KB |
|    StepsToGetBackToInitialAsync | 5,789.290 μs (5.789 ms) |  57.7345 μs |  48.2109 μs | 1,071.77 |   28.91 | 1898.4375 |  7,705 KB |
| StepsToGetBackToInitialParallel | 6,209.266 μs (6.209 ms) | 124.1580 μs | 275.1256 μs | 1,138.94 |   49.67 | 1898.4375 |  7,707 KB |
|    StepsToGetBackToInitialPLinq | 6,773.728 μs (6.777 ms) | 133.1873 μs | 136.7736 μs | 1,249.72 |   48.94 | 1898.4375 |  7,709 KB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Setup-8 | 484393 | 2_339 ns/op (2.339 μs / op)) |  896 B/op | 23 allocs/op |
| Sync-8 | 3 |  450_950_267 ns/op (460.950 ms/op) | 149_761_752 B/op ~ 149 MB	| 6_760_006 allocs/op |
| Async-8 | 4 |  299_714_200 ns/op (299.714 ms/op) | 149_763_040 B/op ~ 149 MB	| 6_760_021 allocs/op |

## Day 14
### Part 2

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Ratio-8 | 6246 | 179_402 ns/op |  15_509 B/op | 15 allocs/op |
| Optimum-8 | 358 |  3_142_936 ns/op | 217_322 B/op	| 212 allocs/op |

## Day 18
### Part 1
#### Go
goos: windows
goarch: amd64
pkg: advent/cmd/day18
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
Benchmark_findPath/priority:_day18_test1-8                 74326             15810 ns/op            8197 B/op        138 allocs/op
Benchmark_findPath/graph:_day18_test1-8                   147038              7432 ns/op            2972 B/op         34 allocs/op
Benchmark_findPath/priority:_day18_test2-8                   880           1387014 ns/op          744620 B/op       4133 allocs/op
Benchmark_findPath/graph:_day18_test2-8                    21504             48975 ns/op           18771 B/op        154 allocs/op
Benchmark_findPath/priority:_day18_test3-8                   392           2602726 ns/op         1414987 B/op       7631 allocs/op
Benchmark_findPath/graph:_day18_test3-8                    25068             47732 ns/op           17724 B/op        151 allocs/op
Benchmark_findPath/priority:_day18_test5-8                    82          13601543 ns/op         7704622 B/op      42584 allocs/op
Benchmark_findPath/graph:_day18_test5-8                    22935             48925 ns/op           16042 B/op        162 allocs/op

## Day 20
### Part 1
#### Go

goos: windows
goarch: amd64
pkg: advent/cmd/day20
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
Benchmark_findPath/Queue:__test1-8                805958              1531 ns/op             479 B/op          8 allocs/op
Benchmark_findPath/PriorityQueue:__test1-8       1000000              1707 ns/op             412 B/op         14 allocs/op
Benchmark_findPath/Queue:__test2-8                 69220             16543 ns/op           13187 B/op         12 allocs/op
Benchmark_findPath/PriorityQueue:__test2-8         48312             24379 ns/op            5649 B/op        121 allocs/op
