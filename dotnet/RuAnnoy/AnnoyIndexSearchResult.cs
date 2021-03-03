using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndexSearchResult
    {
        private AnnoyIndexSearchResult()
        {
        }

        public int Count { get; private set; }

        public bool IsDistanceIncluded { get; private set; }

        public ReadOnlyMemory<long> IdList { get; private set; }

        public ReadOnlyMemory<float> DistanceList { get; private set; }

        internal static AnnoyIndexSearchResult LoadFromPtr(IntPtr searchResult, bool isDistanceIncluded)
        {
            var count = (int)NativeMethods.GetResultCount(searchResult);
            var result = new AnnoyIndexSearchResult
            {
                Count = count,
                IsDistanceIncluded = isDistanceIncluded,
            };

            if (count > 0)
            {
                var idList = new long[count];
                var idListPtr = NativeMethods.GetIdList(searchResult);
                Marshal.Copy(idListPtr, idList, 0, count);
                result.IdList = idList;

                if (isDistanceIncluded)
                {
                    var distanceList = new float[count];
                    var distanceListPtr = NativeMethods.GetDistanceList(searchResult);
                    Marshal.Copy(distanceListPtr, distanceList, 0, count);
                    result.DistanceList = distanceList;
                }
            }

            return result;
        }
    }
}
