using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndex : IAnnoyIndex, IDisposable
    {
        private readonly IntPtr _indexPtr;

        public AnnoyIndex(
            string path,
            int dimension,
            IndexType type)
        {
            Dimension = dimension;
            _indexPtr = NativeMethods.LoadAnnoyIndex(path, dimension, type);
        }

        public int Dimension { get; }

        public static IAnnoyIndex Load(
            string path,
            int dimension,
            IndexType type)
        {
            return new AnnoyIndex(path, dimension, type);
        }

        public ReadOnlyMemory<float> GetItemVector(long itemIndex)
        {
            var itemVectorPtr = NativeMethods.GetItemVector(_indexPtr, itemIndex);
            var itemVector = new float[Dimension];
            Marshal.Copy(itemVectorPtr, itemVector, 0, Dimension);
            return itemVector;
        }

        public AnnoyIndexSearchResult GetNearest(
            ReadOnlyMemory<float> queryVector,
            ulong nResult,
            int searchK,
            bool shouldIncludeDistance)
        {
            var searchResultPtr = NativeMethods.GetNearest(
                  _indexPtr,
                  queryVector.ToArray(),
                  new UIntPtr(nResult),
                  searchK,
                  shouldIncludeDistance);
            try
            {
                return AnnoyIndexSearchResult.LoadFromPtr(searchResultPtr);
            }
            finally
            {
                NativeMethods.FreeSearchResult(searchResultPtr);
            }
        }

        public void Dispose()
        {
            NativeMethods.FreeAnnoyIndex(_indexPtr);
        }
    }
}
