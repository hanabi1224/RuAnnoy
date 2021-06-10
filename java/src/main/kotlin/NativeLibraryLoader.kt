package com.github.hanabi1224.RuAnnoy

import java.io.*

internal class NativeLibraryLoader {
    companion object {
        val os = System.getProperty("os.name").lowercase()
        val pwd = System.getProperty("user.dir")
        val isWin = os.contains("windows")
        val isLinux = os.contains("linux")

        fun getLibraryFileName(libName: String): String {
            if (isWin) {
                return "$libName.dll"
            }
            if (isLinux) {
                return "lib$libName.so"
            }
            return "lib$libName.dylib"
        }

        fun loadLibrary(prefix: String, libName: String): Boolean {
            val path = "$prefix${getLibraryFileName(libName)}"
            val url = NativeLibraryLoader::class.java.getResource(path)
            if (url == null) {
                return false
            }

            val tmpFile = File.createTempFile(libName, ".tmp")
            tmpFile.deleteOnExit()
            val outStream = tmpFile.outputStream()
            outStream.use {
                val inStream = url.openStream()
                inStream.use { inStream.copyTo(outStream) }
            }

            System.load(tmpFile.absolutePath)
            return true
        }
    }
}
