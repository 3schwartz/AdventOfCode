﻿using System.Runtime.CompilerServices;

namespace Day20
{
    internal class ImageSet
    {
        private readonly Lazy<IList<(int, int)>> neighbors;
        private readonly IList<int> algorithm;
        private IList<(int,int)> Neighbors => neighbors.Value;
        private ISet<(int, int)> image;
        private int minX = -2;
        private int minY = -2;
        private int maxX;
        private int maxY;
        private int neighborInitValue = 0;
        internal ImageSet(string[] lines)
        {
            neighbors = new Lazy<IList<(int,int)>>(GetNeighbors);

            algorithm = CreateImageEnhancementAlgorithm(lines[0]);

            var imageLines = lines[2..];
            maxY = imageLines.Length + 2;
            maxX = imageLines[0].Length + 2;

            image = new HashSet<(int, int)>();
            for (var i = 0; i < imageLines.Length; i++)
            {
                var imageLine = imageLines[i];
                for (var j = 0; j < imageLine.Length; j++)
                {
                    if(imageLine[j] == '#')
                    {
                        image.Add((i,j));
                    }
                }
            }
        }

        private static IList<(int,int)> GetNeighbors()
        {
            var neighbors = new List<(int,int)>(9);
            foreach(var y in Enumerable.Range(-1, 3))
            {
                foreach (var x in Enumerable.Range(-1, 3))
                {
                    neighbors.Add((y,x));
                }
            }
            return neighbors;
        }

        internal void Enhance(int times)
        {
            foreach (var _ in Enumerable.Range(0, times))
            {
                var visited = new HashSet<(int, int)>();
                var newImage = new HashSet<(int, int)>();
                var toVisit = image;
                do
                {
                    var nextToVisit = new HashSet<(int, int)>();
                    foreach (var pixel in toVisit)
                    {
                        EvaluatePixel(visited, newImage, image, pixel, nextToVisit);
                    }
                    nextToVisit.ExceptWith(visited);
                    toVisit = nextToVisit;
                } while (toVisit.Count > 0);

                minY -= 2;
                minX -= 2;
                maxX += 2;
                maxY += 2;

                var initNeighborResolved = GetInitNeighborResolved();
                neighborInitValue = algorithm[initNeighborResolved];

                image = newImage;
            }
        }

        internal int GetPixelCount()
        {
            return image.Count;
        }

        private int GetInitNeighborResolved()
        {
            Span<int> initNeighborBinary = stackalloc int[9];

            for (var i = 0; i < initNeighborBinary.Length; i++)
            {
                initNeighborBinary[i] = neighborInitValue;
            }

            var value = 0;

            for (var i = 0; i < initNeighborBinary.Length; i++)
            {
                if (initNeighborBinary[^(i + 1)] == 1)
                {
                    value += (int)Math.Pow(2, i);
                }
            }

            return value;
        }

        private bool IsOuterNeighbor((int,int) pixel)
        {
            return pixel.Item1 > maxY || pixel.Item1 < minY ||
                   pixel.Item2 > maxX || pixel.Item2 < minX;
        }

        private int GetBinaryValue(ISet<(int,int)> oldImage, IList<(int,int)> pixelNeighbors, int i)
        {
            if (oldImage.Contains(pixelNeighbors[i]))
            {
                return 1;
            }

            if (IsOuterNeighbor(pixelNeighbors[i]))
            {
                return neighborInitValue;
            }

            return 0;
        }

        private void EvaluatePixel(ISet<(int,int)> visited,
            ISet<(int,int)> newImage, ISet<(int,int)> oldImage,
            (int,int) pixel, ISet<(int,int)> nextToVisit)
        {
            visited.Add(pixel);
            
            Span<int> binaries = stackalloc int[9];
            var pixelNeighbors = FindPixelNeighbors(pixel);
            for (var i = 0; i < 9; i++)
            {
                binaries[i] = GetBinaryValue(oldImage, pixelNeighbors, i);
            }

            var binaryRecord = CreateBinaryRecord(binaries);

            if (algorithm[binaryRecord.Lookup] == 1)
            {
                newImage.Add(pixel);
            }

            foreach (var pixelNeighbor in pixelNeighbors)
            {
                if(IsOuterNeighbor(pixelNeighbor)) continue;
                nextToVisit.Add(pixelNeighbor);
            }
        }

        private static BinaryResult CreateBinaryRecord(Span<int> binaries)
        {
            var lookup = 0;
            var sum = 0;

            for (var i = 0; i < binaries.Length; i++)
            {
                if (binaries[^(i + 1)] == 1)
                {
                    lookup += (int)Math.Pow(2, i);
                }

                sum += binaries[i];
            }
            return new BinaryResult(sum, lookup);
        }

        private record struct BinaryResult(int Sum, int Lookup);

        private IList<(int,int)> FindPixelNeighbors((int,int) pixel)
        {
            return Neighbors
                .Select(n => (pixel.Item1 + n.Item1, pixel.Item2 + n.Item2))
                .ToList();
        }

        internal static IList<int> CreateImageEnhancementAlgorithm(string line)
        {
            var algorithm = new List<int>(line.Length);
            foreach (var element in line)
            {
                algorithm.Add(element == '#' ? 1 : 0);
            }
            return algorithm;
        }
    }
}
