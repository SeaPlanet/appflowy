///
//  Generated code. Do not modify.
//  source: workspace_query.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

enum QueryWorkspaceRequest_OneOfWorkspaceId {
  workspaceId, 
  notSet
}

class QueryWorkspaceRequest extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, QueryWorkspaceRequest_OneOfWorkspaceId> _QueryWorkspaceRequest_OneOfWorkspaceIdByTag = {
    1 : QueryWorkspaceRequest_OneOfWorkspaceId.workspaceId,
    0 : QueryWorkspaceRequest_OneOfWorkspaceId.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QueryWorkspaceRequest', createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'workspaceId')
    ..hasRequiredFields = false
  ;

  QueryWorkspaceRequest._() : super();
  factory QueryWorkspaceRequest({
    $core.String? workspaceId,
  }) {
    final _result = create();
    if (workspaceId != null) {
      _result.workspaceId = workspaceId;
    }
    return _result;
  }
  factory QueryWorkspaceRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QueryWorkspaceRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QueryWorkspaceRequest clone() => QueryWorkspaceRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QueryWorkspaceRequest copyWith(void Function(QueryWorkspaceRequest) updates) => super.copyWith((message) => updates(message as QueryWorkspaceRequest)) as QueryWorkspaceRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static QueryWorkspaceRequest create() => QueryWorkspaceRequest._();
  QueryWorkspaceRequest createEmptyInstance() => create();
  static $pb.PbList<QueryWorkspaceRequest> createRepeated() => $pb.PbList<QueryWorkspaceRequest>();
  @$core.pragma('dart2js:noInline')
  static QueryWorkspaceRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryWorkspaceRequest>(create);
  static QueryWorkspaceRequest? _defaultInstance;

  QueryWorkspaceRequest_OneOfWorkspaceId whichOneOfWorkspaceId() => _QueryWorkspaceRequest_OneOfWorkspaceIdByTag[$_whichOneof(0)]!;
  void clearOneOfWorkspaceId() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get workspaceId => $_getSZ(0);
  @$pb.TagNumber(1)
  set workspaceId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasWorkspaceId() => $_has(0);
  @$pb.TagNumber(1)
  void clearWorkspaceId() => clearField(1);
}

enum QueryWorkspaceParams_OneOfWorkspaceId {
  workspaceId, 
  notSet
}

class QueryWorkspaceParams extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, QueryWorkspaceParams_OneOfWorkspaceId> _QueryWorkspaceParams_OneOfWorkspaceIdByTag = {
    1 : QueryWorkspaceParams_OneOfWorkspaceId.workspaceId,
    0 : QueryWorkspaceParams_OneOfWorkspaceId.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'QueryWorkspaceParams', createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'workspaceId')
    ..hasRequiredFields = false
  ;

  QueryWorkspaceParams._() : super();
  factory QueryWorkspaceParams({
    $core.String? workspaceId,
  }) {
    final _result = create();
    if (workspaceId != null) {
      _result.workspaceId = workspaceId;
    }
    return _result;
  }
  factory QueryWorkspaceParams.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QueryWorkspaceParams.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QueryWorkspaceParams clone() => QueryWorkspaceParams()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QueryWorkspaceParams copyWith(void Function(QueryWorkspaceParams) updates) => super.copyWith((message) => updates(message as QueryWorkspaceParams)) as QueryWorkspaceParams; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static QueryWorkspaceParams create() => QueryWorkspaceParams._();
  QueryWorkspaceParams createEmptyInstance() => create();
  static $pb.PbList<QueryWorkspaceParams> createRepeated() => $pb.PbList<QueryWorkspaceParams>();
  @$core.pragma('dart2js:noInline')
  static QueryWorkspaceParams getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryWorkspaceParams>(create);
  static QueryWorkspaceParams? _defaultInstance;

  QueryWorkspaceParams_OneOfWorkspaceId whichOneOfWorkspaceId() => _QueryWorkspaceParams_OneOfWorkspaceIdByTag[$_whichOneof(0)]!;
  void clearOneOfWorkspaceId() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get workspaceId => $_getSZ(0);
  @$pb.TagNumber(1)
  set workspaceId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasWorkspaceId() => $_has(0);
  @$pb.TagNumber(1)
  void clearWorkspaceId() => clearField(1);
}

