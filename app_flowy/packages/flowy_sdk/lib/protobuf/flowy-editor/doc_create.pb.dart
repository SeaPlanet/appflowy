///
//  Generated code. Do not modify.
//  source: doc_create.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CreateDocRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateDocRequest', createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewId')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..hasRequiredFields = false
  ;

  CreateDocRequest._() : super();
  factory CreateDocRequest({
    $core.String? viewId,
    $core.String? name,
  }) {
    final _result = create();
    if (viewId != null) {
      _result.viewId = viewId;
    }
    if (name != null) {
      _result.name = name;
    }
    return _result;
  }
  factory CreateDocRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateDocRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateDocRequest clone() => CreateDocRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateDocRequest copyWith(void Function(CreateDocRequest) updates) => super.copyWith((message) => updates(message as CreateDocRequest)) as CreateDocRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CreateDocRequest create() => CreateDocRequest._();
  CreateDocRequest createEmptyInstance() => create();
  static $pb.PbList<CreateDocRequest> createRepeated() => $pb.PbList<CreateDocRequest>();
  @$core.pragma('dart2js:noInline')
  static CreateDocRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateDocRequest>(create);
  static CreateDocRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get viewId => $_getSZ(0);
  @$pb.TagNumber(1)
  set viewId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasViewId() => $_has(0);
  @$pb.TagNumber(1)
  void clearViewId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
}

class Doc extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Doc', createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'viewId')
    ..hasRequiredFields = false
  ;

  Doc._() : super();
  factory Doc({
    $core.String? id,
    $core.String? name,
    $core.String? viewId,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (viewId != null) {
      _result.viewId = viewId;
    }
    return _result;
  }
  factory Doc.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Doc.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Doc clone() => Doc()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Doc copyWith(void Function(Doc) updates) => super.copyWith((message) => updates(message as Doc)) as Doc; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Doc create() => Doc._();
  Doc createEmptyInstance() => create();
  static $pb.PbList<Doc> createRepeated() => $pb.PbList<Doc>();
  @$core.pragma('dart2js:noInline')
  static Doc getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Doc>(create);
  static Doc? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get viewId => $_getSZ(2);
  @$pb.TagNumber(3)
  set viewId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasViewId() => $_has(2);
  @$pb.TagNumber(3)
  void clearViewId() => clearField(3);
}

