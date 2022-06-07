using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using Xunit;

namespace Day23.Tests;

public class Day23Tests
{
    [Fact]
    public void WhenAreStackEqual()
    {
        var stackOne = CreateRoom('B', 'A');
        var stackSecond = CreateRoom('B', 'A');

        var foo = stackOne.Equals(stackSecond);

        var hej = "hej";
    }

    [Fact]
    public void WhenAreStateEqual()
    {
        // Arrang
        var roomsOne = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', 'A')};
        var roomsSecond = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', 'A')};
        var stateOne = new State(Enumerable.Repeat('.', 11).ToList(), roomsOne);
        var stateSecond = new State(Enumerable.Repeat('.', 11).ToList(), roomsSecond);
        var visited = new HashSet<State>();

        // Act
        visited.Add(stateOne);
        visited.Add(stateSecond);

        // Assert
        Assert.Single(visited);
    }

    [Fact]
    public void GivenRooms_WhenCalculateLeastEnergy_ThenCorrect()
    {
        // Arrange
        var rooms = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', 'A')};

        // Act
        var energy = CalculateLeastEnergy(rooms);

        // Assert
        Assert.Equal(12521, energy);
    }

    private Stack<char> CreateRoom(char outer, char inner)
    {
        var stack = new Stack<char>();
        stack.Push(inner);
        stack.Push(outer);
        return stack;
    }



    private readonly IDictionary<char, int> MoveCost = new Dictionary<char, int> { { 'A', 1 }, { 'B', 10 }, { 'C', 100 }, { 'D', 1000 } };

    private int CalculateLeastEnergy(List<Stack<char>> rooms)
    {
        Enumerable.Repeat('.', 11).ToList();

        new State(Enumerable.Repeat('.', 11).ToList(), rooms);

        return 0;
    }

    private record struct State(IList<char> HallWay, IList<Stack<char>> Rooms) : IEqualityComparer<State>
    {
        public bool Equals(State other) => Equals(this, other);

        public bool Equals(State current, State other)
        {
            var equals = true;
            equals &= current.HallWay.SequenceEqual(other.HallWay);

            Span<char> roomThis = stackalloc char[2];
            Span<char> roomOther = stackalloc char[2];

            for (int i = 0; i < current.Rooms.Count; i++)
            {
                equals &= current.Rooms[i].Count == other.Rooms[i].Count;
                for (int j = 0; j < current.Rooms[i].Count; j++)
                {
                    roomThis = current.Rooms[i].ToArray();
                    roomOther = other.Rooms[i].ToArray();
                    equals &= roomThis.SequenceEqual(roomOther);
                }
            }

            return equals;
        }

        public override int GetHashCode()
        {
            return GetHashCode(this);
        }

        public int GetHashCode([DisallowNull] State obj)
        {
            var hash = 1;
            hash = obj.HallWay.Select(h => h.GetHashCode()).Aggregate(hash, (current, h) => current + h);

            Span<char> room = stackalloc char[2];
            for (int i = 0; i < obj.Rooms.Count; i++)
            {
                for (int j = 0; j < obj.Rooms[i].Count; j++)
                {
                    hash = obj.Rooms[i].ToArray().Select(h => h.GetHashCode()).Aggregate(hash, (current, h) => current + h);
                }
            }
            return hash;
        }
    }
}