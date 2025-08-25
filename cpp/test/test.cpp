#include <quickcodes/quickcodes.h>
#include <cassert>
#include <iostream>
#include <fstream>
#include <filesystem>
#include <random>

using namespace quickcodes;
namespace fs = std::filesystem;

fs::path get_output_dir() {
    // Começar do diretório atual (build/) e subir dois níveis para chegar na raiz do projeto
    return fs::current_path().parent_path().parent_path() / "examples" / "output";
}

void ensure_output_dir() {
    auto output_dir = get_output_dir();
    std::cout << "Creating output directory: " << output_dir << std::endl;
    fs::create_directories(output_dir);
}

void test_generate_qr_code() {
    std::string data = "Hello, QuickCodes!";
    auto result = generate(BarcodeType::QRCode, data);
    assert(!result.empty());
    std::cout << "Generate QR code test passed\n";
}

void test_generate_to_file() {
    ensure_output_dir();
    std::string data = "1234567890128";
    auto output_path = get_output_dir() / "test_cpp_ean13.png";
    std::cout << "Generating file at: " << output_path << std::endl;

    generate_to_file(BarcodeType::EAN13, data, output_path.string());
    assert(fs::exists(output_path));

    std::cout << "Generate to file test passed\n";
}

void test_read_from_file() {
    std::cout << "Read from file test skipped (Reader functionality not yet implemented)\n";
}

void test_read_from_bytes() {
    std::cout << "Read from bytes test skipped (Reader functionality not yet implemented)\n";
}

void test_empty_data() {
    try {
        generate(BarcodeType::QRCode, "");
        assert(false && "Should have thrown");
    } catch (const QuickCodesException& e) {
        assert(std::string(e.what()).find("empty") != std::string::npos);
    }
    std::cout << "Empty data test passed\n";
}

void test_invalid_ean13_data() {
    try {
        generate(BarcodeType::EAN13, "invalid");
        assert(false && "Should have thrown");
    } catch (const QuickCodesException& e) {
        // The error message should indicate a problem with the data
        std::cout << "Got error message: " << e.what() << '\n';
        assert(e.error_code() != 0);
    }
    std::cout << "Invalid EAN13 data test passed\n";
}

void test_valid_qr_code_data() {
    auto result = generate(BarcodeType::QRCode, "https://example.com");
    assert(!result.empty());
    std::cout << "Valid QR code data test passed\n";
}

int main() {
    try {
        ensure_output_dir();
        test_generate_qr_code();
        test_generate_to_file();
        test_read_from_file();
        test_read_from_bytes();
        test_empty_data();
        test_invalid_ean13_data();
        test_valid_qr_code_data();
        std::cout << "All tests completed!\n";
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Test failed: " << e.what() << '\n';
        return 1;
    }
}