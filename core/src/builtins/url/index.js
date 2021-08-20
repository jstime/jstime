// @ts-check

// 1. Infrastructure

// 1.2. Parsers https://url.spec.whatwg.org/#parsers

/**
 * https://url.spec.whatwg.org/#eof-code-point
 * The EOF code point is a conceptual code point that signifies the end of a string or code point stream.
 */
const EOF_CODE_POINT = Symbol("eof code point")

/**
 * https://url.spec.whatwg.org/#pointer
 */
class Pointer {
  /**
   * https://url.spec.whatwg.org/#pointer
   * @param {string} input 
   */
  constructor (input) {
    this.input = input;
    this._value = 0;
  }

  /**
   * https://url.spec.whatwg.org/#c
   */
  get c () {
    if (this._value === -1) {
      throw new Error('Cannot use c when pointer points to nowhere');
    } else if (this._value === this.input.length) {
      return EOF_CODE_POINT;
    }
    
    return this.input.codePointAt(this._value);
  }

  /**
   * https://url.spec.whatwg.org/#remaining
   */
  get remaining () {
    if (this.c === EOF_CODE_POINT) {
      throw new Error('Cannot use remaining when c is the EOF code point');
    }

    return this.input.substring(this.c + 1);
  }
}

// 1.3. Percent-encoded bytes https://url.spec.whatwg.org/#percent-encoded-bytes

const utf8Encoder = new TextEncoder();
const utf8Decoder = new TextDecoder('utf-8', { ignoreBOM: true });

/**
 * https://encoding.spec.whatwg.org/#utf-8-encode
 * @param {string} string 
 */
function utf8Encode (string) {
  return utf8Encoder.encode(string);
}

/**
 * https://encoding.spec.whatwg.org/#utf-8-decode-without-bom-or-fail
 * @param {BufferSource} bytes 
 */
function utf8DecodeWithoutBOM(bytes) {
  return utf8Decoder.decode(bytes);
}

/**
 * https://url.spec.whatwg.org/#percent-encode
 * @param {number} byte 
 */
function percentEncode (byte) {
  const hex = byte.toString(16).toUpperCase();
  return `%${hex.length === 1 ? `0${hex}` : hex}`
}

/**
 * 
 * @param {number} byte 
 */
function isASCIIHex (byte) {
  return (byte >= 0x30 && byte <= 0x39)
    || (byte >= 0x41 && byte <= 0x46)
    || (byte >= 0x61 && byte <= 0x66)
}

// https://url.spec.whatwg.org/#percent-decode
/**
 * 
 * @param {string | Uint8Array} input 
 */
function percentDecode (input) {
  if (typeof input === 'string') {
    input = utf8Encode(input);
  }

  const output = new Uint8Array(input.byteLength);
  let i = 0;
  for (let j = 0; j < input.byteLength; j++) {
    const byte = input[j];
    if (byte !== 0x25) {
      output[i++] = byte;
    } else if (byte === 0x25
      && !isASCIIHex(input[j+1])
      && !isASCIIHex(input[j+2])
    ) {
      output[i++] = byte;
    } else {
      output[i++] = parseInt(String.fromCodePoint(input[j + 1], input[j + 2]), 16);
      j += 2;
    }
  }

  return output;
}

/**
 * https://url.spec.whatwg.org/#c0-control-percent-encode-set
 * @param {number} codePoint 
 */
function isC0ControlPercentEncode (codePoint) {
  return (codePoint >= 0x00 && codePoint <= 0x1F ) || codePoint > 0x7E;
}

/**
 * https://url.spec.whatwg.org/#fragment-percent-encode-set
 * @param {number} codePoint 
 */
function isFragmentPercentEncode (codePoint) {
  const set = new Set([0x20, 0x22, 0x3C, 0x3E, 0x60]);
  return isC0ControlPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#query-percent-encode-set
 * @param {number} codePoint 
 */
function isQueryPercentEncode (codePoint) {
  const set = new Set([0x20, 0x22, 0x23, 0x3C, 0x3E]);
  return isC0ControlPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#special-query-percent-encode-set
 * @param {number} codePoint 
 */
function isSpecialQueryPercentEncode (codePoint) {
  return isQueryPercentEncode(codePoint) || codePoint === 0x27;
}

/**
 * https://url.spec.whatwg.org/#path-percent-encode-set
 * @param {number} codePoint 
 */
function isPathPercentEncode (codePoint) {
  const set = new Set([0x3F, 0x60, 0x7B, 0x7D]);
  return isQueryPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#userinfo-percent-encode-set
 * @param {number} codePoint 
 */
function isUserinfoPercentEncode (codePoint) {
  const set = new Set([0x2F, 0x3A, 0x3B, 0x3D, 0x40, 0x5B, 0x5C, 0x5D, 0x5E, 0x7C]);
  return isPathPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#component-percent-encode-set
 * @param {number} codePoint 
 */
function isComponentPercentEncode (codePoint) {
  const set = new Set([0x24, 0x25, 0x26, 0x2B, 0x2C])
  return isUserinfoPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set
 * @param {number} codePoint 
 */
function isApplicationXWWWFormUrlencodedPercentEncode (codePoint) {
  const set = new Set([0x21, 0x27, 0x28, 0x29, 0x7E]);
  return isComponentPercentEncode(codePoint) || set.has(codePoint);
}

/**
 * https://url.spec.whatwg.org/#string-percent-encode-after-encoding
 * @param {string} input 
 * @param {(codePoint: number) => boolean} percentEncodePredicate 
 * @param {boolean} [spaceAsPlus] Default: `false`
 * @returns 
 */
function percentEncodeAfterEncoding (input, percentEncodePredicate, spaceAsPlus = false) {
  let output = '';
  for (let i = 0; i < input.length; i++) {
    const codePoint = input.codePointAt(i);
    if (spaceAsPlus && codePoint === 0x20) {
      output += '+';
    } else {
      const bytes = utf8Encode(input[i]);
      for (let j = 0; j < bytes.byteLength; j++) {
        const byte = bytes[j];
        if (percentEncodePredicate(byte)) {
          output += percentEncode(byte);
        } else {
          output += String.fromCodePoint(byte);
        }
      }
    }
  }

  return output;
}

// https://url.spec.whatwg.org/#utf-8-percent-encode
// https://url.spec.whatwg.org/#string-utf-8-percent-encode
/**
 * 
 * @param {number | string} input 
 * @param {(codePoint: number) => boolean} percentEncodePredicate 
 * @returns 
 */
function utf8PercentEncode (input, percentEncodePredicate) {
  if (typeof input === 'number') {
    input = String.fromCodePoint(input)
  }

  return percentEncodeAfterEncoding(input, percentEncodePredicate)
}