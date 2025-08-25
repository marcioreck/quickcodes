using System;
using System.Reflection;
using System.Runtime.InteropServices;

namespace QuickCodes
{
    public enum BarcodeType
    {
        QRCode,
        EAN13,
        UPCA,
        Code128,
        DataMatrix,
        PDF417,
        Aztec
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct QuickCodesError
    {
        public IntPtr Message;
        public int Code;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct QuickCodesResult
    {
        public IntPtr Data;
        public UIntPtr Length;
        public IntPtr Error;
    }

    public class BarcodeException : Exception
    {
        public int ErrorCode { get; }

        public BarcodeException(string message, int errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }
    }

    public static class BarcodeGenerator
    {
        private const string LibraryName = "libquickcodes";

        static BarcodeGenerator()
        {
            NativeLibrary.SetDllImportResolver(typeof(BarcodeGenerator).Assembly, ImportResolver);
        }

        private static IntPtr ImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName == LibraryName)
            {
                // Try to load from the same directory as the assembly
                var assemblyDir = Path.GetDirectoryName(assembly.Location);
                if (assemblyDir != null)
                {
                    var libraryPath = Path.Combine(assemblyDir, "libquickcodes.so");
                    if (NativeLibrary.TryLoad(libraryPath, out var handle))
                    {
                        return handle;
                    }
                }

                // Try to load from system library path
                if (NativeLibrary.TryLoad("libquickcodes.so", out var sysHandle))
                {
                    return sysHandle;
                }
            }

            return IntPtr.Zero;
        }

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr quickcodes_generate(
            [MarshalAs(UnmanagedType.LPStr)] string barcodeType,
            [MarshalAs(UnmanagedType.LPStr)] string data);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr quickcodes_generate_to_file(
            [MarshalAs(UnmanagedType.LPStr)] string barcodeType,
            [MarshalAs(UnmanagedType.LPStr)] string data,
            [MarshalAs(UnmanagedType.LPStr)] string outputPath);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr quickcodes_read_from_file(
            [MarshalAs(UnmanagedType.LPStr)] string filePath);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr quickcodes_read_from_bytes(
            byte[] data,
            UIntPtr length);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void quickcodes_free_result(IntPtr result);

        public static byte[] Generate(BarcodeType barcodeType, string data)
        {
            if (string.IsNullOrEmpty(data))
            {
                throw new ArgumentException("Data cannot be empty", nameof(data));
            }

            var result = quickcodes_generate(barcodeType.ToString(), data);
            if (result == IntPtr.Zero)
            {
                throw new BarcodeException("Failed to generate barcode", -1);
            }

            try
            {
                var quickCodesResult = Marshal.PtrToStructure<QuickCodesResult>(result);
                if (quickCodesResult.Error != IntPtr.Zero)
                {
                    var error = Marshal.PtrToStructure<QuickCodesError>(quickCodesResult.Error);
                    var message = Marshal.PtrToStringAnsi(error.Message);
                    throw new BarcodeException(message ?? "Unknown error", error.Code);
                }

                if (quickCodesResult.Data == IntPtr.Zero)
                {
                    throw new BarcodeException("No data returned", -1);
                }

                var length = (int)quickCodesResult.Length;
                var output = new byte[length];
                Marshal.Copy(quickCodesResult.Data, output, 0, length);
                return output;
            }
            finally
            {
                quickcodes_free_result(result);
            }
        }

        public static void GenerateToFile(BarcodeType barcodeType, string data, string outputPath)
        {
            if (string.IsNullOrEmpty(data))
            {
                throw new ArgumentException("Data cannot be empty", nameof(data));
            }

            if (string.IsNullOrEmpty(outputPath))
            {
                throw new ArgumentException("Output path cannot be empty", nameof(outputPath));
            }

            var result = quickcodes_generate_to_file(barcodeType.ToString(), data, outputPath);
            if (result == IntPtr.Zero)
            {
                throw new BarcodeException("Failed to generate barcode", -1);
            }

            try
            {
                var quickCodesResult = Marshal.PtrToStructure<QuickCodesResult>(result);
                if (quickCodesResult.Error != IntPtr.Zero)
                {
                    var error = Marshal.PtrToStructure<QuickCodesError>(quickCodesResult.Error);
                    var message = Marshal.PtrToStringAnsi(error.Message);
                    throw new BarcodeException(message ?? "Unknown error", error.Code);
                }
            }
            finally
            {
                quickcodes_free_result(result);
            }
        }

        public static string ReadFromFile(string filePath)
        {
            if (string.IsNullOrEmpty(filePath))
            {
                throw new ArgumentException("File path cannot be empty", nameof(filePath));
            }

            var result = quickcodes_read_from_file(filePath);
            if (result == IntPtr.Zero)
            {
                throw new BarcodeException("Failed to read barcode", -1);
            }

            try
            {
                var quickCodesResult = Marshal.PtrToStructure<QuickCodesResult>(result);
                if (quickCodesResult.Error != IntPtr.Zero)
                {
                    var error = Marshal.PtrToStructure<QuickCodesError>(quickCodesResult.Error);
                    var message = Marshal.PtrToStringAnsi(error.Message);
                    throw new BarcodeException(message ?? "Unknown error", error.Code);
                }

                if (quickCodesResult.Data == IntPtr.Zero)
                {
                    throw new BarcodeException("No data returned", -1);
                }

                return Marshal.PtrToStringAnsi(quickCodesResult.Data) ?? throw new BarcodeException("Failed to decode data", -1);
            }
            finally
            {
                quickcodes_free_result(result);
            }
        }

        public static string ReadFromBytes(byte[] data)
        {
            if (data == null || data.Length == 0)
            {
                throw new ArgumentException("Data cannot be empty", nameof(data));
            }

            var result = quickcodes_read_from_bytes(data, (UIntPtr)data.Length);
            if (result == IntPtr.Zero)
            {
                throw new BarcodeException("Failed to read barcode", -1);
            }

            try
            {
                var quickCodesResult = Marshal.PtrToStructure<QuickCodesResult>(result);
                if (quickCodesResult.Error != IntPtr.Zero)
                {
                    var error = Marshal.PtrToStructure<QuickCodesError>(quickCodesResult.Error);
                    var message = Marshal.PtrToStringAnsi(error.Message);
                    throw new BarcodeException(message ?? "Unknown error", error.Code);
                }

                if (quickCodesResult.Data == IntPtr.Zero)
                {
                    throw new BarcodeException("No data returned", -1);
                }

                return Marshal.PtrToStringAnsi(quickCodesResult.Data) ?? throw new BarcodeException("Failed to decode data", -1);
            }
            finally
            {
                quickcodes_free_result(result);
            }
        }
    }
}