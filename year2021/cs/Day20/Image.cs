using System.Collections;
using System.Diagnostics;

namespace Day20;

internal class Image
{

    private readonly Lazy<IList<(int, int)>> neighbors = new(GetNeighbors);
    private IList<(int,int)> Neighbors => neighbors.Value;

    private static IList<(int, int)> GetNeighbors()
    {
        var neighbors = new List<(int, int)>(25);
        for (var i = -2; i <= 2; i++)
        {
            for (var j = -2; j <=2 ; j++)
            {
                neighbors.Add((i,j));
            }
        }

        return neighbors;
    }

    private IList<(int, int)> FindNeighbors((int, int) pixel)
    {
        return Neighbors
            .Select(n => (pixel.Item1 - n.Item1, pixel.Item2 - n.Item2))
            .ToList();
    }



    private DefaultDict image;
    private readonly IList<int> imageEnhancementAlgorithm;

    internal Image(string[] lines)
    {
        imageEnhancementAlgorithm = CreateImageEnhancementAlgorithm(lines[0]);
        var imageLines = lines[2..];
        image = new DefaultDict(imageLines);
    }

    internal void Enhance(int times)
    {
        Span<int> binaries = stackalloc int[9];
        foreach (var _ in Enumerable.Range(0, times))
        {
            foreach (var valueTuple in image.GetKeys())
            {
                var pixelNeighbors = FindNeighbors(valueTuple);
                foreach (var pixelNeighbor in pixelNeighbors)
                {
                    image.TryAdd(pixelNeighbor, 0);
                }
            }

            var tmpImage = new DefaultDict();
            foreach (var ((item1, item2), _) in image)
            {
                if (
                    image.TryGetValueAndAddToNew(tmpImage, (item1 - 1, item2 - 1), out var zero) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1 - 1, item2), out var one) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1 - 1, item2 + 1), out var second) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1, item2 - 1), out var third) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1, item2), out var fourth) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1, item2 + 1), out var fifth) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1 + 1, item2 - 1), out var six) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1 + 1, item2), out var seven) &&
                    image.TryGetValueAndAddToNew(tmpImage, (item1 + 1, item2 + 1), out var eight)
                )
                {
                    binaries[0] = zero;
                    binaries[1] = one;
                    binaries[2] = second;
                    binaries[3] = third;
                    binaries[4] = fourth;
                    binaries[5] = fifth;
                    binaries[6] = six;
                    binaries[7] = seven;
                    binaries[8] = eight;

                    var resolvedPixel = ResolvePixel(binaries);

                    var updatePixel = imageEnhancementAlgorithm[resolvedPixel];

                    tmpImage.Add((item1, item2), updatePixel);
                }
            }

            image = tmpImage;
        }
    }

    internal int GetPixelCount()
    {
        var pixelCount = 0;
        foreach (var value in image.GetValues())
        {
            pixelCount += value;
        }

        return pixelCount;
    }

    private int ResolvePixel(Span<int> ints)
    {
        var value = 0;

        for (var i = 0; i < ints.Length; i++)
        {
            if (ints[^(i + 1)] == 1)
            {
                value += Image.Power(2, i);
            }
        }

        return value;
    }

    internal static int Power(int value, int power)
    {
        if (power == 0)
        {
            return 1;
        }

        var final = 1;
        while (power > 0)
        {
            final *= value;
            power--;
        }

        return final;
    }

    internal class DefaultDict : IEnumerable<((int,int), int)>
    {
        private readonly Dictionary<(int, int), int> dict;
        internal DefaultDict(string[] imageLines)
        {
            dict = new Dictionary<(int, int), int>(imageLines.Length * imageLines[0].Length);
            for (var i = 0; i < imageLines.Length; i++)
            {
                var imageLine = imageLines[i];
                for (var j = 0; j < imageLine.Length; j++)
                {
                    dict.Add((i, j), GetSign(imageLine[j]).Sign);
                }
            }
        }

        internal DefaultDict()
        {
            dict = new Dictionary<(int, int), int>();
        }

        internal Dictionary<(int, int), int>.ValueCollection GetValues()
        {
            return dict.Values;
        }

        public IEnumerator<((int, int), int)> GetEnumerator()
        {
            foreach (var (key, value) in dict)
            {
                yield return (key, value);
            }
        }

        internal IList<(int, int)> GetKeys()
        {
            return dict.Keys.ToList();
        }

        internal void Add((int, int) key, int value)
        {
            dict[key] = value;
        }

        internal void TryAdd((int,int) key, int value)
        {
            dict.TryAdd(key, value);
        }

        internal bool TryGetValueAndAddToNew(DefaultDict newDict, (int,int) key, out int value)
        {
            if (dict.TryGetValue(key, out value))
            {
                newDict.TryAdd(key, value);
                return true;
            }

            return false;
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }

    private static SignResult GetSign(char sign)
    {
        switch (sign)
        {
            case '.':
                return new SignResult(true, 0);
            case '#':
                return new SignResult(true, 1);
            default:
                Debug.WriteLine("Element isn't known");
                return new SignResult(false, 0);
        }
    }

    private record struct SignResult(bool Found, int Sign);

    internal static IList<int> CreateImageEnhancementAlgorithm(string line)
    {
        var algorithm = new List<int>(line.Length);
        for (int i = line.Length-1; i >= 0; i--)
        {
            algorithm.Add(GetSign(line[i]).Sign);
        }
        //foreach (var element in line)
        //{
        //    algorithm.Add(GetSign(element).Sign);
        //}
        return algorithm;
    }
}