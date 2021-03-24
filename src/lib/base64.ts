import { decode } from "https://deno.land/std@0.91.0/encoding/base64.ts";

const textDecoder = new TextDecoder("utf-8");

export const decodeBase64 = (data: string): string =>
  textDecoder.decode(decode(data));
