namespace Day15
{
    internal class PriorityQueueFinder
    {
        internal Node[][] CreateNodes(string[] data, int numberOfTiles)
        {
            var nRow = data.Length;
            var nCol = data[0].Length;

            var nodes = new Node[nRow * numberOfTiles][];
            for (var idx = 0; idx < nRow * numberOfTiles; idx++)
            {
                nodes[idx] = new Node[nCol * numberOfTiles];
                for (var jdx = 0; jdx < nCol * numberOfTiles; jdx++)
                {
                    nodes[idx][jdx] = new Node(data, idx, jdx, nRow, nCol);
                }
            }
            nodes[0][0] = new Node(0, 0, 0, 0);

            return nodes;
        }

        internal class Node
        {
            public int Cost { get; }
            public int OverallCost { get; private set; } = int.MaxValue;
            public IDictionary<char, int> Coordinate { get; }
            public Node? PreviousNode { get; private set; }
            private Node() { }
            public Node(int cost, int x, int y, int overallCost)
            {
                Cost = cost;
                Coordinate = new Dictionary<char, int>
                {
                    {'x', x }, {'y', y}
                };
                OverallCost = overallCost;
            }

            public Node(string[] data, int idx, int jdx, int nRow, int nCol)
            {
                Cost = CalculateNodeTileCost(data, idx, jdx, nRow, nCol);
                Coordinate = new Dictionary<char, int>
                {
                    {'x', idx }, {'y', jdx}
                };
            }

            private int CalculateNodeTileCost(string[] data, int idx, int jdx, int nRow, int nCol)
            {
                var cost = data[idx % nRow][jdx % nCol] - '0' + idx / nRow + jdx / nCol;
                return (cost - 1) % 9 + 1;
            }

            internal void FindNodeCost(Node previous, PriorityQueue<Node, int> queue)
            {
                if (OverallCost != int.MaxValue) return;

                var updatedCost = Cost + previous.OverallCost;
                if (updatedCost < OverallCost)
                {
                    OverallCost = updatedCost;
                    PreviousNode = previous;
                }

                queue.Enqueue(this, OverallCost);
            }
        }

        internal int FindShortest(Node[][] nodes)
        {
            var queue = new PriorityQueue<Node, int>();
            queue.Enqueue(nodes[0][0], 0);

            var nRow = nodes.Length;
            var nCol = nodes[0].Length;

            while (queue.Count != 0)
            {
                var node = queue.Dequeue();
                foreach (var neighbors in FindNeigbors(node, nRow, nCol))
                {
                    nodes[neighbors.Item1][neighbors.Item2].FindNodeCost(node, queue);
                }
            }

            return nodes[nRow-1][nCol-1].OverallCost;
        }

        internal IList<(int, int)> FindNeigbors(Node node, int nRow, int nCol)
        {
            var neighbors = new List<(int, int)>();
            var x = node.Coordinate['x'];
            var y = node.Coordinate['y'];

            if (x > 1) neighbors.Add((x - 1, y));
            if (x < nRow - 1) neighbors.Add((x + 1, y));
            if (y > 1) neighbors.Add((x, y - 1));
            if (y < nCol - 1) neighbors.Add((x, y + 1));

            return neighbors;
        }
    }
}
