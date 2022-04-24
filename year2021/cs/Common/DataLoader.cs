namespace Common
{
    public static class DataLoader
    {
        public static string[] GetData(string path)
        {
            return File.ReadAllLines(path);
        }
    }
}
