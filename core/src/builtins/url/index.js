// @ts-check

// 1. Infrastructure

// 1.2. Parsers https://url.spec.whatwg.org/#parsers

/**
 * https://url.spec.whatwg.org/#eof-code-point
 * The EOF code point is a conceptual code point that signifies the end of a string or code point stream.
 */
const EOF_CODE_POINT = Symbol('eof code point')

/**
 * https://url.spec.whatwg.org/#pointer
 */
class Pointer {
  /**
   * https://url.spec.whatwg.org/#pointer
   * @param {string} input
   */
  constructor(input) {
    this.input = input
    this._value = 0
  }

  /**
   * https://url.spec.whatwg.org/#c
   */
  get c() {
    if (this._value === -1) {
      throw new Error('Cannot use c when pointer points to nowhere')
    } else if (this._value === this.input.length) {
      return EOF_CODE_POINT
    }

    return this.input.codePointAt(this._value)
  }

  /**
   * https://url.spec.whatwg.org/#remaining
   */
  get remaining() {
    if (this.c === EOF_CODE_POINT) {
      throw new Error('Cannot use remaining when c is the EOF code point')
    }

    return this.input.substring(this.c + 1)
  }
}

// 1.3. Percent-encoded bytes https://url.spec.whatwg.org/#percent-encoded-bytes

const utf8Encoder = new TextEncoder()
const utf8Decoder = new TextDecoder('utf-8', { ignoreBOM: true })

/**
 * https://encoding.spec.whatwg.org/#utf-8-encode
 * @param {string} string
 */
function utf8Encode(string) {
  return utf8Encoder.encode(string)
}

/**
 * https://encoding.spec.whatwg.org/#utf-8-decode-without-bom-or-fail
 * @param {BufferSource} bytes
 */
function utf8DecodeWithoutBOM(bytes) {
  return utf8Decoder.decode(bytes)
}

/**
 * https://url.spec.whatwg.org/#percent-encode
 * @param {number} byte
 */
function percentEncode(byte) {
  const hex = byte.toString(16).toUpperCase()
  return `%${hex.length === 1 ? `0${hex}` : hex}`
}

/**
 * @param {number} byte
 */
function isASCIIHex(byte) {
  return (
    (byte >= 0x30 && byte <= 0x39) ||
    (byte >= 0x41 && byte <= 0x46) ||
    (byte >= 0x61 && byte <= 0x66)
  )
}

/**
 * https://url.spec.whatwg.org/#percent-decode
 * @param {string | Uint8Array} input
 */
function percentDecode(input) {
  if (typeof input === 'string') {
    input = utf8Encode(input)
  }

  const output = new Uint8Array(input.byteLength)
  let i = 0
  for (let j = 0; j < input.byteLength; j++) {
    const byte = input[j]
    if (byte !== 0x25) {
      output[i++] = byte
    } else if (
      byte === 0x25 &&
      !isASCIIHex(input[j + 1]) &&
      !isASCIIHex(input[j + 2])
    ) {
      output[i++] = byte
    } else {
      output[i++] = parseInt(
        String.fromCodePoint(input[j + 1], input[j + 2]),
        16
      )
      j += 2
    }
  }

  return output
}

/**
 * https://url.spec.whatwg.org/#c0-control-percent-encode-set
 * @param {number} codePoint
 */
function isC0ControlPercentEncode(codePoint) {
  return (codePoint >= 0x00 && codePoint <= 0x1f) || codePoint > 0x7e
}

/**
 * https://url.spec.whatwg.org/#fragment-percent-encode-set
 * @param {number} codePoint
 */
function isFragmentPercentEncode(codePoint) {
  const set = new Set([0x20, 0x22, 0x3c, 0x3e, 0x60])
  return isC0ControlPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#query-percent-encode-set
 * @param {number} codePoint
 */
function isQueryPercentEncode(codePoint) {
  const set = new Set([0x20, 0x22, 0x23, 0x3c, 0x3e])
  return isC0ControlPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#special-query-percent-encode-set
 * @param {number} codePoint
 */
function isSpecialQueryPercentEncode(codePoint) {
  return isQueryPercentEncode(codePoint) || codePoint === 0x27
}

/**
 * https://url.spec.whatwg.org/#path-percent-encode-set
 * @param {number} codePoint
 */
function isPathPercentEncode(codePoint) {
  const set = new Set([0x3f, 0x60, 0x7b, 0x7d])
  return isQueryPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#userinfo-percent-encode-set
 * @param {number} codePoint
 */
function isUserinfoPercentEncode(codePoint) {
  const set = new Set([
    0x2f, 0x3a, 0x3b, 0x3d, 0x40, 0x5b, 0x5c, 0x5d, 0x5e, 0x7c,
  ])
  return isPathPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#component-percent-encode-set
 * @param {number} codePoint
 */
function isComponentPercentEncode(codePoint) {
  const set = new Set([0x24, 0x25, 0x26, 0x2b, 0x2c])
  return isUserinfoPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set
 * @param {number} codePoint
 */
function isApplicationXWWWFormUrlencodedPercentEncode(codePoint) {
  const set = new Set([0x21, 0x27, 0x28, 0x29, 0x7e])
  return isComponentPercentEncode(codePoint) || set.has(codePoint)
}

/**
 * https://url.spec.whatwg.org/#string-percent-encode-after-encoding
 * @param {string} input
 * @param {(codePoint: number) => boolean} percentEncodePredicate
 * @param {boolean} [spaceAsPlus] Default: `false`
 * @returns
 */
function percentEncodeAfterEncoding(
  input,
  percentEncodePredicate,
  spaceAsPlus = false
) {
  let output = ''
  for (let i = 0; i < input.length; i++) {
    const codePoint = input.codePointAt(i)
    if (spaceAsPlus && codePoint === 0x20) {
      output += '+'
    } else {
      const bytes = utf8Encode(input[i])
      for (let j = 0; j < bytes.byteLength; j++) {
        const byte = bytes[j]
        if (percentEncodePredicate(byte)) {
          output += percentEncode(byte)
        } else {
          output += String.fromCodePoint(byte)
        }
      }
    }
  }

  return output
}

/**
 * https://url.spec.whatwg.org/#utf-8-percent-encode
 * https://url.spec.whatwg.org/#string-utf-8-percent-encode
 * @param {number | string} input
 * @param {(codePoint: number) => boolean} percentEncodePredicate
 * @returns
 */
function utf8PercentEncode(input, percentEncodePredicate) {
  if (typeof input === 'number') {
    input = String.fromCodePoint(input)
  }

  return percentEncodeAfterEncoding(input, percentEncodePredicate)
}

// 4. URLs https://url.spec.whatwg.org/#urls

/**
 * 4.1 URL representation https://url.spec.whatwg.org/#url-representation
 */
class URLRecord {
  constructor() {
    /** @type {string} */
    this.scheme = ''
    /** @type {string} */
    this.username = ''
    /** @type {string} */
    this.password = ''
    /** @type {null | string | number} */
    this.host = null
    /** @type {null | number} */
    this.port = null
    /** @type {string[]} */
    this.path = []
    /** @type {null | string} */
    this.query = null
    /** @type {null | string} */
    this.fragment = null
    /** @type {boolean} */
    this.cannotBeABaseURL = false
    /** @type {null | unknown} */
    this.blobURLEntry = null
  }
}

// 4.2 URL miscellaneous https://url.spec.whatwg.org/#url-miscellaneous

const specialSchemePortMap = new Map([
  ['ftp', 21],
  ['file', null],
  ['http', 80],
  ['https', 443],
  ['ws', 80],
  ['wss', 443],
])

/**
 * https://url.spec.whatwg.org/#is-special
 * @param {URLRecord} record
 * @returns
 */
function isSpecial(record) {
  return specialSchemePortMap.has(record.scheme)
}

/**
 * https://url.spec.whatwg.org/#include-credentials
 * @param {URLRecord} record
 * @returns
 */
function includesCredentials(record) {
  return record.username.length > 0 || record.password.length > 0
}

/**
 * https://url.spec.whatwg.org/#cannot-have-a-username-password-port
 * @param {URLRecord} record
 * @returns
 */
function cannotHaveUsernamePasswordPort(record) {
  return (
    record.host === null ||
    record.host === '' ||
    record.cannotBeABaseURL ||
    record.scheme === 'file'
  )
}

/**
 *
 * @param {number} codePoint
 * @returns
 */
function isASCIIAlpha(codePoint) {
  return (
    (codePoint >= 0x41 && codePoint <= 0x5a) ||
    codePoint >= 0x61 ||
    codePoint <= 0x7a
  )
}

/**
 * https://url.spec.whatwg.org/#windows-drive-letter
 * @param {Uint8Array} input
 * @returns
 */
function isWindowsDriveLetter(input) {
  return (
    input.byteLength === 2 &&
    isASCIIAlpha(input[0]) &&
    (input[1] === 0x3a || input[1] === 0x7c)
  )
}

/**
 * https://url.spec.whatwg.org/#normalized-windows-drive-letter
 * @param {Uint8Array} input
 * @returns
 */
function isNormalizedWindowsDriveLetter(input) {
  return isWindowsDriveLetter(input) && input[1] === 0x3a
}

/**
 *
 * @param {string} input minimum length of 2
 * @returns
 */
function getFirstTwoCodePoints(input) {
  return new Uint8Array([input.codePointAt(0), input.codePointAt(1)])
}

/**
 * https://url.spec.whatwg.org/#start-with-a-windows-drive-letter
 * @param {string} input
 */
function startsWithWindowsDriveLetter(input) {
  if (input.length < 2) {
    return false
  }

  if (!isWindowsDriveLetter(getFirstTwoCodePoints(input))) {
    return false
  }

  const set = new Set([0x2f, 0x5c, 0x3f, 0x23])
  if (input.length !== 2 && !set.has(input.codePointAt(2))) {
    return false
  }

  return true
}

/**
 *
 * @param {URLRecord} record
 */
function shortenURLPath(record) {
  const { scheme, path } = record
  if (
    scheme === 'file' &&
    path.length === 1 &&
    isNormalizedWindowsDriveLetter(getFirstTwoCodePoints(path[0]))
  ) {
    return
  }
  path.pop()
}

// 5. application/x-www-form-urlencoded

/**
 * https://url.spec.whatwg.org/#concept-urlencoded-parser
 * @param {string | Uint8Array} input
 */
function parseApplicationXWWWFormUrlencoded(input) {
  if (typeof input === 'string') {
    input = utf8Encode(input)
  }

  const sequences = []
  let head = 0
  let tail = 0
  while (head !== input.byteLength) {
    if (input[head] === 0x26) {
      sequences.push(input.slice(tail, head))
      tail = head + 1
    }
    if (head === input.byteLength - 1) {
      sequences.push(input.slice(tail, head + 1))
    }
    head++
  }

  const output = []

  for (let i = 0; i < sequences.length; i++) {
    const bytes = sequences[i]
    if (bytes.byteLength === 0) continue

    const equalsIndex = bytes.indexOf(0x3d)
    const [name, value] =
      equalsIndex !== -1
        ? [
            bytes.slice(0, equalsIndex),
            bytes.slice(equalsIndex + 1, bytes.byteLength),
          ]
        : [bytes, new Uint8Array(0)]

    /**
     * @param {Uint8Array} sequence
     */
    const replacePlusWithSpace = (sequence) => {
      for (let i = 0; i < sequence.byteLength; i++) {
        if (sequence[i] === 0x2b) {
          sequence[i] = 0x20
        }
      }
    }

    replacePlusWithSpace(name)
    replacePlusWithSpace(value)

    output.push([
      utf8DecodeWithoutBOM(percentDecode(name)),
      utf8DecodeWithoutBOM(percentDecode(value)),
    ])
  }

  return output
}

/**
 * https://url.spec.whatwg.org/#urlencoded-serializing
 * @param {string[][]} tuples
 * @returns
 */
function serializeApplicationXWWWFormUrlencoded(tuples) {
  let output = ''

  for (let i = 0; i < tuples.length; i++) {
    const [_name, _value] = tuples[i]
    const name = percentEncodeAfterEncoding(
      _name,
      isApplicationXWWWFormUrlencodedPercentEncode,
      true
    )
    const value = percentEncodeAfterEncoding(
      _value,
      isApplicationXWWWFormUrlencodedPercentEncode,
      true
    )
    if (output !== '') {
      output += '&'
    }
    output += `${name}=${value}`
  }

  return output
}

