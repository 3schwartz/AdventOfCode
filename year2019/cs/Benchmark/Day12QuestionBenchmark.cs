using BenchmarkDotNet.Attributes;
using Day12.Tests;

namespace Benchmark
{
    [MemoryDiagnoser]
    //[SimpleJob(launchCount: 2, warmupCount: 1, targetCount: 2)]
    public class QuestionBenchmark
    {
        private IList<Person>? input;

        [Params(10, 100, 1_000, 10_000)]
        public int Size { get; set; }

        [GlobalSetup]
        public void Setup()
        {
            input = new List<Person>(Size);
            for (int i = 0; i < Size; i++)
            {
                input.Add(new Person(Age: Random.Shared.Next(10,100)));
            }
        }

        [Benchmark]
        public void PLinq()
        {
            _ = Question.PLinq(input!);
        }

        [Benchmark]
        public void ParallelForEachConcurrentBag()
        {
            _ = Question.ParallelForEachConcurrentBag(input!);
        }

        [Benchmark]
        public void ParallelForEachLock()
        {
            _ = Question.ParallelForEachLock(input!);
        }

        [Benchmark]
        public void ParallelForEachLocalInitLock()
        {
            _ = Question.ParallelForEachLocalInitLock(input!);
        }

        [Benchmark]
        public void ParallelForEachImmutable()
        {
            _ = Question.ParallelForEachImmutable(input!);
        }

        [Benchmark]
        public void ParallelForEachLocalInitImmutable()
        {
            _ = Question.ParallelForEachLocalInitImmutable(input!);
        }

        [Benchmark]
        public async Task TaskAsync()
        {
            _ = await Question.TaskAsync(input!);
        }
    }
}
