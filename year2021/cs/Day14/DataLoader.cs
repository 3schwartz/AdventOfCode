namespace Day14
{
    internal static class DataLoader
    {
        internal static string[] GetData(string path)
        {
            return File.ReadAllLines(path);
        }
    }
}
