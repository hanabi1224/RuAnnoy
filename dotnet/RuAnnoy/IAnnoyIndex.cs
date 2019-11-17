using System;
using System.Collections.Generic;

namespace RuAnnoy
{
    public interface IAnnoyIndex : IDisposable
    {
        int Dimension { get; }

        IReadOnlyList<float> GetItemVector(long itemIndex);

        AnnoyIndexSearchResult GetNearest(
            IReadOnlyList<float> queryVector,
            ulong nResult,
            int searchK,
            bool shouldIncludeDistance);
    }
}
