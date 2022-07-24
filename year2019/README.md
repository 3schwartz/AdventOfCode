# 2019 Benchmarks
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

| Name | Runs | Average | Allocated | Allocations from heap |
|---------------------------------------	|-----------------:	|--------------------:	|---------------------:	|-------------------------------------------:	|
| ChannelCoder | 187 | 6.390699 ms/op |  3_020_306 B/op | 5_520 allocs/op |
| ReactiveCoder | 2_378 |  0.933929 ms/op | 2_549_760 B/op	| 3_720 allocs/op |