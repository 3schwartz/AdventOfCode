namespace Day16
{
    internal static class HexConverter
    {
        internal static ReadOnlySpan<char> ToBinary(string hexString)
        {
            return
                string.Join(string.Empty,
                    hexString
                        .Select(hex =>
                            Convert.ToString(Convert.ToInt32(hex.ToString(), 16), 2)
                                .PadLeft(4, '0'))).AsSpan();
        }
    }
}
