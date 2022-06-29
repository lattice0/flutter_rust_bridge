// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class FlutterRustBridgeExample {
  Future<Uint8List> drawMandelbrot(
      {required Size imageSize,
      required Point zoomPoint,
      required double scale,
      required int numThreads,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDrawMandelbrotConstMeta;

  Future<String> passingComplexStructs({required TreeNode root, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kPassingComplexStructsConstMeta;

  Future<BoxedPoint> returningStructsWithBoxedFields({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kReturningStructsWithBoxedFieldsConstMeta;

  Future<int> offTopicMemoryTestInputArray({required Uint8List input, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputArrayConstMeta;

  Future<Uint8List> offTopicMemoryTestOutputZeroCopyBuffer({required int len, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputZeroCopyBufferConstMeta;

  Future<Uint8List> offTopicMemoryTestOutputVecU8({required int len, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputVecU8ConstMeta;

  Future<int> offTopicMemoryTestInputVecOfObject({required List<Size> input, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputVecOfObjectConstMeta;

  Future<List<Size>> offTopicMemoryTestOutputVecOfObject({required int len, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputVecOfObjectConstMeta;

  Future<int> offTopicMemoryTestInputComplexStruct({required TreeNode input, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputComplexStructConstMeta;

  Future<TreeNode> offTopicMemoryTestOutputComplexStruct({required int len, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputComplexStructConstMeta;

  Future<int> offTopicDeliberatelyReturnError({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicDeliberatelyReturnErrorConstMeta;

  Future<int> offTopicDeliberatelyPanic({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOffTopicDeliberatelyPanicConstMeta;
}

class BoxedPoint {
  final Point point;

  BoxedPoint({
    required this.point,
  });
}

class Point {
  final double x;
  final double y;

  Point({
    required this.x,
    required this.y,
  });
}

class Size {
  final int width;
  final int height;

  Size({
    required this.width,
    required this.height,
  });
}

class TreeNode {
  final String name;
  final List<TreeNode> children;

  TreeNode({
    required this.name,
    required this.children,
  });
}

class FlutterRustBridgeExampleImpl extends FlutterRustBridgeBase<FlutterRustBridgeExampleWire>
    implements FlutterRustBridgeExample {
  factory FlutterRustBridgeExampleImpl(ffi.DynamicLibrary dylib) =>
      FlutterRustBridgeExampleImpl.raw(FlutterRustBridgeExampleWire(dylib));

  FlutterRustBridgeExampleImpl.raw(FlutterRustBridgeExampleWire inner) : super(inner);

  Future<Uint8List> drawMandelbrot(
          {required Size imageSize,
          required Point zoomPoint,
          required double scale,
          required int numThreads,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_draw_mandelbrot(port_, _api2wire_box_autoadd_size(imageSize),
            _api2wire_box_autoadd_point(zoomPoint), _api2wire_f64(scale), _api2wire_i32(numThreads)),
        parseSuccessData: _wire2api_ZeroCopyBuffer_Uint8List,
        constMeta: kDrawMandelbrotConstMeta,
        argValues: [imageSize, zoomPoint, scale, numThreads],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kDrawMandelbrotConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "draw_mandelbrot",
        argNames: ["imageSize", "zoomPoint", "scale", "numThreads"],
      );

  Future<String> passingComplexStructs({required TreeNode root, dynamic hint}) => executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_passing_complex_structs(port_, _api2wire_box_autoadd_tree_node(root)),
        parseSuccessData: _wire2api_String,
        constMeta: kPassingComplexStructsConstMeta,
        argValues: [root],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kPassingComplexStructsConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "passing_complex_structs",
        argNames: ["root"],
      );

  Future<BoxedPoint> returningStructsWithBoxedFields({dynamic hint}) => executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_returning_structs_with_boxed_fields(port_),
        parseSuccessData: _wire2api_boxed_point,
        constMeta: kReturningStructsWithBoxedFieldsConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kReturningStructsWithBoxedFieldsConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "returning_structs_with_boxed_fields",
        argNames: [],
      );

  Future<int> offTopicMemoryTestInputArray({required Uint8List input, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_input_array(port_, _api2wire_uint_8_list(input)),
        parseSuccessData: _wire2api_i32,
        constMeta: kOffTopicMemoryTestInputArrayConstMeta,
        argValues: [input],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputArrayConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_input_array",
        argNames: ["input"],
      );

  Future<Uint8List> offTopicMemoryTestOutputZeroCopyBuffer({required int len, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_output_zero_copy_buffer(port_, _api2wire_i32(len)),
        parseSuccessData: _wire2api_ZeroCopyBuffer_Uint8List,
        constMeta: kOffTopicMemoryTestOutputZeroCopyBufferConstMeta,
        argValues: [len],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputZeroCopyBufferConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_output_zero_copy_buffer",
        argNames: ["len"],
      );

  Future<Uint8List> offTopicMemoryTestOutputVecU8({required int len, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_output_vec_u8(port_, _api2wire_i32(len)),
        parseSuccessData: _wire2api_uint_8_list,
        constMeta: kOffTopicMemoryTestOutputVecU8ConstMeta,
        argValues: [len],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputVecU8ConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_output_vec_u8",
        argNames: ["len"],
      );

  Future<int> offTopicMemoryTestInputVecOfObject({required List<Size> input, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_input_vec_of_object(port_, _api2wire_list_size(input)),
        parseSuccessData: _wire2api_i32,
        constMeta: kOffTopicMemoryTestInputVecOfObjectConstMeta,
        argValues: [input],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputVecOfObjectConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_input_vec_of_object",
        argNames: ["input"],
      );

  Future<List<Size>> offTopicMemoryTestOutputVecOfObject({required int len, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_output_vec_of_object(port_, _api2wire_i32(len)),
        parseSuccessData: _wire2api_list_size,
        constMeta: kOffTopicMemoryTestOutputVecOfObjectConstMeta,
        argValues: [len],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputVecOfObjectConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_output_vec_of_object",
        argNames: ["len"],
      );

  Future<int> offTopicMemoryTestInputComplexStruct({required TreeNode input, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_off_topic_memory_test_input_complex_struct(port_, _api2wire_box_autoadd_tree_node(input)),
        parseSuccessData: _wire2api_i32,
        constMeta: kOffTopicMemoryTestInputComplexStructConstMeta,
        argValues: [input],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestInputComplexStructConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_input_complex_struct",
        argNames: ["input"],
      );

  Future<TreeNode> offTopicMemoryTestOutputComplexStruct({required int len, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_memory_test_output_complex_struct(port_, _api2wire_i32(len)),
        parseSuccessData: _wire2api_tree_node,
        constMeta: kOffTopicMemoryTestOutputComplexStructConstMeta,
        argValues: [len],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicMemoryTestOutputComplexStructConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_memory_test_output_complex_struct",
        argNames: ["len"],
      );

  Future<int> offTopicDeliberatelyReturnError({dynamic hint}) => executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_deliberately_return_error(port_),
        parseSuccessData: _wire2api_i32,
        constMeta: kOffTopicDeliberatelyReturnErrorConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicDeliberatelyReturnErrorConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_deliberately_return_error",
        argNames: [],
      );

  Future<int> offTopicDeliberatelyPanic({dynamic hint}) => executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_off_topic_deliberately_panic(port_),
        parseSuccessData: _wire2api_i32,
        constMeta: kOffTopicDeliberatelyPanicConstMeta,
        argValues: [],
        hint: hint,
      ));

  FlutterRustBridgeTaskConstMeta get kOffTopicDeliberatelyPanicConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "off_topic_deliberately_panic",
        argNames: [],
      );

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  ffi.Pointer<wire_Point> _api2wire_box_autoadd_point(Point raw) {
    final ptr = inner.new_box_autoadd_point();
    _api_fill_to_wire_point(raw, ptr.ref);
    return ptr;
  }

  ffi.Pointer<wire_Size> _api2wire_box_autoadd_size(Size raw) {
    final ptr = inner.new_box_autoadd_size();
    _api_fill_to_wire_size(raw, ptr.ref);
    return ptr;
  }

  ffi.Pointer<wire_TreeNode> _api2wire_box_autoadd_tree_node(TreeNode raw) {
    final ptr = inner.new_box_autoadd_tree_node();
    _api_fill_to_wire_tree_node(raw, ptr.ref);
    return ptr;
  }

  double _api2wire_f64(double raw) {
    return raw;
  }

  int _api2wire_i32(int raw) {
    return raw;
  }

  ffi.Pointer<wire_list_size> _api2wire_list_size(List<Size> raw) {
    final ans = inner.new_list_size(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_size(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  ffi.Pointer<wire_list_tree_node> _api2wire_list_tree_node(List<TreeNode> raw) {
    final ans = inner.new_list_tree_node(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_tree_node(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_point(Point apiObj, ffi.Pointer<wire_Point> wireObj) {
    _api_fill_to_wire_point(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_size(Size apiObj, ffi.Pointer<wire_Size> wireObj) {
    _api_fill_to_wire_size(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tree_node(TreeNode apiObj, ffi.Pointer<wire_TreeNode> wireObj) {
    _api_fill_to_wire_tree_node(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_point(Point apiObj, wire_Point wireObj) {
    wireObj.x = _api2wire_f64(apiObj.x);
    wireObj.y = _api2wire_f64(apiObj.y);
  }

  void _api_fill_to_wire_size(Size apiObj, wire_Size wireObj) {
    wireObj.width = _api2wire_i32(apiObj.width);
    wireObj.height = _api2wire_i32(apiObj.height);
  }

  void _api_fill_to_wire_tree_node(TreeNode apiObj, wire_TreeNode wireObj) {
    wireObj.name = _api2wire_String(apiObj.name);
    wireObj.children = _api2wire_list_tree_node(apiObj.children);
  }
}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

Uint8List _wire2api_ZeroCopyBuffer_Uint8List(dynamic raw) {
  return raw as Uint8List;
}

Point _wire2api_box_point(dynamic raw) {
  return _wire2api_point(raw);
}

BoxedPoint _wire2api_boxed_point(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1) throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return BoxedPoint(
    point: _wire2api_box_point(arr[0]),
  );
}

double _wire2api_f64(dynamic raw) {
  return raw as double;
}

int _wire2api_i32(dynamic raw) {
  return raw as int;
}

List<Size> _wire2api_list_size(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_size).toList();
}

List<TreeNode> _wire2api_list_tree_node(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_tree_node).toList();
}

Point _wire2api_point(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2) throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return Point(
    x: _wire2api_f64(arr[0]),
    y: _wire2api_f64(arr[1]),
  );
}

Size _wire2api_size(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2) throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return Size(
    width: _wire2api_i32(arr[0]),
    height: _wire2api_i32(arr[1]),
  );
}

TreeNode _wire2api_tree_node(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2) throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return TreeNode(
    name: _wire2api_String(arr[0]),
    children: _wire2api_list_tree_node(arr[1]),
  );
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class FlutterRustBridgeExampleWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FlutterRustBridgeExampleWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FlutterRustBridgeExampleWire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup)
      : _lookup = lookup;

  void wire_draw_mandelbrot(
    int port_,
    ffi.Pointer<wire_Size> image_size,
    ffi.Pointer<wire_Point> zoom_point,
    double scale,
    int num_threads,
  ) {
    return _wire_draw_mandelbrot(
      port_,
      image_size,
      zoom_point,
      scale,
      num_threads,
    );
  }

  late final _wire_draw_mandelbrotPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_Size>, ffi.Pointer<wire_Point>, ffi.Double,
              ffi.Int32)>>('wire_draw_mandelbrot');
  late final _wire_draw_mandelbrot = _wire_draw_mandelbrotPtr
      .asFunction<void Function(int, ffi.Pointer<wire_Size>, ffi.Pointer<wire_Point>, double, int)>();

  void wire_passing_complex_structs(
    int port_,
    ffi.Pointer<wire_TreeNode> root,
  ) {
    return _wire_passing_complex_structs(
      port_,
      root,
    );
  }

  late final _wire_passing_complex_structsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_TreeNode>)>>(
          'wire_passing_complex_structs');
  late final _wire_passing_complex_structs =
      _wire_passing_complex_structsPtr.asFunction<void Function(int, ffi.Pointer<wire_TreeNode>)>();

  void wire_returning_structs_with_boxed_fields(
    int port_,
  ) {
    return _wire_returning_structs_with_boxed_fields(
      port_,
    );
  }

  late final _wire_returning_structs_with_boxed_fieldsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_returning_structs_with_boxed_fields');
  late final _wire_returning_structs_with_boxed_fields =
      _wire_returning_structs_with_boxed_fieldsPtr.asFunction<void Function(int)>();

  void wire_off_topic_memory_test_input_array(
    int port_,
    ffi.Pointer<wire_uint_8_list> input,
  ) {
    return _wire_off_topic_memory_test_input_array(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_arrayPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
          'wire_off_topic_memory_test_input_array');
  late final _wire_off_topic_memory_test_input_array =
      _wire_off_topic_memory_test_input_arrayPtr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_off_topic_memory_test_output_zero_copy_buffer(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_zero_copy_buffer(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_zero_copy_bufferPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_zero_copy_buffer');
  late final _wire_off_topic_memory_test_output_zero_copy_buffer =
      _wire_off_topic_memory_test_output_zero_copy_bufferPtr.asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_output_vec_u8(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_vec_u8(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_vec_u8Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>('wire_off_topic_memory_test_output_vec_u8');
  late final _wire_off_topic_memory_test_output_vec_u8 =
      _wire_off_topic_memory_test_output_vec_u8Ptr.asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_input_vec_of_object(
    int port_,
    ffi.Pointer<wire_list_size> input,
  ) {
    return _wire_off_topic_memory_test_input_vec_of_object(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_vec_of_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_size>)>>(
          'wire_off_topic_memory_test_input_vec_of_object');
  late final _wire_off_topic_memory_test_input_vec_of_object =
      _wire_off_topic_memory_test_input_vec_of_objectPtr.asFunction<void Function(int, ffi.Pointer<wire_list_size>)>();

  void wire_off_topic_memory_test_output_vec_of_object(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_vec_of_object(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_vec_of_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_vec_of_object');
  late final _wire_off_topic_memory_test_output_vec_of_object =
      _wire_off_topic_memory_test_output_vec_of_objectPtr.asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_input_complex_struct(
    int port_,
    ffi.Pointer<wire_TreeNode> input,
  ) {
    return _wire_off_topic_memory_test_input_complex_struct(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_complex_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_TreeNode>)>>(
          'wire_off_topic_memory_test_input_complex_struct');
  late final _wire_off_topic_memory_test_input_complex_struct =
      _wire_off_topic_memory_test_input_complex_structPtr.asFunction<void Function(int, ffi.Pointer<wire_TreeNode>)>();

  void wire_off_topic_memory_test_output_complex_struct(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_complex_struct(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_complex_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_complex_struct');
  late final _wire_off_topic_memory_test_output_complex_struct =
      _wire_off_topic_memory_test_output_complex_structPtr.asFunction<void Function(int, int)>();

  void wire_off_topic_deliberately_return_error(
    int port_,
  ) {
    return _wire_off_topic_deliberately_return_error(
      port_,
    );
  }

  late final _wire_off_topic_deliberately_return_errorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_off_topic_deliberately_return_error');
  late final _wire_off_topic_deliberately_return_error =
      _wire_off_topic_deliberately_return_errorPtr.asFunction<void Function(int)>();

  void wire_off_topic_deliberately_panic(
    int port_,
  ) {
    return _wire_off_topic_deliberately_panic(
      port_,
    );
  }

  late final _wire_off_topic_deliberately_panicPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_off_topic_deliberately_panic');
  late final _wire_off_topic_deliberately_panic =
      _wire_off_topic_deliberately_panicPtr.asFunction<void Function(int)>();

  ffi.Pointer<wire_Point> new_box_autoadd_point() {
    return _new_box_autoadd_point();
  }

  late final _new_box_autoadd_pointPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Point> Function()>>('new_box_autoadd_point');
  late final _new_box_autoadd_point = _new_box_autoadd_pointPtr.asFunction<ffi.Pointer<wire_Point> Function()>();

  ffi.Pointer<wire_Size> new_box_autoadd_size() {
    return _new_box_autoadd_size();
  }

  late final _new_box_autoadd_sizePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Size> Function()>>('new_box_autoadd_size');
  late final _new_box_autoadd_size = _new_box_autoadd_sizePtr.asFunction<ffi.Pointer<wire_Size> Function()>();

  ffi.Pointer<wire_TreeNode> new_box_autoadd_tree_node() {
    return _new_box_autoadd_tree_node();
  }

  late final _new_box_autoadd_tree_nodePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_TreeNode> Function()>>('new_box_autoadd_tree_node');
  late final _new_box_autoadd_tree_node =
      _new_box_autoadd_tree_nodePtr.asFunction<ffi.Pointer<wire_TreeNode> Function()>();

  ffi.Pointer<wire_list_size> new_list_size(
    int len,
  ) {
    return _new_list_size(
      len,
    );
  }

  late final _new_list_sizePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_list_size> Function(ffi.Int32)>>('new_list_size');
  late final _new_list_size = _new_list_sizePtr.asFunction<ffi.Pointer<wire_list_size> Function(int)>();

  ffi.Pointer<wire_list_tree_node> new_list_tree_node(
    int len,
  ) {
    return _new_list_tree_node(
      len,
    );
  }

  late final _new_list_tree_nodePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_list_tree_node> Function(ffi.Int32)>>('new_list_tree_node');
  late final _new_list_tree_node = _new_list_tree_nodePtr.asFunction<ffi.Pointer<wire_list_tree_node> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr.asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>('free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct =
      _free_WireSyncReturnStructPtr.asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_Size extends ffi.Struct {
  @ffi.Int32()
  external int width;

  @ffi.Int32()
  external int height;
}

class wire_Point extends ffi.Struct {
  @ffi.Double()
  external double x;

  @ffi.Double()
  external double y;
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_list_tree_node extends ffi.Struct {
  external ffi.Pointer<wire_TreeNode> ptr;

  @ffi.Int32()
  external int len;
}

class wire_TreeNode extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;

  external ffi.Pointer<wire_list_tree_node> children;
}

class wire_list_size extends ffi.Struct {
  external ffi.Pointer<wire_Size> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
