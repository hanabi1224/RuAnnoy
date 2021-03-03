using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    internal static class NativeMethods
    {
        const string DLLPATH = @"ru_annoy";

        [DllImport(DLLPATH, EntryPoint = "load_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern IntPtr LoadAnnoyIndex(
            string path,
            Int32 dimension,
            IndexType indexType);

        [DllImport(DLLPATH, EntryPoint = "free_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern IntPtr FreeAnnoyIndex(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_dimension", CharSet = CharSet.Ansi)]
        internal static extern Int32 GetDimension(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_item_vector", CharSet = CharSet.Ansi)]
        internal static extern void GetItemVector(IntPtr index, Int64 itemIndex, [Out] float[] itemVector);

        [DllImport(DLLPATH, EntryPoint = "get_nearest", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetNearest(
            IntPtr index,
            float[] vector,
            UIntPtr nResults,
            Int32 searchK,
            bool shouldIncludeDistance);

        [DllImport(DLLPATH, EntryPoint = "get_nearest_to_item", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetNearestToItem(
            IntPtr index,
            Int64 itemIndex,
            UIntPtr nResults,
            Int32 searchK,
            bool shouldIncludeDistance);

        [DllImport(DLLPATH, EntryPoint = "free_search_result", CharSet = CharSet.Ansi)]
        internal static extern IntPtr FreeSearchResult(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_result_count", CharSet = CharSet.Ansi)]
        internal static extern UIntPtr GetResultCount(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_id_list", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetIdList(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_distance_list", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetDistanceList(IntPtr searchResult);
    }
}
