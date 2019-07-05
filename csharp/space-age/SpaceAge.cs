using System;
using System.Collections.Generic;

public class SpaceAge
{
    private int seconds;
    private const double EarthSecondsPerYear = 31557600.0;

    private Dictionary<string, double> yearLength;
    public SpaceAge(int seconds)
    {
        this.seconds = seconds;
        yearLength = new Dictionary<string, double> {
            {"Mercury", 0.2408467 * EarthSecondsPerYear },
            {"Venus", 0.61519726 * EarthSecondsPerYear },
            {"Earth", EarthSecondsPerYear },
            {"Mars", 1.8808158 * EarthSecondsPerYear },
            {"Jupiter", 11.862615 * EarthSecondsPerYear },
            {"Saturn", 29.447498 * EarthSecondsPerYear },
            {"Uranus", 84.016846 * EarthSecondsPerYear },
            {"Neptune", 164.79132 * EarthSecondsPerYear }
        };
    }

    public double OnEarth() => seconds / yearLength["Earth"];

    public double OnMercury() => seconds / yearLength["Mercury"];

    public double OnVenus() => seconds / yearLength["Venus"];

    public double OnMars() => seconds / yearLength["Mars"];

    public double OnJupiter() => seconds / yearLength["Jupiter"];

    public double OnSaturn() => seconds / yearLength["Saturn"];

    public double OnUranus() => seconds / yearLength["Uranus"];

    public double OnNeptune() => seconds / yearLength["Neptune"];
}