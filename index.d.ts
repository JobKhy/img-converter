/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

type format = "png" | "jpg" | "jpeg" | "bmp" | "gif" | "tiff" | "webp";

export declare function convertImage(
  inputPath: string,
  outputPath: string,
  format: format
): void;
export declare function convertImageFile(
  inputData: Buffer,
  format: format
): Buffer;
