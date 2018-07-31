using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndexSearchResult
    {
        private AnnoyIndexSearchResult()
        {
        }

        public int Count { get; private set; }

        public IReadOnlyList<long> IdList { get; private set; }

        public IReadOnlyList<float> DistanceList { get; private set; }

        internal static AnnoyIndexSearchResult LoadFromPtr(IntPtr searchResult)
        {
            var count = (int)NativeMethods.GetResultCount(searchResult);
            var idList = new long[count];
            var distanceList = new float[count];

            if (count > 0)
            {
                var idListPtr = NativeMethods.GetIdList(searchResult);
                Marshal.Copy(idListPtr, idList, 0, count);

                var distanceListPtr = NativeMethods.GetDistanceList(searchResult);
                Marshal.Copy(distanceListPtr, distanceList, 0, count);
            }

            return new AnnoyIndexSearchResult
            {
                Count = count,
                IdList = idList,
                DistanceList = distanceList,
            };
        }
    }
}
