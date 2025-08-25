#ifndef QUICKCODES_H
#define QUICKCODES_H

#include <stdint.h>
#include <stddef.h>

// Error handling
typedef struct {
    char* message;
    int code;
} QuickCodesError;

// Result types
typedef struct {
    uint8_t* data;
    size_t len;
    QuickCodesError* error;
} QuickCodesResult;

// Barcode generation functions
QuickCodesResult* quickcodes_generate(const char* barcode_type, const char* data);
QuickCodesResult* quickcodes_generate_to_file(const char* barcode_type, const char* data, const char* output_path);

// Barcode reading functions
QuickCodesResult* quickcodes_read_from_file(const char* file_path);
QuickCodesResult* quickcodes_read_from_bytes(const uint8_t* data, size_t len);

// Memory management
void quickcodes_free_result(QuickCodesResult* result);

#endif // QUICKCODES_H
