using System;

public static class ResistorColor
{
    public static int ColorCode(string color)
    {
        var colors = Colors();
        return Array.IndexOf(colors, color);
    }

    public static string[] Colors()
    {
        return new string[] { "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white" };
    }
}