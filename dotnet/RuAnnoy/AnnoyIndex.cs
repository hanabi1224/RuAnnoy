using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    public class AnnoyIndex : DisposeBase, IAnnoyIndex
    {
        private IntPtr _indexPtr;

        private AnnoyIndex(
            IntPtr indexPtr,
            int dimension,
            IndexType type)
        {
            _indexPtr = indexPtr;
            Dimension = dimension;
            Type = type;
        }

        public int Dimension { get; }

        public IndexType Type { get; }

        public static IAnnoyIndex? Load(
            string path,
            int dimension,
            IndexType type)
        {
            var indexPtr = NativeMethods.LoadAnnoyIndex(path, dimension, type);
            if (indexPtr != IntPtr.Zero)
            {
                return new AnnoyIndex(indexPtr, dimension, type);
            }
            else
            {
                return null;
            }
        }

        public IReadOnlyList<float> GetItemVector(long itemIndex)
        {
            if (_indexPtr == IntPtr.Zero)
            {
                throw new ObjectDisposedException("index");
            }

            var itemVector = new float[Dimension];
            NativeMethods.GetItemVector(_indexPtr, itemIndex, itemVector);
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
                return AnnoyIndexSearchResult.LoadFromPtr(searchResultPtr, shouldIncludeDistance);
            }
            finally
            {
                NativeMethods.FreeSearchResult(searchResultPtr);
            }
        }

        public AnnoyIndexSearchResult GetNearestToItem(
            long itemIndex,
            ulong nResult,
            int searchK,
            bool shouldIncludeDistance)
        {
            if (_indexPtr == IntPtr.Zero)
            {
                throw new ObjectDisposedException("index");
            }

            var searchResultPtr = NativeMethods.GetNearestToItem(
                  _indexPtr,
                  itemIndex,
                  new UIntPtr(nResult),
                  searchK,
                  shouldIncludeDistance);
            try
            {
                return AnnoyIndexSearchResult.LoadFromPtr(searchResultPtr, shouldIncludeDistance);
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
