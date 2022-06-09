using System.Diagnostics.CodeAnalysis;

namespace Day23
{
    internal class AmphipodRoomSorter
    {
        private readonly IReadOnlyDictionary<char, int> moveCost =
        new Dictionary<char, int> { { 'A', 1 }, { 'B', 10 }, { 'C', 100 }, { 'D', 1000 } };
        private readonly IReadOnlyDictionary<char, int> finalRoomIdx =
            new Dictionary<char, int> { { 'A', 2 }, { 'B', 4 }, { 'C', 6 }, { 'D', 8 } };
        private readonly IReadOnlyDictionary<int, char> finalRoomState =
            new Dictionary<int, char> { { 0, 'A' }, { 1, 'B' }, { 2, 'C' }, { 3, 'D' } };
        private readonly ISet<State> visited = new HashSet<State>();
        private readonly IReadOnlyList<int> hallwayEntries = new List<int>() { 2, 4, 6, 8 };

        private readonly char[] end = Enumerable.Repeat('A', 2)
            .Concat(Enumerable.Repeat('B', 2))
            .Concat(Enumerable.Repeat('C', 2))
            .Concat(Enumerable.Repeat('D', 2))
            .ToArray();

        internal int CalculateLeastEnergy(List<Stack<char>> rooms)
        {
            var initialState = new State(Enumerable.Repeat('.', 11).ToList(), rooms);

            var leastEnergyPath = new PriorityQueue<State, int>();
            leastEnergyPath.Enqueue(initialState, 0);

            while (leastEnergyPath.Count > 0)
            {
                leastEnergyPath.TryDequeue(out var state, out var cost);

                if (IsEnd(state)) return cost;

                if (visited.Contains(state)) continue;

                visited.Add(state);

                foreach (var (newCost, newState) in GetPossibleStates(state, cost))
                {
                    if (visited.Contains(newState)) continue;

                    leastEnergyPath.Enqueue(newState, newCost);
                }
            }

            return 0;
        }

        private IList<(int Cost, State State)> GetPossibleStates(State state, int cost)
        {
            var newStates = new List<(int Cost, State State)>();
            AddAmphipodsWhichCanMoveOutFromRooms(newStates, state, cost);
            AddAmphipodsWhichCanMoveIntoRooms(newStates, state, cost);

            return newStates;
        }

        private void AddAmphipodsWhichCanMoveIntoRooms(IList<(int Cost, State State)> newStates, State state, int cost)
        {
            for (var i = 0; i <= 10; i++)
            {
                if (state.HallWay[i] == '.') continue;
                var possibleRooms = new List<int>();
                AddRightFreeSpots(possibleRooms, state, i);
                AddLeftFreeSpots(possibleRooms, state, i);

                foreach (var room in possibleRooms)
                {
                    if (!hallwayEntries.Contains(room)) continue;

                    var destination = finalRoomIdx[state.HallWay[i]];

                    if (destination != room) continue;

                    var destinationRoom = state.Rooms[destination / 2 - 1];

                    if (!IsRoomPossibleToEnter(destinationRoom, state.HallWay[i])) continue;

                    var clone = state.Copy();
                    clone.HallWay[i] = '.';
                    clone.Rooms[destination / 2 - 1].Push(state.HallWay[i]);
                    var totalCost = cost + moveCost[state.HallWay[i]] *
                        (Math.Abs(destination - i) + 3 - clone.Rooms[destination / 2 - 1].Count);
                    newStates.Add((totalCost, clone));
                }
            }
        }

        private void AddAmphipodsWhichCanMoveOutFromRooms(IList<(int Cost, State State)> newStates, State state, int cost)
        {
            for (var i = 0; i < 4; i++)
            {
                var finalState = finalRoomState[i];
                var room = state.Rooms[i];

                if (room.All(c => c == finalState)) continue;

                var hallwayIdx = (i + 1) * 2;
                if (room.Count == 0) continue;

                var roomCost = moveCost[room.Peek()] * (3 - room.Count);

                // TODO: Optimize list creation
                var possibleHallwayPositions = new List<int>();
                AddRightFreeSpots(possibleHallwayPositions, state, hallwayIdx);
                AddLeftFreeSpots(possibleHallwayPositions, state, hallwayIdx);

                foreach (var position in possibleHallwayPositions)
                {
                    if (hallwayEntries.Contains(position)) continue;

                    var clone = state.Copy();
                    var moved = clone.Rooms[i].Pop();
                    clone.HallWay[position] = moved;
                    var totalCost = cost + roomCost + moveCost[moved] * Math.Abs(hallwayIdx - position);
                    newStates.Add((totalCost, clone));

                }
            }
        }
        private static bool IsRoomPossibleToEnter(IReadOnlyCollection<char> room, char toEnter)
        {
            return room.Count == 0 || room.All(c => c == toEnter);
        }

        private static void AddRightFreeSpots(IList<int> positions, State state, int idx)
        {
            for (var j = idx + 1; j <= 10; j++)
            {
                if (state.HallWay[j] != '.') break;
                positions.Add(j);
            }
        }

        private static void AddLeftFreeSpots(IList<int> positions, State state, int idx)
        {
            for (var j = idx - 1; j >= 0; j--)
            {
                if (state.HallWay[j] != '.') break;
                positions.Add(j);
            }
        }

        private bool IsEnd(State state)
        {
            Span<char> flatten = stackalloc char[8];
            var idx = 0;
            foreach (var room in state.Rooms)
            {
                foreach (var member in room)
                {
                    flatten[idx] = member;
                    idx++;
                }
            }

            return flatten.SequenceEqual(end);
        }

        internal record struct State(IList<char> HallWay, IList<Stack<char>> Rooms) : IEqualityComparer<State>
        {
            private static void GetFlattenState(State state, ref Span<char> flatten)
            {
                var idx = 0;
                foreach (var place in state.HallWay)
                {
                    flatten[idx] = place;
                    idx++;
                }

                foreach (var room in state.Rooms)
                {
                    foreach (var member in room)
                    {
                        flatten[idx] = member;
                        idx++;
                    }
                }
            }

            public readonly bool Equals(State other) => Equals(this, other);

            public bool Equals(State current, State other)
            {
                Span<char> currentFlat = stackalloc char[19];
                GetFlattenState(current, ref currentFlat);
                Span<char> otherFlatten = stackalloc char[19];
                GetFlattenState(other, ref otherFlatten);

                return currentFlat.SequenceEqual(otherFlatten);
            }

            public override int GetHashCode() => GetHashCode(this);

            public int GetHashCode([DisallowNull] State obj)
            {
                Span<char> flatten = stackalloc char[19];
                GetFlattenState(obj, ref flatten);
                var hash = 1;
                foreach (var flat in flatten)
                {
                    hash ^= flat.GetHashCode();
                }

                return hash;
            }

            public State Copy()
            {
                var hallway = new List<char>();
                foreach (var c in HallWay)
                {
                    hallway.Add(c);
                }

                var rooms = new List<Stack<char>>();
                foreach (var room in Rooms)
                {
                    rooms.Add(new Stack<char>(room.ToArray().Reverse()));
                }

                return new State(hallway, rooms);
            }
        }
    }
}
