using System;
using System.Runtime.InteropServices;

namespace RuAnnoy
{
    internal static class NativeMethods
    {
        const string DLLPATH = @"annoy_rs_ffi";

        [DllImport(DLLPATH, EntryPoint = "load_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern IntPtr LoadAnnoyIndex(
            string path,
            Int32 dimension,
            IndexType indexType);

        [DllImport(DLLPATH, EntryPoint = "free_annoy_index", CharSet = CharSet.Ansi)]
        internal static extern void FreeAnnoyIndex(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_dimension", CharSet = CharSet.Ansi)]
        internal static extern int GetDimension(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_size", CharSet = CharSet.Ansi)]
        internal static extern ulong GetSize(IntPtr index);

        [DllImport(DLLPATH, EntryPoint = "get_item_vector", CharSet = CharSet.Ansi)]
        internal static extern void GetItemVector(IntPtr index, ulong itemIndex, [Out] float[] itemVector);

        [DllImport(DLLPATH, EntryPoint = "get_nearest", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetNearest(
            IntPtr index,
            float[] vector,
            uint nResults,
            int searchK,
            bool shouldIncludeDistance);

        [DllImport(DLLPATH, EntryPoint = "get_nearest_to_item", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetNearestToItem(
            IntPtr index,
            ulong itemIndex,
            uint nResults,
            int searchK,
            bool shouldIncludeDistance);

        [DllImport(DLLPATH, EntryPoint = "free_search_result", CharSet = CharSet.Ansi)]
        internal static extern void FreeSearchResult(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_result_count", CharSet = CharSet.Ansi)]
        internal static extern ulong GetResultCount(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_id_list", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetIdList(IntPtr searchResult);

        [DllImport(DLLPATH, EntryPoint = "get_distance_list", CharSet = CharSet.Ansi)]
        internal static extern IntPtr GetDistanceList(IntPtr searchResult);
    }
}
