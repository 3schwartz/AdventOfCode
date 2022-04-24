namespace Day15
{
    internal class PathFinder
    {
        internal int FindShortestPath(string[] data)
        {
            var nRow = data.Length;
            var nCol = data[0].Length;

            var dataInt = new int[nRow][];
            for (var idx = 0; idx < nRow; idx++)
            {
                dataInt[idx] = new int[nCol];
                for(var jdx = 0; jdx < nCol; jdx++)
                {
                    dataInt[idx][jdx] = data[idx][jdx] - '0';
                }
            }

            var minimum = int.MaxValue;
            var i = 0;
            var j = 0;
            var path = -dataInt[i][j];

            return FollowPath(dataInt, i, j, nRow, nCol, path, minimum);
        }

        private int FollowPath(int[][] data, int i, int j, int nRow, int nCol, int path, int minimum)
        {
            path += data[i][j];

            if (path >= minimum)
            {
                return minimum;
            }
            if (i == nRow - 1 && j == nCol - 1)
            {
                Console.WriteLine($"Found minimum {path}");
                return path;
            }

            if (j < nCol - 1)
            {
                minimum = FollowPath(data, i, j + 1, nRow, nCol, path, minimum);
            }

            if (i < nRow - 1)
            {
                minimum = FollowPath(data, i + 1, j, nRow, nCol, path, minimum);
            }

            return minimum;
        }
    }
}
