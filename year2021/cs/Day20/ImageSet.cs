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
            neighbors = new Lazy<IList<(int,int)>>(() => GetNeighBors());

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

        private static IList<(int,int)> GetNeighBors()
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

        internal void Enchance(int times)
        {
            var visited = new HashSet<(int,int)>();
            var newImage = new HashSet<(int,int)>();
            Span<int> binaries = stackalloc int[9];
            foreach(var pixel in image)
            {
                visited.Add(pixel);
                var pixelNeighbors = FindPixelNeighbors(pixel);
                for(var i = 0; i < 9; i++)
                {
                    binaries[i] = image.Contains(pixelNeighbors[i]) ? 1 : 0;
                }
                var pixelLookup = CreateBinaryFromSpan(binaries);
                if(algorithm[pixelLookup] == 1)
                {
                    newImage.Add(pixel);
                }
            }
            

        }

        private static void EvaluatePixel(ISet<(int,int)> visited,
            ISet<(int,int)> newImage, ISet<(int,int)> oldImage,
            (int,int) pixel)
        {
            visited.Add(pixel);
            var pixelNeighbors = FindPixelNeighbors(pixel);
            for (var i = 0; i < 9; i++)
            {
                binaries[i] = oldImage.Contains(pixelNeighbors[i]) ? 1 : 0;
            }
            var pixelLookup = CreateBinaryFromSpan(binaries);
            if (algorithm[pixelLookup] == 1)
            {
                newImage.Add(pixel);
            }
        }

        private static int CreateBinaryFromSpan(Span<int> ints)
        {
            var value = 0;

            for (var i = 0; i < ints.Length; i++)
            {
                if (ints[^(i + 1)] == 1)
                {
                    value += (int)Math.Pow(2, i);
                }
            }
            return value;
        }

        private IList<(int,int)> FindPixelNeighbors((int,int) pixel)
        {
            return Neighbors
                .Select(n => (pixel.Item1 + n.Item1, pixel.Item1 + n.Item2))
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
