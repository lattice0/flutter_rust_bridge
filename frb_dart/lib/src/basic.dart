import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_independent.dart';
import 'package:flutter_rust_bridge/src/utils.dart';
import 'package:meta/meta.dart';

final _instances = <Type>{};

/// Base class for generated bindings of Flutter Rust Bridge.
/// Normally, users do not extend this class manually. Instead,
/// users should directly use the generated class.
abstract class FlutterRustBridgeBase<T extends FlutterRustBridgeWireBase> {
  FlutterRustBridgeBase(this.inner) {
    _sanityCheckSingleton();
    _setUpRustToDartComm();
  }

  @protected
  final T inner;

  void _sanityCheckSingleton() {
    if (_instances.contains(runtimeType)) {
      throw Exception(
          'Subclasses of `FlutterRustBridgeBase` should be singletons - there should not be two instances (runtimeType=$runtimeType)');
    }
    _instances.add(runtimeType);
  }

  void _setUpRustToDartComm() {
    inner.store_dart_post_cobject(NativeApi.postCObject.cast());
  }

  /// Execute a normal ffi call. Usually called by generated code instead of manually called.
  @protected
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> task) {
    final completer = Completer<dynamic>();
    final sendPort = singleCompletePort(completer);
    task.callFfi(sendPort.nativePort);
    return completer.future.then((dynamic raw) =>
        _transformRust2DartMessage(raw, task.parseSuccessData));
  }

  /// Similar to [executeNormal], except that this will return synchronously
  @protected
  Uint8List executeSync(FlutterRustBridgeSyncTask task) {
    final raw = task.callFfi();

    final bytes = Uint8List.fromList(raw.ptr.asTypedList(raw.len));
    final success = raw.success > 0;

    inner.free_WireSyncReturnStruct(raw);

    if (success) {
      return bytes;
    } else {
      throw FfiException('EXECUTE_SYNC', utf8.decode(bytes), null);
    }
  }

  /// Similar to [executeNormal], except that this will return a [Stream] instead of a [Future].
  @protected
  Stream<S> executeStream<S>(FlutterRustBridgeTask<S> task) {
    final receivePort = ReceivePort();
    task.callFfi(receivePort.sendPort.nativePort);

    return () async* {
      await for (final raw in receivePort) {
        try {
          yield _transformRust2DartMessage(raw, task.parseSuccessData);
        } on _CloseStreamException {
          receivePort.close();
        }
      }
    }();
  }

  S _transformRust2DartMessage<S>(
      dynamic raw, S Function(dynamic) parseSuccessData) {
    final action = raw[0];
    switch (action) {
      case _RUST2DART_ACTION_SUCCESS:
        assert(raw.length == 2);
        return parseSuccessData(raw[1]);
      case _RUST2DART_ACTION_ERROR:
        assert(raw.length == 4);
        throw FfiException(raw[1], raw[2], raw[3]);
      case _RUST2DART_ACTION_CLOSE_STREAM:
        assert(raw.length == 1);
        throw _CloseStreamException();
      default:
        throw Exception('Unsupported message, action=$action raw=$raw');
    }
  }

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_SUCCESS = 0;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_ERROR = 1;

  // ignore: constant_identifier_names
  static const _RUST2DART_ACTION_CLOSE_STREAM = 2;
}

/// A task to call FFI function.
@immutable
class FlutterRustBridgeTask<S> extends FlutterRustBridgeBaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final void Function(int port) callFfi;

  /// Parse the returned data from the underlying function
  final S Function(dynamic) parseSuccessData;

  const FlutterRustBridgeTask({
    required this.callFfi,
    required this.parseSuccessData,
    required FlutterRustBridgeTaskConstMeta constMeta,
    required List<dynamic> argValues,
    required dynamic hint,
  }) : super(
          constMeta: constMeta,
          argValues: argValues,
          hint: hint,
        );
}

/// A task to call FFI function, but it is synchronous.
@immutable
class FlutterRustBridgeSyncTask extends FlutterRustBridgeBaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final WireSyncReturnStruct Function() callFfi;

  const FlutterRustBridgeSyncTask({
    required this.callFfi,
    required FlutterRustBridgeTaskConstMeta constMeta,
    required List<dynamic> argValues,
    required dynamic hint,
  }) : super(
          constMeta: constMeta,
          argValues: argValues,
          hint: hint,
        );
}

/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(
    ffi.Pointer<
            ffi.NativeFunction<
                ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>
        ptr,
  );

  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct val);
}

class _CloseStreamException {}

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external ffi.Pointer<ffi.Uint8> ptr;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Int32()
  external int len;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Uint8()
  external int success;
}

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at some point during runtime.
class FrbOpaque {
  ffi.Pointer? _ptr;
  final ffi.Pointer<ffi.NativeFunction<ffi.Void Function(ffi.Pointer)>> _drop;
  final ffi.Pointer<ffi.NativeFunction<ffi.Pointer Function(ffi.Pointer)>> _lend;

  /// This constructor should never be called manually.
  FrbOpaque.unsafe(int? ptr, int drop, int lend)
      : assert(ptr == null || ptr > 0),
        assert(drop > 0),
        assert(lend > 0),
        _ptr = ptr == null ? null : Pointer.fromAddress(ptr),
        _drop = Pointer.fromAddress(drop),
        _lend = Pointer.fromAddress(lend);

  /// Call Rust destructors on the backing memory of this pointer.
  /// This function should be run at least once during the lifetime of the program, and can be run many times.
  ///
  /// When passed into a Rust function, Rust enacts *shared ownership* and inhibits disposal of this pointer's contents, even if [dispose] is immediately run.
  /// Furthermore, if that same function reuses the allocation (usually by returning the same opaque pointer) ownership of this pointer will be moved into that new opaque pointer.
  void dispose() {
    if (!isStale()) {
      _drop.asFunction<void Function(ffi.Pointer)>()(_ptr!);
      _ptr = null;
    }
  }

  static ffi.Pointer lend(FrbOpaque ptr) {
    if (!ptr.isStale()) {
      return ptr._lend.asFunction<ffi.Pointer Function(ffi.Pointer)>()(ptr._ptr!);
    } else {
      // next best thing here, this is equivalent to an Option::<Arc<T>>::None
      return ffi.nullptr;
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has actually
  /// been reclaimed.
  // not nullptr, this is an internal bookkeeping method
  bool isStale() => _ptr == null;
}
