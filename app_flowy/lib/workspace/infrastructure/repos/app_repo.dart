import 'dart:async';
import 'dart:typed_data';
import 'package:app_flowy/workspace/domain/i_app.dart';
import 'package:dartz/dartz.dart';
import 'package:flowy_log/flowy_log.dart';
import 'package:flowy_sdk/dispatch/dispatch.dart';
import 'package:flowy_sdk/protobuf/flowy-dart-notify/subject.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/app_create.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/app_delete.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/app_query.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/app_update.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/observable.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/view_create.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/view_create.pbenum.dart';
import 'package:flowy_sdk/rust_stream.dart';
import 'helper.dart';

class AppRepository {
  String appId;
  AppRepository({
    required this.appId,
  });

  Future<Either<App, WorkspaceError>> getAppDesc() {
    final request = QueryAppRequest.create()..appIds.add(appId);

    return WorkspaceEventReadApp(request).send();
  }

  Future<Either<View, WorkspaceError>> createView(String name, String desc, ViewType viewType) {
    final request = CreateViewRequest.create()
      ..belongToId = appId
      ..name = name
      ..desc = desc
      ..viewType = viewType;

    return WorkspaceEventCreateView(request).send();
  }

  Future<Either<List<View>, WorkspaceError>> getViews() {
    final request = QueryAppRequest.create()..appIds.add(appId);

    return WorkspaceEventReadApp(request).send().then((result) {
      return result.fold(
        (app) => left(app.belongings.items),
        (error) => right(error),
      );
    });
  }

  Future<Either<Unit, WorkspaceError>> delete() {
    final request = QueryAppRequest.create()..appIds.add(appId);
    return WorkspaceEventDeleteApp(request).send();
  }

  Future<Either<Unit, WorkspaceError>> updateApp({String? name}) {
    UpdateAppRequest request = UpdateAppRequest.create()..appId = appId;

    if (name != null) {
      request.name = name;
    }
    return WorkspaceEventUpdateApp(request).send();
  }
}

class AppListenerRepository {
  StreamSubscription<SubscribeObject>? _subscription;
  AppViewsChangeCallback? _viewsChanged;
  AppUpdatedCallback? _update;
  late WorkspaceNotificationParser _parser;
  String appId;

  AppListenerRepository({
    required this.appId,
  });

  void startListening({AppViewsChangeCallback? viewsChanged, AppUpdatedCallback? update}) {
    _viewsChanged = viewsChanged;
    _update = update;
    _parser = WorkspaceNotificationParser(id: appId, callback: _bservableCallback);
    _subscription = RustStreamReceiver.listen((observable) => _parser.parse(observable));
  }

  void _bservableCallback(WorkspaceNotification ty, Either<Uint8List, WorkspaceError> result) {
    switch (ty) {
      case WorkspaceNotification.AppViewsChanged:
        if (_viewsChanged != null) {
          result.fold(
            (payload) {
              final repeatedView = RepeatedView.fromBuffer(payload);
              _viewsChanged!(left(repeatedView.items));
            },
            (error) => _viewsChanged!(right(error)),
          );
        }
        break;
      case WorkspaceNotification.AppUpdated:
        if (_update != null) {
          result.fold(
            (payload) {
              final app = App.fromBuffer(payload);
              _update!(app);
            },
            (error) => Log.error(error),
          );
        }
        break;
      default:
        break;
    }
  }

  Future<void> close() async {
    await _subscription?.cancel();
  }
}
