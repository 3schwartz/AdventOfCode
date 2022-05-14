using System;
using System.Collections.Generic;
using System.Linq;

namespace Day19.Tests
{
    internal class Scanner
    {
        private readonly Lazy<ISet<(int, int, int)>> beacons; 
        private ISet<(int, int, int)> Beacons { get {  return beacons.Value; } }

        private readonly Lazy<IList<HashSet<(int, int, int)>>> rotations;

        public IList<HashSet<(int, int, int)>> Rotations { get { return rotations.Value; }
        }

        private IList<HashSet<(int, int, int)>> GetRotation()
        {
            var rotations = new List<HashSet<(int, int, int)>>(24);

            var tempBeacons = new HashSet<(int,int,int)>();
            foreach (var beacon in Beacons)
            {
                tempBeacons.Add(beacon);
            }

            foreach(var _ in Enumerable.Range(0, 4))
            {
                foreach (var __ in Enumerable.Range(0, 4))
                {
                    rotations.Add(tempBeacons);
                    var beaconsFirstThird = new HashSet<(int, int, int)>();
                    foreach (var beacon in tempBeacons)
                    {
                        beaconsFirstThird.Add((beacon.Item3, beacon.Item2, -beacon.Item1));
                    }
                    tempBeacons = beaconsFirstThird;
                }
                var beaconsFirstTwoNegateFirst = new HashSet<(int, int, int)>();
                var beaconsFirstTwoNegateSecond = new HashSet<(int, int, int)>();
                foreach (var beacon in tempBeacons)
                {
                    beaconsFirstTwoNegateFirst.Add((beacon.Item2, -beacon.Item1, beacon.Item3));
                    beaconsFirstTwoNegateSecond.Add((-beacon.Item2, beacon.Item1, beacon.Item3));
                }
                rotations.Add(beaconsFirstTwoNegateFirst);
                rotations.Add(beaconsFirstTwoNegateSecond);

                var beaconsSecondAndThird = new HashSet<(int, int, int)>();
                foreach (var beacon in tempBeacons)
                {
                    beaconsSecondAndThird.Add((beacon.Item1, beacon.Item3, -beacon.Item2));
                }
                tempBeacons = beaconsSecondAndThird;
            }
            return rotations;
        }

        internal static int GetLargestManhattenDistance(ISet<(int, int, int)> beacons)
        {
            var beaconsList = beacons.ToList();
            
            var max = int.MinValue;
            for (int i = 0; i < beaconsList.Count; i++)
            {
                for (int j = 0; j < beaconsList.Count; j++)
                {
                    if (i == j) continue;

                    var first = beaconsList[i];
                    var second = beaconsList[j];
                    var distance = first.Item1 - second.Item1 +
                        first.Item2 - second.Item2 +
                        first.Item3 - second.Item3;
                    if(distance > max)
                    {
                        max = distance;
                    }
                }
            }
            return max;
        }

        public static IList<Scanner> CreateScanners(string lines)
        {
            return lines.Split("\r\n\r\n")
                .Select(scanner => new Scanner(scanner))
                .ToList();
        }

        public static ISet<(int,int,int)> FindBeacons(IList<Scanner> scanners)
        {
            var beaconsIntersected = scanners[0].Beacons;

            var queue = new Queue<int>(Enumerable.Range(1, scanners.Count-1));

            while(queue.Count > 0)
            {
                var idx = queue.Dequeue();

                var intersect = scanners[idx].GetIntersections(beaconsIntersected);
                if (intersect.Insersected)
                {
                    beaconsIntersected.UnionWith(intersect.Intersections);
                    Console.WriteLine($"Found scanner: {idx}, beacons count: {beaconsIntersected.Count}");
                    continue;
                }
                queue.Enqueue(idx);
                Console.WriteLine($"Tried scanner: {idx}");
            }
            return beaconsIntersected;
        }

        private record struct IntersectResult(bool Insersected, ISet<(int,int,int)> Intersections);

        private IntersectResult GetIntersections(ISet<(int, int, int)> beaconsIntersected)
        {
            foreach(var rotation in Rotations)
            {
                foreach(var beacon in beaconsIntersected)
                {
                    foreach(var shift in rotation)
                    {
                        (int,int,int) offset = Shift(shift, beacon);
                        var intersections = new HashSet<(int, int, int)>();
                        foreach(var beaconInRotation in rotation)
                        {
                            intersections.Add(Shift(beaconInRotation, offset));
                        }

                        var intersect = beaconsIntersected.Intersect(intersections);

                        if (intersect.Count() >= 12)
                        {
                            return new IntersectResult(true, intersections);
                        }
                    }
                }
            }

            return new IntersectResult(false, new HashSet<(int,int,int)>());
        }

        private static (int, int, int) Shift((int, int, int) shift, (int, int, int) beacon)
        {
            return (shift.Item1 - beacon.Item1, shift.Item2 - beacon.Item2, shift.Item3 - beacon.Item3);
        }

        public Scanner(string lines)
        {
            beacons = new(() => new HashSet<(int, int, int)>());
            rotations = new Lazy<IList<HashSet<(int, int, int)>>>(() => GetRotation());

            Span<int> ints = stackalloc int[3];
            var idx = 0;
            foreach (var line in lines.Split("\r\n")[1..])
            {
                foreach(var b in line.Trim().Split(','))
                {
                    var parsed = int.TryParse(b, out var coordinate);
                    if (!parsed)
                    {
                        Console.WriteLine($"Error parsing {b}");
                    }
                    ints[idx] = coordinate;
                    idx = (idx + 1) % ints.Length;
                }
                
                Beacons.Add((ints[0], ints[1], ints[2]));
            }
        }
    }
}
