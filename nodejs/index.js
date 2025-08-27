const { 
  generateBarcode,
  generateBarcodeToFile,
  readBarcodeFromFile,
  readAllBarcodesFromFile,
  readBarcodeFromBytes,
  getVersion,
  getSupportedTypes,
  getSupportedFormats 
} = require('./quickcodes.node');

module.exports = {
  generateBarcode,
  generateBarcodeToFile,
  readBarcodeFromFile,
  readAllBarcodesFromFile,
  readBarcodeFromBytes,
  getVersion,
  getSupportedTypes,
  getSupportedFormats
};
