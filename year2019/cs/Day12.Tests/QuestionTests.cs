using System.Collections.Concurrent;
using System.Collections.Immutable;

namespace Day12.Tests
{
    public record struct Person(int Age);

    // Best way to return a list from parallel execution
    // https://stackoverflow.com/questions/67010378/c-sharp-how-to-populate-and-return-listt-from-parallel-foreach-using-partition
    public static class Question {

        public static IList<Person> PLinq(IList<Person> input)
        {
            return input.AsParallel().Select(p =>
            {
                p.Age += 1;
                return p;
            }).ToList();
        }

        // Example from Microsoft uses ConcurrentBag
        // https://docs.microsoft.com/en-us/dotnet/standard/parallel-programming/how-to-write-a-simple-parallel-foreach-loop
        public static IList<Person> ParallelForEachConcurrentBag(IList<Person> input)
        {
            var persons = new ConcurrentBag<Person>();
            Parallel.ForEach(input, (p) =>
            {
                p.Age += 1;
                persons.Add(p);
            });
            return persons.ToList();
        }

        public static IList<Person> ParallelForEachLock(IList<Person> input)
        {
            object mutex = new();
            var persons = new List<Person>(input.Count);
            Parallel.ForEach(input, (p) =>
            {
                p.Age += 1;
                lock (mutex)
                {
                    persons.Add(p);
                }
            });
            return persons;
        }

        public static IList<Person> ParallelForEachLocalInitLock(IList<Person> input)
        {
            object mutex = new();
            var persons = new List<Person>(input.Count);

            Parallel.ForEach(
                source: input,
                localInit: () => new List<Person>(),
                body: (p, state, local) => {
                    p.Age += 1;
                    local.Add(p);
                    return local;
                },
                local =>
                {
                    lock (mutex)
                    {
                        persons.AddRange(local);
                    }
                });
            return persons;
        }

        public static IList<Person> ParallelForEachImmutable(IList<Person> input)
        {
            var persons = ImmutableList<Person>.Empty;
            Parallel.ForEach(input, (p) =>
            {
                p.Age += 1;
                ImmutableInterlocked.Update(ref persons, l => l.Add(p));
            });
            return persons;
        }

        public static IList<Person> ParallelForEachLocalInitImmutable(IList<Person> input)
        {
            var persons = ImmutableList<Person>.Empty;
            Parallel.ForEach(
                source: input,
                localInit: () => new List<Person>(),
                body: (p, state, local) => {
                    p.Age += 1;
                    local.Add(p);
                    return local;
                },
                local =>
                {
                    ImmutableInterlocked.Update(ref persons, l => l.AddRange(local));
                });
            return persons;
        }

        public static async Task<IList<Person>> TaskAsync(IList<Person> input)
        {
            var tasks = new List<Task<Person>>();
            foreach (var p in input)
            {
                tasks.Add(Task.Run(() => UpdateAge(p)));
            }

            return await Task.WhenAll(tasks);
        }

        private static Person UpdateAge(Person p)
        {
            p.Age += 1;
            return p;
        }
    }

    public class QuestionTests
    {
        private readonly IList<Person> expected = new List<Person>()
        {
            new Person{Age=2},new Person{Age=4},new Person{Age=6}
        };
        private readonly IList<Person> input = new List<Person>()
        {
            new Person{Age=1},new Person{Age=3},new Person{Age=5}
        };

        [Fact]
        public void PLinqTest() {
            // Act
            var actual = Question.PLinq(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }

        #region ConcurrentBag
        [Fact]
        public void ParallelForEachConcurrentBagTest()
        {
            // Act
            var actual = Question.ParallelForEachConcurrentBag(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }
        #endregion

        #region Lock
        [Fact]
        public void ParallelForEachLockTest()
        {
            // Act
            var actual = Question.ParallelForEachLock(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }

        [Fact]
        public void ParallelForEachLocalInitLockTest()
        {
            // Act
            var actual = Question.ParallelForEachLocalInitLock(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }
        #endregion

        #region Immutable
        [Fact]
        public void ParallelForEachImmutableTest()
        {
            // Act
            var actual = Question.ParallelForEachImmutable(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }

        [Fact]
        public void ParallelForEachLocalInitImmutableTest()
        {
            // Act
            var actual = Question.ParallelForEachLocalInitImmutable(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }
        #endregion

        [Fact]
        public async Task TaskTest()
        {
            // Act
            var actual = await Question.TaskAsync(input);

            // Assert
            Assert.True(actual.All(expected.Contains));
        }
    }
}
