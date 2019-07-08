using System;
using System.Collections.Generic;

public class Robot
{
    private static Random rnd = new Random();
    private const string Letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    private const string Numbers = "0123456789";
    private static HashSet<string> usedNames = new HashSet<string>();
    public string Name { get; set; }

    public Robot()
    {
        Reset();
    }

    private string GenerateName()
    {
        string name;

        do
        {
            name = GetLetterPrefix() + GetRandomNumbers();
        } while (usedNames.Contains(name));

        usedNames.Add(name);

        return name;
    }

    private string GetLetterPrefix()
    {
        string prefix = "";

        for (int i = 0; i < 2; i++)
        {
            prefix += Letters[rnd.Next(Letters.Length)];
        }

        return prefix;
    }

    private string GetRandomNumbers()
    {
        string nums = "";

        for (int i = 0; i < 3; i++)
        {
            nums += Numbers[rnd.Next(Numbers.Length)];
        }

        return nums;
    }

    public void Reset()
    {
        Name = GenerateName();
    }
}