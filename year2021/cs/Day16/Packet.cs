namespace Day16
{
    internal class Packet
    {
        public int Version { get; }
        public int Id { get; }
        public long? LiteralValue { get; }

        public IList<Packet> Packets { get; } = new List<Packet>();

        internal Packet(ref ReadOnlySpan<char> packet)
        {
            Version = Convert.ToInt32(packet[..3].ToString(), 2);
            packet = packet[3..];
            Id = Convert.ToInt32(packet[..3].ToString(), 2);
            packet = packet[3..];
            if (Id == 4)
            {
                LiteralValue = DecodeVarInt(ref packet);
            }
            else
            {
                var lengthTypeId = packet[0];
                packet = packet[1..];
                switch (lengthTypeId)
                {
                    case '0':
                        {
                            var subPacketsLength = Convert.ToInt32(packet[..15]
                                .ToString(), 2);
                            packet = packet[15..];

                            var subPackets = packet[..subPacketsLength];
                            while (subPackets.Length != 0)
                            {
                                Packets.Add(new Packet(ref subPackets));
                            }

                            packet = packet[subPacketsLength..];
                            break;
                        }
                    case '1':
                        {
                            var subPacketsCount = Convert.ToInt32(packet[..11].ToString(), 2);
                            packet = packet[11..];
                            for (var i = 0; i < subPacketsCount; i++)
                            {
                                Packets.Add(new Packet(ref packet));
                            }

                            break;
                        }
                }
            }
        }

        [Obsolete("Just for fun to compare memory allocation compared to span")]
        internal Packet(ref string packet)
        {
            Version = Convert.ToInt32(packet[..3], 2);
            packet = packet[3..];
            Id = Convert.ToInt32(packet[..3], 2);
            packet = packet[3..];
            if (Id == 4)
            {
                LiteralValue = DecodeVarInt(ref packet);
            }
            else
            {
                var lengthTypeId = packet[0];
                packet = packet[1..];
                switch (lengthTypeId)
                {
                    case '0':
                        {
                            var subPacketsLength = Convert.ToInt32(packet[..15], 2);
                            packet = packet[15..];

                            var subPackets = packet[..subPacketsLength];
                            while (subPackets.Length != 0)
                            {
                                Packets.Add(new Packet(ref subPackets));
                            }

                            packet = packet[subPacketsLength..];
                            break;
                        }
                    case '1':
                        {
                            var subPacketsCount = Convert.ToInt32(packet[..11], 2);
                            packet = packet[11..];
                            for (var i = 0; i < subPacketsCount; i++)
                            {
                                Packets.Add(new Packet(ref packet));
                            }

                            break;
                        }
                }
            }
        }

        internal long? Evaluate()
        {
            return Id switch
            {
                0 => Packets.Aggregate((long?)0, (a, b) => a + b.Evaluate()),
                1 => Packets.Aggregate((long?)1, (a, b) => a * b.Evaluate()),
                2 => Packets.Min(p => p.Evaluate()),
                3 => Packets.Max(p => p.Evaluate()),
                4 => LiteralValue!,
                5 => BiggestPackage(),
                6 => SmalletstPackage(),
                7 => EqualPackage(),
                _ => throw new ArgumentException($"Id not known: {Id}"),
            };
        }

        private long? EqualPackage()
        {
            var first = Packets[0].Evaluate();
            var second = Packets[1].Evaluate();
            return first == second ? 1 : 0;
        }

        private long? BiggestPackage()
        {
            var first = Packets[0].Evaluate();
            var second = Packets[1].Evaluate();
            return first > second ? 1 : 0;
        }

        private long? SmalletstPackage()
        {
            var first = Packets[0].Evaluate();
            var second = Packets[1].Evaluate();
            return first < second ? 1 : 0;
        }

        internal int GetTotalVersion()
        {
            return Version + Packets.Sum(packet => packet.GetTotalVersion());
        }

        private long DecodeVarInt(ref ReadOnlySpan<char> literal)
        {
            var binaries = new List<string>(8);
            char readMore;
            do
            {
                readMore = literal[0];
                binaries.Add(literal.Slice(1, 4).ToString());
                literal = literal[5..];
            } while (readMore != '0');

            return Convert.ToInt64(string.Join(string.Empty, binaries), 2);
        }

        [Obsolete("Just for fun to compare memory allocation compared to span")]
        private long DecodeVarInt(ref string literal)
        {
            var binaries = new List<string>(8);
            char readMore;
            do
            {
                readMore = literal[0];
                binaries.Add(literal[1..5]);
                literal = literal[5..];
            } while (readMore != '0');

            return Convert.ToInt64(string.Join(string.Empty, binaries), 2);
        }
    }
}
