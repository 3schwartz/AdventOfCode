using BenchmarkDotNet.Attributes;
using Day10.Tests;

namespace Benchmark
{
    [MemoryDiagnoser]
    [SimpleJob(launchCount:2, warmupCount:1, targetCount:2)]
    public class Day10Benchmark
    {
        private MonitoringStation? monitoringStation;

        [GlobalSetup]
        public void Setup()
        {
            var data = File.ReadAllText("../../../../../../../../../data/day10_data.txt");
            var asteroidMap = new AsteroidMap(data);
            monitoringStation = new MonitoringStation(asteroidMap);
        }

        [Benchmark]
        public void DetectedAsteroidsAsync()
        {
            _ = monitoringStation.FindLocationWithMaxDetectedAsteroidsAsync();
        }

        [Benchmark]
        public void DetectedAsteroidsSync()
        {
            _ = monitoringStation.FindLocationWithMaxDetectedAsteroidsNew();
        }
    }
}
