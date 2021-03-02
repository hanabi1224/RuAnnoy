package com.github.hanabi1224.RuAnnoy;

// jni: javac -h . src\main\java\com\github\hanabi1224\RuAnnoy\NativeMethods.java
class NativeMethods
{
    native static long loadIndex(String path, int dimension, byte type);

    native static void freeIndex(long pointer);

    native static float[] getItemVector(long indexPointer, long itemIndex);

    native static int getNearestToItem(
            long indexPointer,
            long itemIndex, 
            int nResult, 
            int searchK, 
            boolean shouldIncludeDistance,
            long[] idList,
            float[] distanceList
    );
        
    native static int getNearest(
            long indexPointer,
            float[] queryVector, 
            int nResult, 
            int searchK, 
            boolean shouldIncludeDistance,
            long[] idList,
            float[] distanceList
        );
}
