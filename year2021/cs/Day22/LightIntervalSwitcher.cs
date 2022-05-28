namespace Day22
{
    internal class LightIntervalSwitcher
    {
        private readonly IList<(int action, IList<(int From, int Until)> intervals)> instructions;

        public LightIntervalSwitcher(string[] lines)
        {
            instructions = new List<(int action, IList<(int From, int Until)>)>();
            foreach (var line in lines)
            {
                var instruction = CreateInterval(line);

                instructions.Add(instruction);
            }
        }

        private static (int action, IList<(int From, int Until)> intervals) CreateInterval(string line)
        {
            var intervals = new List<(int From, int Until)>();
            var splitted = line.Split();
            var action = splitted[0] == "on" ? 1 : 0;
            foreach (var instruction in splitted[1].Split(','))
            {
                var coordinates = instruction[2..].Split("..");
                intervals.Add((int.Parse(coordinates[0]), int.Parse(coordinates[1]) + 1));
            }
            return (action, intervals);
        }

        private static bool IsInstructionValid(IList<(int From, int Until)> intervals)
        {
            foreach (var (From, Until) in intervals)
            {
                if (From < -50 || Until > 51)
                {
                    return false;
                }
            }
            return true;
        }

        internal long GetOnLights(bool useLimit)
        {
            var orderedBorders = CreateOrderedIntervalBorders(useLimit);

            var (xMap, yMap, zMap) = CreateMapsOrderedIntervalBordersAgainstIndex(orderedBorders);

            var intervalGrid = InitializeIntervalOnGrid(orderedBorders);

            EvaluateIntervalOnGrid(intervalGrid, xMap, yMap, zMap, useLimit);

            return CalculateSumLightsWithinIntervalOn(intervalGrid, orderedBorders);
        }

        private static long CalculateSumLightsWithinIntervalOn(int[][][] intervalGrid, IList<List<int>> borders)
        {
            long lightsOn = 0;
            for (int x = 0; x < intervalGrid.Length; x++)
            {
                for (int y = 0; y < intervalGrid[x].Length; y++)
                {
                    for (int z = 0; z < intervalGrid[x][y].Length; z++)
                    {
                        if (intervalGrid[x][y][z] == 1)
                        {
                            lightsOn +=
                                (long)(borders[0][x + 1] - borders[0][x]) *
                                (long)(borders[1][y + 1] - borders[1][y]) *
                                (long)(borders[2][z + 1] - borders[2][z]);
                        }
                    }
                }
            }
            return lightsOn;
        }

        private void EvaluateIntervalOnGrid(
            int[][][] intervalGrid,
            IDictionary<int, int> xMap, IDictionary<int, int> yMap, IDictionary<int, int> zMap,
            bool useLimit)
        {
            foreach (var (action, intervals) in instructions)
            {
                if (useLimit && !IsInstructionValid(intervals)) continue;

                for (int x = xMap[intervals[0].From]; x < xMap[intervals[0].Until]; x++)
                {
                    for (int y = yMap[intervals[1].From]; y < yMap[intervals[1].Until]; y++)
                    {
                        for (int z = zMap[intervals[2].From]; z < zMap[intervals[2].Until]; z++)
                        {
                            intervalGrid[x][y][z] = action;
                        }
                    }
                }
            }
        }

        private static int[][][] InitializeIntervalOnGrid(IList<List<int>> borders)
        {
            var intervalGrid = new int[borders[0].Count][][];
            for (int i = 0; i < borders[0].Count; i++)
            {
                intervalGrid[i] = new int[borders[1].Count][];
                for (int j = 0; j < borders[1].Count; j++)
                {
                    intervalGrid[i][j] = new int[borders[2].Count];
                }
            }
            return intervalGrid;
        }

        private static (IDictionary<int, int> xMap, IDictionary<int, int> yMap, IDictionary<int, int> zMap)
            CreateMapsOrderedIntervalBordersAgainstIndex(IList<List<int>> orderedBorders)
        {
            var xMap = CreateMap(orderedBorders[0]);
            var yMap = CreateMap(orderedBorders[1]);
            var zMap = CreateMap(orderedBorders[2]);

            return (xMap, yMap, zMap);
        }

        private static IDictionary<int, int> CreateMap(IList<int> list)
        {
            var map = new Dictionary<int, int>();
            for (int i = 0; i < list.Count; i++)
            {
                map.Add(list[i], i);
            }
            return map;
        }

        private IList<List<int>> CreateOrderedIntervalBorders(bool useLimit)
        {
            var orderedBorders = new List<List<int>>(3);
            foreach (var i in Enumerable.Range(0, 3))
            {
                var set = new HashSet<int>();
                foreach (var (_, interval) in instructions)
                {
                    if (useLimit && !IsInstructionValid(interval)) continue;

                    set.Add(interval[i].From);
                    set.Add(interval[i].Until);
                }
                orderedBorders.Add(set.OrderBy(i => i).ToList());
            }

            return orderedBorders;
        }
    }
}
