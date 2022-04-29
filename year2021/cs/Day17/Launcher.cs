using System.Text.RegularExpressions;

namespace Day17
{
    internal class Launcher
    {
        public int xMax { get; }
        internal int xMin { get; }
        internal int yMax { get; }
        public int yMin { get; }

        public Launcher(string input)
        {
            var pattern = new Regex(@"(-?\d+)");
            var match = pattern.Matches(input);
            var ints = match.Select(c => c.Value).Select(int.Parse).ToList();
            xMin = ints[0];
            xMax = ints[1];
            yMin = ints[2];
            yMax = ints[3];
        }

        internal record struct MaxHorizontalVelocity(bool Found, int xVelocity, int yVelocity, int yMaxHeight);
        private record struct Horizontal(int X, int DeltaX, int Step);
        private record struct Vertical(int Y, int DeltaY, int Step);

        internal int GetVelocitiesCount()
        {
            var distinct = new HashSet<(int, int)>();
            GetPositionsInGrid().ToList()
                .ForEach(c => distinct.Add((c.Item2.DeltaX, c.Item1.DeltaY)));
            return distinct.Count;
        }

        internal MaxHorizontalVelocity GetMaxHorizontalVelocity()
        {
            foreach(var position in GetPositionsInGrid())
            {
                var yMax = Enumerable.Range(1, position.Item1.DeltaY)
                    .Aggregate(0, (c, p) => c + p);
                return new MaxHorizontalVelocity(true, position.Item2.DeltaX, position.Item1.DeltaY, yMax);
            };
            return new MaxHorizontalVelocity(false, 0,0,0);
        }

        private IEnumerable<(Vertical, Horizontal)> GetPositionsInGrid()
        {
            foreach (var vertical in GetVerticalPositions())
            {
                foreach(var horizontal in GetHorizontalAtStep(vertical.Step))
                {
                    yield return (vertical, horizontal);
                }
            }
        }

        private IEnumerable<Horizontal> GetHorizontalAtStep(int step)
        {
            foreach (var delta in Enumerable.Range(1, xMax + 1))
            {
                var dx = delta;
                var x = 0;
                foreach(var _ in Enumerable.Range(0, step))
                {
                    x += dx;
                    if(dx > 0)
                    {
                        dx--;
                    }
                    if(dx < 0)
                    {
                        dx++;
                    }
                }

                if (xMin <= x && x <= xMax)
                {
                    yield return new Horizontal(x, delta, step);
                }
            }
        }

        private IEnumerable<Vertical> GetVerticalPositions()
        {
            var delta = -yMin;
            while(delta >= yMin)
            {
                var y = 0;
                var step = 0;
                var dy = delta;
                do
                {
                    if (yMin <= y && y <= yMax)
                    {
                        yield return new Vertical(y, delta, step);
                    }
                    y += dy;
                    dy--;
                    step++;
                } while (y >= yMin);

                delta--;
            }
        }
    }
}