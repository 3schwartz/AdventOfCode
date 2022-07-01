using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Day7.Tests
{
    internal static class CoderAlgorithms
    {
        internal static int FindMaxThrusterSignal(IList<int> codes,
            Func<IList<int>, (int A, int B, int C, int D, int E), Task<int>> findThrusterSignal)
        {
            Span<int> inputs = stackalloc int[5] { -1, -1, -1, -1, -1 };
            var maxSignal = int.MinValue;

            for (var first = 5; first < 10; first++)
            {
                inputs[0] = first;
                for (var second = 5; second < 10; second++)
                {
                    if (inputs.Contains(second)) continue;
                    inputs[1] = second;

                    for (var third = 5; third < 10; third++)
                    {
                        if (inputs.Contains(third)) continue;
                        inputs[2] = third;

                        for (var fourth = 5; fourth < 10; fourth++)
                        {
                            if (inputs.Contains(fourth)) continue;
                            inputs[3] = fourth;

                            for (var fifth = 5; fifth < 10; fifth++)
                            {
                                if (inputs.Contains(fifth)) continue;
                                inputs[4] = fifth;

                                var last = findThrusterSignal(codes,
                                        (inputs[0], inputs[1], inputs[2], inputs[3], inputs[4]))
                                    .Result;

                                if (last > maxSignal)
                                {
                                    maxSignal = last;
                                }
                                inputs[4] = -1;
                            }
                            inputs[3] = -1;
                        }
                        inputs[2] = -1;
                    }
                    inputs[1] = -1;
                }
                inputs[0] = -1;
            }
            return maxSignal;
        }
    }
}
