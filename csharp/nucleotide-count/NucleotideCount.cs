using System;
using System.Collections.Generic;

public static class NucleotideCount
{
    public static IDictionary<char, int> Count(string sequence)
    {
        var count = new Dictionary<char, int>(){
            {'A', 0},
            {'T', 0},
            {'C', 0},
            {'G', 0}
        };

        foreach (char c in sequence)
        {
            if (!"ATCG".Contains(c))
            {
                throw new ArgumentException("Invalid character in sequence.");
            }
            count[c]++;
        }

        return count;
    }
}