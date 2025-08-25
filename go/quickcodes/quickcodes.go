package quickcodes

/*
#cgo LDFLAGS: -L/usr/local/lib -lquickcodes
#include "quickcodes.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"runtime"
	"unsafe"
)

// BarcodeType represents the supported barcode formats
type BarcodeType string

const (
	QRCode     BarcodeType = "QRCode"
	EAN13      BarcodeType = "EAN13"
	UPCA       BarcodeType = "UPCA"
	Code128    BarcodeType = "Code128"
	DataMatrix BarcodeType = "DataMatrix"
	PDF417     BarcodeType = "PDF417"
	Aztec      BarcodeType = "Aztec"
)

// Generate creates a barcode of the specified type with the given data
func Generate(barcodeType BarcodeType, data string) ([]byte, error) {
	cBarcodeType := C.CString(string(barcodeType))
	defer C.free(unsafe.Pointer(cBarcodeType))

	cData := C.CString(data)
	defer C.free(unsafe.Pointer(cData))

	result := C.quickcodes_generate(cBarcodeType, cData)
	if result == nil {
		return nil, errors.New("failed to generate barcode")
	}
	defer C.quickcodes_free_result(result)

	if result.error != nil {
		errMsg := C.GoString(result.error.message)
		return nil, errors.New(errMsg)
	}

	if result.data == nil {
		return nil, errors.New("no data returned")
	}

	// Copy the data to a Go slice
	output := C.GoBytes(unsafe.Pointer(result.data), C.int(result.len))
	return output, nil
}

// GenerateToFile creates a barcode and saves it directly to a file
func GenerateToFile(barcodeType BarcodeType, data string, outputPath string) error {
	cBarcodeType := C.CString(string(barcodeType))
	defer C.free(unsafe.Pointer(cBarcodeType))

	cData := C.CString(data)
	defer C.free(unsafe.Pointer(cData))

	cOutputPath := C.CString(outputPath)
	defer C.free(unsafe.Pointer(cOutputPath))

	result := C.quickcodes_generate_to_file(cBarcodeType, cData, cOutputPath)
	if result == nil {
		return errors.New("failed to generate barcode")
	}
	defer C.quickcodes_free_result(result)

	if result.error != nil {
		errMsg := C.GoString(result.error.message)
		return errors.New(errMsg)
	}

	return nil
}

// ReadFromFile reads and decodes a barcode from a file
func ReadFromFile(filePath string) (string, error) {
	cFilePath := C.CString(filePath)
	defer C.free(unsafe.Pointer(cFilePath))

	result := C.quickcodes_read_from_file(cFilePath)
	if result == nil {
		return "", errors.New("failed to read barcode")
	}
	defer C.quickcodes_free_result(result)

	if result.error != nil {
		errMsg := C.GoString(result.error.message)
		return "", errors.New(errMsg)
	}

	if result.data == nil {
		return "", errors.New("no data returned")
	}

	// Convert the C string to a Go string
	output := C.GoString((*C.char)(unsafe.Pointer(result.data)))
	return output, nil
}

// ReadFromBytes reads and decodes a barcode from bytes
func ReadFromBytes(data []byte) (string, error) {
	if len(data) == 0 {
		return "", errors.New("empty input data")
	}

	cData := (*C.uchar)(unsafe.Pointer(&data[0]))
	result := C.quickcodes_read_from_bytes(cData, C.size_t(len(data)))
	if result == nil {
		return "", errors.New("failed to read barcode")
	}
	defer C.quickcodes_free_result(result)

	if result.error != nil {
		errMsg := C.GoString(result.error.message)
		return "", errors.New(errMsg)
	}

	if result.data == nil {
		return "", errors.New("no data returned")
	}

	// Convert the C string to a Go string
	output := C.GoString((*C.char)(unsafe.Pointer(result.data)))
	return output, nil
}

func init() {
	// This is needed to keep the Go runtime from collecting our callbacks
	runtime.LockOSThread()
}
