#cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/with_flutter/rust/src/api.rs --dart-output frb_example/with_flutter/lib/bridge_generated.dart
#cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input something/src/api.rs --dart-output something/bridge_generated.dart

cargo run --manifest-path=frb_codegen/Cargo.toml -- --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart
