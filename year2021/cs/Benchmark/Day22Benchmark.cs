using BenchmarkDotNet.Attributes;
using Day22;

namespace Benchmark
{
    [MemoryDiagnoser]
    [MediumRunJob, SkewnessColumn, KurtosisColumn]
    public class Day22Benchmark
    {
        private string[]? lines;

        [GlobalSetup]
        public async Task Setup()
        {
            lines = await File.ReadAllLinesAsync("../../../../../../../../../data/day22_data.txt");
        }

        [Benchmark(Baseline =true)]
        public void LightSwitcher()
        {
            _ = new LightSwitcher(lines!).GetOnLights(true);
        }

        [Benchmark]
        public void LightIntervalSwitcher()
        {
            _ = new LightIntervalSwitcher(lines!).GetOnLights(true);
        }

        [Benchmark]
        public void LightIntervalSwitcherLessIf()
        {
            _ = new LightIntervalSwitcherLessIf(lines!).GetOnLights(true);
        }
    }
}
