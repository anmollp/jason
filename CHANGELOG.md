# Changelog

## v1.6.0
- Added support for RFC 7396 JSON Merge Patch
- Introduced recursive in-place document mutation
- Implemented full RFC-compliant behavior:
  - If patch is not an object → target is fully replaced 
  - If patch is an object → fields are applied recursively 
  - If a field value is null → the field is removed from the target 
  - If a field exists in both target and patch:
    - Objects → recursively merged 
    - Scalars → replaced  
- Added deep recursive traversal for nested JSON objects 
- Preserves untouched fields while applying partial updates
- Supports mixed-depth object updates without full document replacement

## v1.5.0
- Added recursive JSON diff generation
- Generate patch operations from two JSON documents
- Supports:
  - Object additions
  - Object removals
  - Object replacements
  - Nested object traversal
  - Produces PatchOperation sequences compatible with the existing JSON Patch engine

## v1.4.0
- JSON Patch (RFC 6902-inspired) support on top of the existing JSON parser, serializer, and JSON Pointer system
- `Add` — insert new values into objects and arrays
- `Replace` — update existing values via JSON Pointer paths
- `Remove` — delete values and return removed elements
- `Move` — relocate values within a JSON document
- `Copy` — duplicate values within a JSON document
- `Test` (experimental foundation added / planned depending on your state) — value equality validation via pointer paths

## v1.3.0
- Added JSON Pointer traversal API.
- Support for object and array navigation.
- Support for root document lookup.
- Support for RFC 6901 escape sequences (~0, ~1).
- Added comprehensive pointer test coverage.

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