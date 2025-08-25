using System;
using System.IO;
using Xunit;

namespace QuickCodes.Tests
{
    public class BarcodeGeneratorTests : IDisposable
    {
        private readonly string _outputDir;

        public BarcodeGeneratorTests()
        {
            _outputDir = Path.GetFullPath(Path.Combine(
                AppDomain.CurrentDomain.BaseDirectory,
                "..", "..", "..", "..", "..",
                "examples", "output"
            ));
            Directory.CreateDirectory(_outputDir);
        }

        public void Dispose()
        {
            // Não vamos mais limpar os arquivos de teste para que possam ser inspecionados
        }

        [Fact]
        public void GenerateQRCode_ShouldReturnValidData()
        {
            var data = "Hello, QuickCodes!";
            var result = BarcodeGenerator.Generate(BarcodeType.QRCode, data);

            Assert.NotNull(result);
            Assert.True(result.Length > 0);
        }

        [Fact]
        public void GenerateToFile_ShouldCreateFile()
        {
            var data = "1234567890128";
            var outputPath = Path.Combine(_outputDir, "test_dotnet_ean13.png");

            BarcodeGenerator.GenerateToFile(BarcodeType.EAN13, data, outputPath);

            Assert.True(File.Exists(outputPath));
        }

        [Fact(Skip = "Reader functionality not yet implemented")]
        public void ReadFromFile_ShouldReturnCorrectData()
        {
            var data = "Hello, QuickCodes!";
            var outputPath = Path.Combine(_outputDir, "test_dotnet_qr.png");

            BarcodeGenerator.GenerateToFile(BarcodeType.QRCode, data, outputPath);
            var result = BarcodeGenerator.ReadFromFile(outputPath);

            Assert.Equal(data, result);
        }

        [Fact(Skip = "Reader functionality not yet implemented")]
        public void ReadFromBytes_ShouldReturnCorrectData()
        {
            var data = "Hello, QuickCodes!";
            var barcode = BarcodeGenerator.Generate(BarcodeType.QRCode, data);
            var result = BarcodeGenerator.ReadFromBytes(barcode);

            Assert.Equal(data, result);
        }

        [Theory]
        [InlineData("")]
        [InlineData(null)]
        public void Generate_WithEmptyData_ShouldThrow(string data)
        {
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.Generate(BarcodeType.QRCode, data));
        }

        [Fact]
        public void Generate_WithInvalidEAN13Data_ShouldThrow()
        {
            Assert.Throws<BarcodeException>(() => BarcodeGenerator.Generate(BarcodeType.EAN13, "invalid"));
        }

        [Fact]
        public void Generate_WithValidQRCodeData_ShouldSucceed()
        {
            var result = BarcodeGenerator.Generate(BarcodeType.QRCode, "https://example.com");
            Assert.NotNull(result);
            Assert.True(result.Length > 0);
        }

        [Theory]
        [InlineData("")]
        [InlineData(null)]
        public void GenerateToFile_WithEmptyData_ShouldThrow(string data)
        {
            var outputPath = Path.Combine(_outputDir, "test_dotnet_error.png");
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.GenerateToFile(BarcodeType.QRCode, data, outputPath));
        }

        [Theory]
        [InlineData("")]
        [InlineData(null)]
        public void GenerateToFile_WithEmptyPath_ShouldThrow(string path)
        {
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.GenerateToFile(BarcodeType.QRCode, "test", path));
        }

        [Theory]
        [InlineData("")]
        [InlineData(null)]
        public void ReadFromFile_WithEmptyPath_ShouldThrow(string path)
        {
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.ReadFromFile(path));
        }

        [Fact]
        public void ReadFromBytes_WithNullData_ShouldThrow()
        {
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.ReadFromBytes(null!));
        }

        [Fact]
        public void ReadFromBytes_WithEmptyData_ShouldThrow()
        {
            Assert.Throws<ArgumentException>(() => BarcodeGenerator.ReadFromBytes(Array.Empty<byte>()));
        }
    }
}