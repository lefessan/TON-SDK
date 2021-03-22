# Module utils

Misc utility Functions.


## Functions
[convert_address](#convert_address) – Converts address from any TON format to any TON format

[compress](#compress) – Compresses data using Zstandard algorithm

[decompress](#decompress) – Decompresses data using Zstandard algorithm

## Types
[AddressStringFormat](#AddressStringFormat)

[ParamsOfConvertAddress](#ParamsOfConvertAddress)

[ResultOfConvertAddress](#ResultOfConvertAddress)

[ParamsOfCompress](#ParamsOfCompress)

[ResultOfCompress](#ResultOfCompress)

[ParamsOfDecompress](#ParamsOfDecompress)

[ResultOfDecompress](#ResultOfDecompress)


# Functions
## convert_address

Converts address from any TON format to any TON format

```ts
type ParamsOfConvertAddress = {
    address: string,
    output_format: AddressStringFormat
}

type ResultOfConvertAddress = {
    address: string
}

function convert_address(
    params: ParamsOfConvertAddress,
): Promise<ResultOfConvertAddress>;
```
### Parameters
- `address`: _string_ – Account address in any TON format.
- `output_format`: _[AddressStringFormat](mod_utils.md#AddressStringFormat)_ – Specify the format to convert to.


### Result

- `address`: _string_ – Address in the specified format


## compress

Compresses data using Zstandard algorithm

```ts
type ParamsOfCompress = {
    uncompressed: string,
    level: number
}

type ResultOfCompress = {
    compressed: string
}

function compress(
    params: ParamsOfCompress,
): Promise<ResultOfCompress>;
```
### Parameters
- `uncompressed`: _string_ – Uncompressed data, encoded in BASE64
- `level`: _number_ – Compression level, from 0 to 21. Where: 0 - default compression level (currently `3`). 1 - lowest compression level (fastest compression); 21 - highest compression level (slowest compression).


### Result

- `compressed`: _string_ – Compressed data, encoded in BASE64


## decompress

Decompresses data using Zstandard algorithm

```ts
type ParamsOfDecompress = {
    compressed: string
}

type ResultOfDecompress = {
    decompressed: string
}

function decompress(
    params: ParamsOfDecompress,
): Promise<ResultOfDecompress>;
```
### Parameters
- `compressed`: _string_ – Compressed data, encoded in BASE64


### Result

- `decompressed`: _string_ – Decompressed data, encoded in BASE64


# Types
## AddressStringFormat
```ts
type AddressStringFormat = {
    type: 'AccountId'
} | {
    type: 'Hex'
} | {
    type: 'Base64'
    url: boolean,
    test: boolean,
    bounce: boolean
}
```
Depends on value of the  `type` field.

When _type_ is _'AccountId'_


When _type_ is _'Hex'_


When _type_ is _'Base64'_


- `url`: _boolean_
- `test`: _boolean_
- `bounce`: _boolean_


Variant constructors:

```ts
function addressStringFormatAccountId(): AddressStringFormat;
function addressStringFormatHex(): AddressStringFormat;
function addressStringFormatBase64(url: boolean, test: boolean, bounce: boolean): AddressStringFormat;
```

## ParamsOfConvertAddress
```ts
type ParamsOfConvertAddress = {
    address: string,
    output_format: AddressStringFormat
}
```
- `address`: _string_ – Account address in any TON format.
- `output_format`: _[AddressStringFormat](mod_utils.md#AddressStringFormat)_ – Specify the format to convert to.


## ResultOfConvertAddress
```ts
type ResultOfConvertAddress = {
    address: string
}
```
- `address`: _string_ – Address in the specified format


## ParamsOfCompress
```ts
type ParamsOfCompress = {
    uncompressed: string,
    level: number
}
```
- `uncompressed`: _string_ – Uncompressed data, encoded in BASE64
- `level`: _number_ – Compression level, from 0 to 21. Where: 0 - default compression level (currently `3`). 1 - lowest compression level (fastest compression); 21 - highest compression level (slowest compression).


## ResultOfCompress
```ts
type ResultOfCompress = {
    compressed: string
}
```
- `compressed`: _string_ – Compressed data, encoded in BASE64


## ParamsOfDecompress
```ts
type ParamsOfDecompress = {
    compressed: string
}
```
- `compressed`: _string_ – Compressed data, encoded in BASE64


## ResultOfDecompress
```ts
type ResultOfDecompress = {
    decompressed: string
}
```
- `decompressed`: _string_ – Decompressed data, encoded in BASE64


