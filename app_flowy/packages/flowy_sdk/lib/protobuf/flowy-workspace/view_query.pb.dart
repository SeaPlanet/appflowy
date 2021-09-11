///
//  Generated code. Do not modify.
//  source: view_query.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class QueryViewRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QueryViewRequest', createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewId')
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'isTrash')
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readBelongings')
    ..hasRequiredFields = false
  ;

  QueryViewRequest._() : super();
  factory QueryViewRequest({
    $core.String? viewId,
    $core.bool? isTrash,
    $core.bool? readBelongings,
  }) {
    final _result = create();
    if (viewId != null) {
      _result.viewId = viewId;
    }
    if (isTrash != null) {
      _result.isTrash = isTrash;
    }
    if (readBelongings != null) {
      _result.readBelongings = readBelongings;
    }
    return _result;
  }
  factory QueryViewRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QueryViewRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QueryViewRequest clone() => QueryViewRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QueryViewRequest copyWith(void Function(QueryViewRequest) updates) => super.copyWith((message) => updates(message as QueryViewRequest)) as QueryViewRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static QueryViewRequest create() => QueryViewRequest._();
  QueryViewRequest createEmptyInstance() => create();
  static $pb.PbList<QueryViewRequest> createRepeated() => $pb.PbList<QueryViewRequest>();
  @$core.pragma('dart2js:noInline')
  static QueryViewRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryViewRequest>(create);
  static QueryViewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get viewId => $_getSZ(0);
  @$pb.TagNumber(1)
  set viewId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasViewId() => $_has(0);
  @$pb.TagNumber(1)
  void clearViewId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get isTrash => $_getBF(1);
  @$pb.TagNumber(2)
  set isTrash($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasIsTrash() => $_has(1);
  @$pb.TagNumber(2)
  void clearIsTrash() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get readBelongings => $_getBF(2);
  @$pb.TagNumber(3)
  set readBelongings($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasReadBelongings() => $_has(2);
  @$pb.TagNumber(3)
  void clearReadBelongings() => clearField(3);
}

class QueryViewParams extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QueryViewParams', createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewId')
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'isTrash')
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'readBelongings')
    ..hasRequiredFields = false
  ;

  QueryViewParams._() : super();
  factory QueryViewParams({
    $core.String? viewId,
    $core.bool? isTrash,
    $core.bool? readBelongings,
  }) {
    final _result = create();
    if (viewId != null) {
      _result.viewId = viewId;
    }
    if (isTrash != null) {
      _result.isTrash = isTrash;
    }
    if (readBelongings != null) {
      _result.readBelongings = readBelongings;
    }
    return _result;
  }
  factory QueryViewParams.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QueryViewParams.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QueryViewParams clone() => QueryViewParams()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QueryViewParams copyWith(void Function(QueryViewParams) updates) => super.copyWith((message) => updates(message as QueryViewParams)) as QueryViewParams; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static QueryViewParams create() => QueryViewParams._();
  QueryViewParams createEmptyInstance() => create();
  static $pb.PbList<QueryViewParams> createRepeated() => $pb.PbList<QueryViewParams>();
  @$core.pragma('dart2js:noInline')
  static QueryViewParams getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryViewParams>(create);
  static QueryViewParams? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get viewId => $_getSZ(0);
  @$pb.TagNumber(1)
  set viewId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasViewId() => $_has(0);
  @$pb.TagNumber(1)
  void clearViewId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get isTrash => $_getBF(1);
  @$pb.TagNumber(2)
  set isTrash($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasIsTrash() => $_has(1);
  @$pb.TagNumber(2)
  void clearIsTrash() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get readBelongings => $_getBF(2);
  @$pb.TagNumber(3)
  set readBelongings($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasReadBelongings() => $_has(2);
  @$pb.TagNumber(3)
  void clearReadBelongings() => clearField(3);
}

class OpenViewRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'OpenViewRequest', createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewId')
    ..hasRequiredFields = false
  ;

  OpenViewRequest._() : super();
  factory OpenViewRequest({
    $core.String? viewId,
  }) {
    final _result = create();
    if (viewId != null) {
      _result.viewId = viewId;
    }
    return _result;
  }
  factory OpenViewRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory OpenViewRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  OpenViewRequest clone() => OpenViewRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  OpenViewRequest copyWith(void Function(OpenViewRequest) updates) => super.copyWith((message) => updates(message as OpenViewRequest)) as OpenViewRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static OpenViewRequest create() => OpenViewRequest._();
  OpenViewRequest createEmptyInstance() => create();
  static $pb.PbList<OpenViewRequest> createRepeated() => $pb.PbList<OpenViewRequest>();
  @$core.pragma('dart2js:noInline')
  static OpenViewRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OpenViewRequest>(create);
  static OpenViewRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get viewId => $_getSZ(0);
  @$pb.TagNumber(1)
  set viewId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasViewId() => $_has(0);
  @$pb.TagNumber(1)
  void clearViewId() => clearField(1);
}

