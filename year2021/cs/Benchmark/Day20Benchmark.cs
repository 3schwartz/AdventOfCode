﻿using BenchmarkDotNet.Attributes;
using Day20;

namespace Benchmark
{
    [MemoryDiagnoser]
    public class Day20BenchmarkPower
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

    [MemoryDiagnoser]
    public class Day20Benchmark
    {
        private string[] lines;

        [Params(1,3,5)]
        public int Times { get; set; }

        [GlobalSetup]
        public async Task GlobalSetup()
        {
            lines = await File.ReadAllLinesAsync("../../../../../../../../../data/day20_data_test.txt");
        }

        [Benchmark]
        public void Image()
        {
            var image = new Image(lines);
            image.Enhance(Times);
        }

        [Benchmark]
        public void ImageSet()
        {
            var imageSet = new ImageSet(lines);
            imageSet.Enhance(Times);
        }
    }

    
}
