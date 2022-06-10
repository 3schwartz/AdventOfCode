using System.Diagnostics.CodeAnalysis;

namespace Day23
{
    internal class AmphipodStringSorter
    {
        private readonly IReadOnlyDictionary<char, int> moveCost =
        new Dictionary<char, int> { { 'A', 1 }, { 'B', 10 }, { 'C', 100 }, { 'D', 1000 } };
        private readonly IReadOnlyDictionary<char, int> finalRoomIdx =
            new Dictionary<char, int> { { 'A', 2 }, { 'B', 4 }, { 'C', 6 }, { 'D', 8 } };
        private readonly IReadOnlyDictionary<int, char> finalRoomState =
            new Dictionary<int, char> { { 0, 'A' }, { 1, 'B' }, { 2, 'C' }, { 3, 'D' } };
        private readonly ISet<string> visited = new HashSet<string>();
        private readonly IReadOnlyList<int> hallwayEntries = new List<int>() { 2, 4, 6, 8 };

        private readonly string end;
        private readonly string initial;
        private readonly int roomSizes;

        public AmphipodStringSorter(IList<char> rooms, int roomSizes)
        {
            end = GetEnd(roomSizes);
            initial = GetInitial(rooms, roomSizes);
            this.roomSizes = roomSizes;
        }

        internal static string GetEnd(int roomSize)
        {
            Span<char> state = stackalloc char[11 + 4 * roomSize];
            for (var i = 0; i < 11; i++)
            {
                state[i] = '.';
            }
            AddChar('A', ref state, 1, roomSize);
            AddChar('B', ref state, 2, roomSize);
            AddChar('C', ref state, 3, roomSize);
            AddChar('D', ref state, 4, roomSize);

            return state.ToString();
        }

        private static void AddChar(char add, ref Span<char> state, int room, int roomSize)
        {
            for (var i = 11 + roomSize * room - roomSize; i < 11 + roomSize * room; i++)
            {
                state[i] = add;
            }
        }

        private static void AddChar(IList<char> add, ref Span<char> state, int room, int roomSize)
        {
            for (var i = 11 + roomSize * room - roomSize; i < 11 + roomSize * room; i++)
            {
                state[i] = add[i-11];
            }
        }

        internal static string GetInitial(IList<char> rooms, int roomSize)
        {
            Span<char> state = stackalloc char[11 + 4 * roomSize];
            for (var i = 0; i < 11; i++)
            {
                state[i] = '.';
            }
            AddChar(rooms, ref state, 1, roomSize);
            AddChar(rooms, ref state, 2, roomSize);
            AddChar(rooms, ref state, 3, roomSize);
            AddChar(rooms, ref state, 4, roomSize);

            return state.ToString();
        }

        internal int CalculateLeastEnergy()
        {
            var leastEnergyPath = new PriorityQueue<string, int>();
            leastEnergyPath.Enqueue(initial, 0);

            while (leastEnergyPath.Count > 0)
            {
                leastEnergyPath.TryDequeue(out var state, out var cost);

                if (state.Equals(end)) return cost;

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

        internal IList<(int Cost, string State)> GetPossibleStates(string state, int cost)
        {
            var newStates = new List<(int Cost, string State)>();
            AddAmphipodsWhichCanMoveOutFromRooms(newStates, state, cost);
            AddAmphipodsWhichCanMoveIntoRooms(newStates, state, cost);

            return newStates;
        }


        internal ReadOnlySpan<char> GetRoom(int i, string state)
        {
            return state.AsSpan().Slice(11 + i * roomSizes, roomSizes);
        }

        private static bool IsRoomCorrect(char finalState, ReadOnlySpan<char> room)
        {
            foreach (var c in room)
            {
                if(c != finalState) return false;
            }
            return true;
        }

        private static bool IsRoomEmpty(ReadOnlySpan<char> room)
        {
            foreach (var c in room)
            {
                if (c != '.') return false;
            }
            return true;
        }

        private static (char,int) RoomOuterElement(ReadOnlySpan<char> room)
        {
            for (var i = 0; i < room.Length; i++)
            {
                if (room[i] != '.') return (room[i], i);
            }

            throw new InvalidOperationException($"Room shouldn't be empty {room}");
        }

        private static int RoomOuterFreeElement(ReadOnlySpan<char> room)
        {
            var idx = 0;
            foreach (var c in room)
            {
                if(c != '.') return idx;
                idx++;
            }
            return idx;

        }

        private static int RoomSize(ReadOnlySpan<char> room)
        {
            var size = 0;
            foreach (var c in room)
            {
                if (c != '.') size++;
            }
            return size;
        }


        private void AddAmphipodsWhichCanMoveOutFromRooms(IList<(int Cost, string State)> newStates, string state, int cost)
        {
            for (var i = 0; i < 4; i++)
            {
                var finalState = finalRoomState[i];
                var room = GetRoom(i, state);

                // TODO: Maybe remove
                //if (IsRoomCorrect(finalState, room)) continue;
                if (IsRoomEmpty(room)) continue;

                var hallwayIdx = (i + 1) * 2;
                var roomSize = RoomSize(room);
                var (outerElement, idx) = RoomOuterElement(room);

                var roomCost = moveCost[outerElement] * (1 + roomSizes - roomSize);

                //// TODO: Optimize list creation
                var possibleHallwayPositions = new List<int>();
                AddRightFreeSpots(possibleHallwayPositions, state, hallwayIdx);
                AddLeftFreeSpots(possibleHallwayPositions, state, hallwayIdx);

                foreach (var position in possibleHallwayPositions)
                {
                    if (hallwayEntries.Contains(position)) continue;
                    
                    Span<char> clone = stackalloc char[state.Length];
                    state.CopyTo(clone);
                    clone[position] = outerElement;
                    clone[11 + i * roomSizes + idx] = '.';
                    var totalCost = cost + roomCost + moveCost[outerElement] * Math.Abs(hallwayIdx - position);
                    newStates.Add((totalCost, clone.ToString()));
                }
            }
        }

        private void AddAmphipodsWhichCanMoveIntoRooms(IList<(int Cost, string State)> newStates, string state, int cost)
        {
            for (var i = 0; i <= 10; i++)
            {
                if (state[i] == '.') continue;
                var possibleRooms = new List<int>();
                AddRightFreeSpots(possibleRooms, state, i);
                AddLeftFreeSpots(possibleRooms, state, i);

                foreach (var room in possibleRooms)
                {
                    if (!hallwayEntries.Contains(room)) continue;

                    var destination = finalRoomIdx[state[i]];

                    if (destination != room) continue;

                    var destinationRoom = GetRoom(destination / 2 - 1, state);

                    if (!IsRoomPossibleToEnter(destinationRoom, state[i])) continue;

                    Span<char> clone = stackalloc char[state.Length];
                    state.CopyTo(clone);
                    clone[i] = '.';
                    clone[11 + (destination / 2 - 1) + RoomOuterFreeElement(destinationRoom)] = state[i];
                    var totalCost = cost + moveCost[state[i]] *
                        (Math.Abs(destination - i) + 1 + roomSizes - RoomSize(destinationRoom) + 1);
                    newStates.Add((totalCost, clone.ToString()));
                }
            }
        }

        private static bool IsRoomPossibleToEnter(ReadOnlySpan<char> room, char toEnter)
        {
            if (room.Length == 0) return true;

            foreach (var c in room)
            {
                if(c == '.') continue;
                if(c!= toEnter) return false;
            }

            return true;
        }

        private static void AddRightFreeSpots(IList<int> positions, string state, int idx)
        {
            for (var j = idx + 1; j <= 10; j++)
            {
                if (state[j] != '.') break;
                positions.Add(j);
            }
        }

        private static void AddLeftFreeSpots(IList<int> positions, string state, int idx)
        {
            for (var j = idx - 1; j >= 0; j--)
            {
                if (state[j] != '.') break;
                positions.Add(j);
            }
        }
    }
}
