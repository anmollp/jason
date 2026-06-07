# Changelog

## v1.2.0
- Iterator-based lexer architecture using Peekable<Chars<'a>>.
- Byte-position tracking to support efficient string slicing.
- Optimized string parsing with separate fast and slow paths.
- Improved internal handling of escaped JSON strings.
- Refactored lexer from indexed character access to a streaming iterator model.
- Simplified parser initialization by consuming tokens directly from the lexer.
- Reduced lexer memory overhead by eliminating the intermediate character buffer.
- Improved readability and maintainability of lexer internals.

## v1.1.0
- Added pretty JSON serializer
- Added nested formatting support
- Added pretty printer tests

## v1.0.0
- Streaming parser architecture
- Compact JSON serializer
- Round-trip tests
- Full JSON lexer/parser