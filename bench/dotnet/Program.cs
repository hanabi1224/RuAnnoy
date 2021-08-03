using System;
using System.Diagnostics;
using RuAnnoy;

namespace Bench
{
    class Program
    {
        static void Main(int dim, ulong size, uint nResult, ulong nLoop)
        {
            foreach (var metric in new[] { "angular", "euclidean" })
            {
                var path = $"index.{metric}.{dim}d.ann";
                var index = AnnoyIndex.Load(path, dim, Enum.Parse<IndexType>(metric, ignoreCase: true));                
                var sw = Stopwatch.StartNew();
                for (ulong i = 0; i < nLoop; i++)
                {
                    var id = i % size;
                    var vector = index.GetItemVector(id);
                    index.GetNearest(vector, nResult, -1, true);
                }
                sw.Stop();
                Console.WriteLine($"[Dotnet] RuAnnoy");
                Console.WriteLine($"[{metric}] Total time elapsed: {sw.Elapsed.TotalSeconds}s");
                Console.WriteLine($"[{metric}] Avg time elapsed: {sw.ElapsedMilliseconds/(float)nLoop}ms");
                Console.WriteLine();
            }
        }
    }
}
