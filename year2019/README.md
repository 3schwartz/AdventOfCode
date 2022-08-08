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
  Job-JTHTNY : .NET 6.0.7 (6.0.722.32202), X64 RyuJIT

IterationCount=2  LaunchCount=2  WarmupCount=1  

```
|                 Method |      Mean |     Error |    StdDev |     Gen 0 |   Gen 1 | Allocated |
|----------------------- |----------:|----------:|----------:|----------:|--------:|----------:|
| DetectedAsteroidsAsync |  5.612 ms |  3.921 ms | 0.6068 ms | 2320.3125 | 23.4375 |      8 MB |
|  DetectedAsteroidsSync | 15.413 ms | 10.536 ms | 1.6305 ms | 1937.5000 | 15.6250 |      8 MB |

#### Go
````
cpu: Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
````

| Name | Runs | Mean | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| Trigonometric-8 | 54 | 23_788_604 ns/op (23.78 ms/op) |  11_312_955 B/op ~ 11 MB | 10_112 allocs/op |
| Slopes-8 | 24 |  43_277_388 ns/op (43.27 ms/op) | 26_828_667 B/op ~ 26 MB	| 21_284 allocs/op |