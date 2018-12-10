using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndex : DisposeBase, IAnnoyIndex
    {
        private IntPtr _indexPtr;

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

        public IReadOnlyList<float> GetItemVector(long itemIndex)
        {
            if (_indexPtr == IntPtr.Zero)
            {
                throw new ObjectDisposedException("index");
            }

            var itemVectorPtr = NativeMethods.GetItemVector(_indexPtr, itemIndex);
            var itemVector = new float[Dimension];
            Marshal.Copy(itemVectorPtr, itemVector, 0, Dimension);
            return itemVector;
        }

        public AnnoyIndexSearchResult GetNearest(
            IReadOnlyList<float> queryVector,
            ulong nResult,
            int searchK,
            bool shouldIncludeDistance)
        {
            if (_indexPtr == IntPtr.Zero)
            {
                throw new ObjectDisposedException("index");
            }

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

        protected override void DisposeResources()
        {
            NativeMethods.FreeAnnoyIndex(_indexPtr);
            _indexPtr = IntPtr.Zero;
        }
    }
}
