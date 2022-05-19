namespace Day20
{
    internal class ImageSet
    {
        private readonly Lazy<IList<(int, int)>> neighbors;
        private readonly IList<int> algorithm;
        private IList<(int,int)> Neighbors
        {
            get { return neighbors.Value; }
        }
        private ISet<(int, int)> image;
        
        internal ImageSet(string[] lines)
        {
            neighbors = new Lazy<IList<(int,int)>>(GetNeighbors);

            algorithm = CreateImageEnhancementAlgorithm(lines[0]);

            var imageLines = lines[2..];

            image = new HashSet<(int, int)>();
            for (int i = 0; i < imageLines.Length; i++)
            {
                var imageLine = imageLines[i];
                for (int j = 0; j < imageLine.Length; j++)
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
                foreach (var pixel in image)
                {
                    EvaluatePixel(visited, newImage, image, pixel);
                }

                image = newImage;
            }
        }

        internal int GetPixelCount()
        {
            return image.Count;
        }

        private void EvaluatePixel(ISet<(int,int)> visited,
            ISet<(int,int)> newImage, ISet<(int,int)> oldImage,
            (int,int) pixel)
        {
            if(!visited.Add(pixel)) return;

            var binaries = new int[9];
            var pixelNeighbors = FindPixelNeighbors(pixel.Item1, pixel.Item2);
            for (var i = 0; i < 9; i++)
            {
                binaries[i] = oldImage.Contains(pixelNeighbors[i]) ? 1 : 0;
            }
            
            var binaryRecord = CreateBinaryRecord(binaries);
            if(binaryRecord.Sum == 0) return;

            if (algorithm[binaryRecord.Lookup] == 1)
            {
                newImage.Add(pixel);
            }

            foreach (var pixelNeighbor in pixelNeighbors)
            {
                EvaluatePixel(visited, newImage, image, pixelNeighbor);
            }
        }

        private static BinaryResult CreateBinaryRecord(int[] ints)
        {
            var lookup = 0;
            var sum = 0;

            for (var i = 0; i < ints.Length; i++)
            {
                if (ints[^(i + 1)] == 1)
                {
                    lookup += (int)Math.Pow(2, i);
                }

                sum += ints[i];
            }
            return new BinaryResult(sum, lookup);
        }

        private record struct BinaryResult(int Sum, int Lookup);

        private IList<(int,int)> FindPixelNeighbors(int y, int x)
        {
            return Neighbors
                .Select(n => (y + n.Item1, x + n.Item2))
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
