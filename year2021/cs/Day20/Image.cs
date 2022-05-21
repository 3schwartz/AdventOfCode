using System.Collections;
using System.Diagnostics;

namespace Day20;

internal class Image
{

    private readonly Lazy<IList<(int, int)>> neighbors = new(GetNeighbors);
    private IList<(int,int)> Neighbors => neighbors.Value;
    private int neighborInitValue = 0;
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
            .Select(n => (pixel.Item1 + n.Item1, pixel.Item2 + n.Item2))
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
                    image.TryAdd(pixelNeighbor, neighborInitValue);
                }
            }

            var initNeighborResolved = GetInitNeighborResolved();
            neighborInitValue = imageEnhancementAlgorithm[initNeighborResolved];

            var tmpImage = new DefaultDict();
            foreach (var ((item1, item2), _) in image)
            {
                var within = true;
                var idx = 0;
                for (int y = -1; y <= 1; y++)
                {
                    for (int x = -1; x <= 1 ; x++)
                    {
                        within &= image.TryGetValueAndAddToNew(tmpImage, (item1 + y, item2 + x), out var value);
                        if (within)
                        {
                            binaries[idx] = value;
                            idx++;
                            continue;
                        }
                        break;
                    }

                }
                if (within)
                {

                    var resolvedPixel = ResolvePixel(binaries);

                    var updatePixel = imageEnhancementAlgorithm[resolvedPixel];

                    tmpImage.Add((item1, item2), updatePixel);
                    continue;
                }

                tmpImage.Add((item1, item2), neighborInitValue);
            }

            image = tmpImage;
        }
    }

    private int GetInitNeighborResolved()
    {
        Span<int> initNeighborBinary = stackalloc int[9];

        for (var i = 0; i < initNeighborBinary.Length; i++)
        {
            initNeighborBinary[i] = neighborInitValue;
        }

        return ResolvePixel(initNeighborBinary);
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
        foreach (var element in line)
        {
            algorithm.Add(GetSign(element).Sign);
        }
        return algorithm;
    }
}