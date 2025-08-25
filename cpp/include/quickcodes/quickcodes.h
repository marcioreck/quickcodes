#ifndef QUICKCODES_H
#define QUICKCODES_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Error information returned by QuickCodes functions
 */
typedef struct {
    char* message;  /**< Error message (owned by the library) */
    int code;       /**< Error code */
} QuickCodesError;

/**
 * @brief Result structure returned by QuickCodes functions
 */
typedef struct {
    uint8_t* data;  /**< Output data (owned by the library) */
    size_t len;     /**< Length of the output data */
    QuickCodesError* error;  /**< Error information (NULL if no error) */
} QuickCodesResult;

/**
 * @brief Generate a barcode
 * 
 * @param barcode_type Type of barcode to generate (e.g., "QRCode", "EAN13")
 * @param data Data to encode in the barcode
 * @return QuickCodesResult* Result containing the generated barcode data or error
 */
QuickCodesResult* quickcodes_generate(const char* barcode_type, const char* data);

/**
 * @brief Generate a barcode and save it to a file
 * 
 * @param barcode_type Type of barcode to generate (e.g., "QRCode", "EAN13")
 * @param data Data to encode in the barcode
 * @param output_path Path where to save the generated barcode
 * @return QuickCodesResult* Result containing success/error information
 */
QuickCodesResult* quickcodes_generate_to_file(const char* barcode_type, const char* data, const char* output_path);

/**
 * @brief Read a barcode from a file
 * 
 * @param file_path Path to the image file containing the barcode
 * @return QuickCodesResult* Result containing the decoded data or error
 */
QuickCodesResult* quickcodes_read_from_file(const char* file_path);

/**
 * @brief Read a barcode from raw bytes
 * 
 * @param data Raw image data
 * @param len Length of the image data
 * @return QuickCodesResult* Result containing the decoded data or error
 */
QuickCodesResult* quickcodes_read_from_bytes(const uint8_t* data, size_t len);

/**
 * @brief Free a result structure
 * 
 * @param result Result structure to free
 */
void quickcodes_free_result(QuickCodesResult* result);

#ifdef __cplusplus
}  // extern "C"

#include <string>
#include <vector>
#include <stdexcept>
#include <memory>

namespace quickcodes {

/**
 * @brief Exception thrown by QuickCodes operations
 */
class QuickCodesException : public std::runtime_error {
public:
    explicit QuickCodesException(const std::string& message, int code = -1)
        : std::runtime_error(message), error_code_(code) {}

    int error_code() const noexcept { return error_code_; }

private:
    int error_code_;
};

/**
 * @brief Supported barcode types
 */
enum class BarcodeType {
    QRCode,
    EAN13,
    UPCA,
    Code128,
    DataMatrix,
    PDF417,
    Aztec
};

/**
 * @brief Convert BarcodeType to string
 */
inline const char* to_string(BarcodeType type) {
    switch (type) {
        case BarcodeType::QRCode: return "QRCode";
        case BarcodeType::EAN13: return "EAN13";
        case BarcodeType::UPCA: return "UPCA";
        case BarcodeType::Code128: return "Code128";
        case BarcodeType::DataMatrix: return "DataMatrix";
        case BarcodeType::PDF417: return "PDF417";
        case BarcodeType::Aztec: return "Aztec";
        default: throw QuickCodesException("Invalid barcode type");
    }
}

/**
 * @brief RAII wrapper for QuickCodesResult
 */
class Result {
public:
    explicit Result(QuickCodesResult* result) : result_(result) {
        if (!result) {
            throw QuickCodesException("Failed to create result");
        }
        if (result->error) {
            auto message = result->error->message ? result->error->message : "Unknown error";
            throw QuickCodesException(message, result->error->code);
        }
    }

    ~Result() {
        if (result_) {
            quickcodes_free_result(result_);
        }
    }

    Result(const Result&) = delete;
    Result& operator=(const Result&) = delete;

    Result(Result&& other) noexcept : result_(other.result_) {
        other.result_ = nullptr;
    }

    Result& operator=(Result&& other) noexcept {
        if (this != &other) {
            if (result_) {
                quickcodes_free_result(result_);
            }
            result_ = other.result_;
            other.result_ = nullptr;
        }
        return *this;
    }

    const uint8_t* data() const { return result_->data; }
    size_t size() const { return result_->len; }

    std::vector<uint8_t> to_vector() const {
        return std::vector<uint8_t>(data(), data() + size());
    }

    std::string to_string() const {
        return std::string(reinterpret_cast<const char*>(data()));
    }

private:
    QuickCodesResult* result_;
};

/**
 * @brief Generate a barcode
 */
inline std::vector<uint8_t> generate(BarcodeType type, const std::string& data) {
    return Result(quickcodes_generate(to_string(type), data.c_str())).to_vector();
}

/**
 * @brief Generate a barcode and save it to a file
 */
inline void generate_to_file(BarcodeType type, const std::string& data, const std::string& output_path) {
    Result(quickcodes_generate_to_file(to_string(type), data.c_str(), output_path.c_str()));
}

/**
 * @brief Read a barcode from a file
 */
inline std::string read_from_file(const std::string& file_path) {
    return Result(quickcodes_read_from_file(file_path.c_str())).to_string();
}

/**
 * @brief Read a barcode from raw bytes
 */
inline std::string read_from_bytes(const std::vector<uint8_t>& data) {
    return Result(quickcodes_read_from_bytes(data.data(), data.size())).to_string();
}

}  // namespace quickcodes
#endif  // __cplusplus

#endif  // QUICKCODES_H
