using BenchmarkDotNet.Attributes;
using Day16;

namespace Benchmark
{
    [MemoryDiagnoser]
    [SimpleJob(launchCount: 1, warmupCount: 1, targetCount: 3)]
    public class Day16Benchmark
    {
        private string hexString;

        [GlobalSetup]
        public void GlobalSetup()
        {
            hexString = File.ReadAllText("../../../../../../../../../data/day16_data.txt");
        }

        [Benchmark]
        public void Packet()
        {
            var binary = HexConverter.ToBinary(hexString);

            var packet = new Packet(ref binary);
            _ = packet.GetTotalVersion();
        }

        [Benchmark]
        public void PacketAsString()
        {
            var binary = HexConverter.ToBinary(hexString).ToString();

            var packet = new Packet(ref binary);
            _ = packet.GetTotalVersion();
        }
    }
}
