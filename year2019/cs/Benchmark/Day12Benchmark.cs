using BenchmarkDotNet.Attributes;
using Day12.Tests;

namespace Benchmark
{
    [MemoryDiagnoser]
    //[SimpleJob(launchCount: 2, warmupCount: 1, targetCount: 2)]
    public class Day12Benchmark
    {
        private string[]? lines;

        [GlobalSetup]
        public void Setup()
        {
            lines = File.ReadAllLines("../../../../../../../../../data/day12_test2_data.txt");
        }

        // Can't use [IterationSetup] since test below 100ms
        // https://benchmarkdotnet.org/articles/samples/IntroSetupCleanupIteration.html
        [Benchmark(Baseline = true)]
        public void SetupWithinEachBenchmark()
        {
            var moons = lines!.Select(line => Moon.CreateMoon(line).Moon).ToList();
            _ = new MoonSimulator(moons!);
        }

        [Benchmark]
        public void StepsToGetBackToInitial()
        {
            var moons = lines!.Select(line => Moon.CreateMoon(line).Moon).ToList();
            var simulator = new MoonSimulator(moons!);
            _ = simulator.StepsToGetBackToInitial();
        }

        [Benchmark]
        public async Task StepsToGetBackToInitialAsync()
        {
            var moons = lines!.Select(line => Moon.CreateMoon(line).Moon).ToList();
            var simulator = new MoonSimulator(moons!);
            _ = await simulator.StepsToGetBackToInitialAsync();
        }

        [Benchmark]
        public void StepsToGetBackToInitialParallel()
        {
            var moons = lines!.Select(line => Moon.CreateMoon(line).Moon).ToList();
            var simulator = new MoonSimulator(moons!);
            _ = simulator.StepsToGetBackToInitialParallel();
        }

        [Benchmark]
        public void StepsToGetBackToInitialPLinq()
        {
            var moons = lines!.Select(line => Moon.CreateMoon(line).Moon).ToList();
            var simulator = new MoonSimulator(moons!);
            _ = simulator.StepsToGetBackToInitialPLinq();
        }
    }
}
