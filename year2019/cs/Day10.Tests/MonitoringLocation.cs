namespace Day10.Tests;

internal readonly struct MonitoringLocation
{
    public int DetectedAsteroids { get; }
    public (int X,int Y) Coordinate { get; }

    public MonitoringLocation(int detectedAsteroids, (int,int) coord)
    {
        DetectedAsteroids = detectedAsteroids;
        Coordinate = coord;
    }
}
