using System;

namespace RuAnnoy
{
    public class Program
    {
        public static void Main()
        {
            var index = new AnnoyIndex(
                @"C:\Users\harlo\git\hanabi1224\RuAnnoy\tests\test.10d.ann",
                10,
                IndexType.Angular);

            Console.WriteLine(index.Dimension);
            var vector = index.GetItemVector(1);
            var result = index.GetNearest(vector, 5, -1, true);
            Console.WriteLine(result.Count);
        }
    }
}
