"""
Tests for QuickCodes Python bindings
"""

import pytest
import quickcodes
import os
import tempfile
from pathlib import Path

class TestQuickCodes:
    """Test suite for QuickCodes Python bindings"""
    
    def test_generate_qr_svg(self):
        """Test QR Code generation as SVG"""
        result = quickcodes.generate("QRCode", "test", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert b"<svg" in result
        assert b"</svg>" in result
    
    def test_generate_qr_png(self):
        """Test QR Code generation as PNG"""
        result = quickcodes.generate("QRCode", "test", "PNG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        # PNG magic bytes
        assert result[:8] == b'\x89\x50\x4e\x47\x0d\x0a\x1a\x0a'
    
    def test_generate_ean13_svg(self):
        """Test EAN-13 generation as SVG"""
        result = quickcodes.generate("EAN13", "123456789012", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert b"<svg" in result
    
    def test_generate_ean13_png(self):
        """Test EAN-13 generation as PNG"""
        result = quickcodes.generate("EAN13", "123456789012", "PNG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert result[:8] == b'\x89\x50\x4e\x47\x0d\x0a\x1a\x0a'
    
    def test_generate_upca(self):
        """Test UPC-A generation"""
        result = quickcodes.generate("UPCA", "03600029145", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert b"<svg" in result
    
    def test_generate_code128(self):
        """Test Code128 generation"""
        result = quickcodes.generate("Code128", "HELLO123", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert b"<svg" in result
    
    def test_generate_to_file(self):
        """Test generate_to_file function"""
        with tempfile.TemporaryDirectory() as tmpdir:
            output_path = os.path.join(tmpdir, "test_qr.svg")
            quickcodes.generate_to_file("QRCode", "test", output_path)
            
            assert os.path.exists(output_path)
            with open(output_path, "rb") as f:
                content = f.read()
                assert b"<svg" in content
    
    def test_qr_error_correction_levels(self):
        """Test different QR error correction levels"""
        for level in ["Low", "Medium", "Quartile", "High"]:
            result = quickcodes.generate(
                "QRCode", 
                "test", 
                "SVG", 
                error_correction=level
            )
            assert isinstance(result, bytes)
            assert len(result) > 0
    
    def test_custom_margins(self):
        """Test custom margin settings"""
        result = quickcodes.generate(
            "QRCode", 
            "test", 
            "SVG",
            margin=30
        )
        assert isinstance(result, bytes)
        assert len(result) > 0
    
    def test_invalid_barcode_type(self):
        """Test error handling for invalid barcode type"""
        with pytest.raises(ValueError, match="Unsupported barcode type"):
            quickcodes.generate("InvalidType", "test", "SVG")
    
    def test_invalid_export_format(self):
        """Test error handling for invalid export format"""
        with pytest.raises(ValueError, match="Unsupported export format"):
            quickcodes.generate("QRCode", "test", "InvalidFormat")
    
    def test_invalid_error_correction(self):
        """Test error handling for invalid error correction level"""
        with pytest.raises(ValueError, match="Unknown error correction"):
            quickcodes.generate("QRCode", "test", "SVG", error_correction="Invalid")
    
    def test_ean13_with_check_digit(self):
        """Test EAN-13 with valid check digit"""
        # 1234567890128 has correct check digit
        result = quickcodes.generate("EAN13", "1234567890128", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
    
    def test_empty_data_handling(self):
        """Test handling of empty data"""
        # QR codes can handle empty strings
        result = quickcodes.generate("QRCode", "", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
        
        # Code128 should fail with empty data
        with pytest.raises(ValueError):
            quickcodes.generate("Code128", "", "SVG")

class TestConstants:
    """Test module constants and convenience features"""
    
    def test_constants_exist(self):
        """Test that convenience constants are available"""
        assert hasattr(quickcodes, 'BARCODE_TYPES')
        assert hasattr(quickcodes, 'EXPORT_FORMATS')
        assert hasattr(quickcodes, 'ERROR_CORRECTION_LEVELS')
    
    def test_barcode_types_constants(self):
        """Test barcode type constants"""
        assert quickcodes.BARCODE_TYPES['QR_CODE'] == 'QRCode'
        assert quickcodes.BARCODE_TYPES['EAN13'] == 'EAN13'
        assert quickcodes.BARCODE_TYPES['UPC_A'] == 'UPCA'
        assert quickcodes.BARCODE_TYPES['CODE128'] == 'Code128'
    
    def test_export_formats_constants(self):
        """Test export format constants"""
        assert quickcodes.EXPORT_FORMATS['SVG'] == 'SVG'
        assert quickcodes.EXPORT_FORMATS['PNG'] == 'PNG'
    
    def test_error_correction_constants(self):
        """Test error correction level constants"""
        assert quickcodes.ERROR_CORRECTION_LEVELS['LOW'] == 'Low'
        assert quickcodes.ERROR_CORRECTION_LEVELS['MEDIUM'] == 'Medium'
        assert quickcodes.ERROR_CORRECTION_LEVELS['QUARTILE'] == 'Quartile'
        assert quickcodes.ERROR_CORRECTION_LEVELS['HIGH'] == 'High'

@pytest.mark.benchmark
class TestPerformance:
    """Performance benchmarks for QuickCodes"""
    
    def test_qr_generation_performance(self, benchmark):
        """Benchmark QR code generation"""
        result = benchmark(quickcodes.generate, "QRCode", "Performance test data", "SVG")
        assert isinstance(result, bytes)
        assert len(result) > 0
    
    def test_ean13_generation_performance(self, benchmark):
        """Benchmark EAN-13 generation"""
        result = benchmark(quickcodes.generate, "EAN13", "123456789012", "PNG")
        assert isinstance(result, bytes)
        assert len(result) > 0
