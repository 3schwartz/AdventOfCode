using BenchmarkDotNet.Attributes;

namespace Benchmark
{
    [MemoryDiagnoser]
    [SimpleJob(launchCount: 1, warmupCount: 1, targetCount: 3)]
    public class Day23Benchmark
    {
        private IList<char> hallWay;
        private List<Stack<char>> rooms;

        [GlobalSetup]
        public void Setup()
        {
            hallWay = Enumerable.Repeat('.', 11).ToList();
            rooms = new List<Stack<char>> {
                CreateRoom('B', 'A'),
                CreateRoom('C', 'D'),
                CreateRoom('B', 'C'),
                CreateRoom('D', 'A')};
        }

        [Benchmark(Baseline = true)]
        public void BaselineEquals()
        {
            _ = hallWay!.Equals(hallWay) && rooms!.Equals(rooms);
        }

        [Benchmark]
        public void FlattenEquals()
        {
            Span<char> currentFlat = stackalloc char[19];
            GetFlattenState(hallWay, rooms, ref currentFlat);
            Span<char> otherFlatten = stackalloc char[19];
            GetFlattenState(hallWay, rooms, ref otherFlatten);

            _ = currentFlat.SequenceEqual(otherFlatten);
        }

        [Benchmark]
        public void BaselineGetHash()
        {
            _ = hallWay.GetHashCode() ^ rooms.GetHashCode();
        }

        [Benchmark]
        public void FlattenGetHash()
        {
            Span<char> flatten = stackalloc char[19];
            GetFlattenState(hallWay, rooms, ref flatten);
            var hash = 1;
            foreach (var flat in flatten)
            {
                hash ^= flat.GetHashCode();
            }
        }

        private static void GetFlattenState(
            IList<char> hallWay,
            IList<Stack<char>> Rooms,
            ref Span<char> flatten)
        {
            var idx = 0;
            foreach (var place in hallWay)
            {
                flatten[idx] = place;
                idx++;
            }

            foreach (var room in Rooms)
            {
                foreach (var member in room)
                {
                    flatten[idx] = member;
                    idx++;
                }
            }
        }

        private Stack<char> CreateRoom(char outer, char inner)
        {
            var stack = new Stack<char>();
            stack.Push(inner);
            stack.Push(outer);
            return stack;
        }
    }
}
