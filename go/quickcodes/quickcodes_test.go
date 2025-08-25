package quickcodes

import (
	"os"
	"path/filepath"
	"testing"
)

func TestGenerateQRCode(t *testing.T) {
	data := "Hello, QuickCodes!"
	result, err := Generate(QRCode, data)
	if err != nil {
		t.Fatalf("Failed to generate QR code: %v", err)
	}
	if len(result) == 0 {
		t.Error("Generated QR code is empty")
	}
}

func TestGenerateToFile(t *testing.T) {
	outputDir := "../../examples/output"
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		t.Fatalf("Failed to create output directory: %v", err)
	}

	data := "1234567890128"
	outputPath := filepath.Join(outputDir, "test_go_ean13.png")

	err := GenerateToFile(EAN13, data, outputPath)
	if err != nil {
		t.Fatalf("Failed to generate EAN-13 to file: %v", err)
	}

	if _, err := os.Stat(outputPath); os.IsNotExist(err) {
		t.Error("Output file was not created")
	}
}

func TestReadFromFile(t *testing.T) {
	t.Skip("Reader functionality not yet implemented")

	outputDir := "../../examples/output"
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		t.Fatalf("Failed to create output directory: %v", err)
	}

	data := "Hello, QuickCodes!"
	outputPath := filepath.Join(outputDir, "test_go_qr.png")

	err := GenerateToFile(QRCode, data, outputPath)
	if err != nil {
		t.Fatalf("Failed to generate QR code for reading test: %v", err)
	}

	result, err := ReadFromFile(outputPath)
	if err != nil {
		t.Fatalf("Failed to read QR code from file: %v", err)
	}

	if result != data {
		t.Errorf("Read data does not match original. Got %q, want %q", result, data)
	}
}

func TestReadFromBytes(t *testing.T) {
	t.Skip("Reader functionality not yet implemented")

	data := "Hello, QuickCodes!"
	barcode, err := Generate(QRCode, data)
	if err != nil {
		t.Fatalf("Failed to generate QR code for bytes test: %v", err)
	}

	result, err := ReadFromBytes(barcode)
	if err != nil {
		t.Fatalf("Failed to read QR code from bytes: %v", err)
	}

	if result != data {
		t.Errorf("Read data does not match original. Got %q, want %q", result, data)
	}
}

func TestInvalidInput(t *testing.T) {
	tests := []struct {
		name        string
		barcodeType BarcodeType
		data        string
		wantErr     bool
	}{
		{
			name:        "Empty data",
			barcodeType: QRCode,
			data:        "",
			wantErr:     true,
		},
		{
			name:        "Invalid EAN-13 data",
			barcodeType: EAN13,
			data:        "invalid",
			wantErr:     true,
		},
		{
			name:        "Valid QR Code data",
			barcodeType: QRCode,
			data:        "https://example.com",
			wantErr:     false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := Generate(tt.barcodeType, tt.data)
			if (err != nil) != tt.wantErr {
				t.Errorf("Generate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
