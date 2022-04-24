﻿using BenchmarkDotNet.Attributes;
using Day14;

namespace Benchmark
{
    [MemoryDiagnoser]
    [SimpleJob(launchCount: 1, warmupCount: 1, targetCount: 3)]
    public class Day14Benchmark
    {
        private string[] data;
        private PolymerInserter polymerInserter;
        private PolymerPair polymerPair;

        [Params(1,5,10,15)]
        public int Insertions { get; set; }

        [GlobalSetup]
        public void GlobalSetup()
        {
            data = DataLoader.GetData("../../../../../data/day14_data.txt");
        }

        [IterationSetup(Target = nameof(PolymerInserter))]
        public void SetupPolymerInserter()
        {
            polymerInserter = new PolymerInserter();
        }

        [Benchmark]
        public void PolymerInserter()
        {
            var polymerTemplate = polymerInserter.DoInsertion(data, Insertions);
            _ = polymerInserter.GetMostCommonMinusLeastCommon(polymerTemplate);
        }

        [IterationSetup(Target = nameof(PolymerPair))]
        public void SetupPolymerPair()
        {
            polymerPair = new PolymerPair();
        }

        [Benchmark]
        public void PolymerPair()
        {
            var polymerTemplateBig = polymerPair.UpdatePairs(data, Insertions);
            _ = polymerPair.MostMinusLeastFromPairs(polymerTemplateBig);
        }
    }
}
