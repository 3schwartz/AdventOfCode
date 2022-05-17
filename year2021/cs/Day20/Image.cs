using System.Collections;
using System.Diagnostics;

namespace Day20;

internal class Image
{
    private DefaultDict image;

    internal Image(string[] lines)
    {
        var imageLines = lines[2..];
        image = new DefaultDict(imageLines);
    }

    internal void Enchance()
    {
        var tmpImage = new DefaultDict();
        Span<int> binaries = stackalloc int[9];
        foreach(var (key,value) in image)
        {
            //image.
        }

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
            for (int i = 0; i < imageLines.Length; i++)
            {
                var imageLine = imageLines[i];
                for (int j = 0; j < imageLine.Length; j++)
                {
                    dict.Add((i, j), GetSign(imageLine[j]).Sign);
                }
            }
        }

        internal DefaultDict()
        {
            dict = new Dictionary<(int, int), int>();
        }

        public IEnumerator<((int, int), int)> GetEnumerator()
        {
            foreach (var (key, value) in dict)
            {
                yield return (key, value);
            }
        }

        internal void Add((int,int) key, int value)
        {
            dict.Add(key, value);
        }

        internal int GetValue(DefaultDict newDict, (int,int) key)
        {
            var exists = dict.TryGetValue(key, out var value);
            if (exists)
            {
                return value;
            }
            newDict.Add(key, 0);
            return 0;
        }

        internal int GetValue((int,int) key)
        {
            var exists = dict.TryGetValue(key, out var value);
            if (exists)
            {
                return value;
            }
            dict.Add(key, 0);
            return 0;
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