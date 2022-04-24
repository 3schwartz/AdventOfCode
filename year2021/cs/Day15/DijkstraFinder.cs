namespace Day15
{
    internal class DijkstraFinder
    {
        internal Node[][] CreateNodes(string[] data)
        {
            var nRow = data.Length;
            var nCol = data[0].Length;

            var nodes = new Node[nRow][];
            for (var idx = 0; idx < nRow; idx++)
            {
                nodes[idx] = new Node[nCol];
                for (var jdx = 0; jdx < nCol; jdx++)
                {
                    nodes[idx][jdx] = new Node(data[idx][jdx] - '0', idx, jdx);
                }
            }
            nodes[0][0] = new Node(0, 0, 0, 0);

            return nodes;
        }

        internal class Node
        {
            public int Cost { get; private set; }
            public int OverallCost { get; private set; } = int.MaxValue;
            public IDictionary<char, int> Coordinate { get; private set; }
            public Node PreviousNode { get; private set; }
            private Node() { }
            public Node(int cost, int x, int y)
            {
                Cost = cost;
                Coordinate = new Dictionary<char, int>
                {
                    {'x', x }, {'y', y}
                };
            }
            public Node(int cost, int x, int y, int overallCost) 
                : this(cost, x, y)
            {
                OverallCost = overallCost;
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
