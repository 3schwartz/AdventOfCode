using BenchmarkDotNet.Attributes;
using Common;
using Day15;

namespace Benchmark
{
    [MemoryDiagnoser]
    [SimpleJob(launchCount: 1, warmupCount: 1, targetCount: 3)]
    public class Day15Benchmark
    {
        private string[] data;
        private DijkstraFinder dijkstraFinder;
        private PriorityQueueFinder priorityQueueFinder;

        [Params(1,3,5,7)]
        public int GridSize {get; set;}

        [GlobalSetup]
        public void GlobalSetup()
        {
            data = DataLoader.GetData("../../../../../../../../../data/day15_data.txt");
            dijkstraFinder = new DijkstraFinder();
            priorityQueueFinder = new PriorityQueueFinder();
        }

        [Benchmark]
        public void PriorityQueue()
        {
            var nodes = priorityQueueFinder.CreateNodes(data, GridSize);
            _ = priorityQueueFinder.FindShortest(nodes);
        }

        [Benchmark]
        public void Dijkstra()
        {
            var nodes = dijkstraFinder.CreateNodes(data, GridSize);
            _ = dijkstraFinder.FindShortest(nodes);
        }
    }
}
