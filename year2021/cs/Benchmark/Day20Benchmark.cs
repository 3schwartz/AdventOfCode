using BenchmarkDotNet.Attributes;
using Day20;

namespace Benchmark
{
    [MemoryDiagnoser]
    internal class Day20Benchmark
    {
        [Params(1,10,100,1000)]
        public int Power { get; set; }

        [Benchmark]
        public void MathPowerBox()
        {
            var _ = (int)Math.Pow(2, Power);
        }

        [Benchmark]
        public void IntLoop()
        {
            var _ = Image.Power(2, Power);
        }
    }
}
