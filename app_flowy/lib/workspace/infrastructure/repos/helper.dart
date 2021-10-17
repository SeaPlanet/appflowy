import 'dart:typed_data';
import 'package:flowy_log/flowy_log.dart';
import 'package:flowy_sdk/protobuf/flowy-dart-notify/protobuf.dart';
import 'package:flowy_sdk/protobuf/flowy-user/protobuf.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/observable.pb.dart';
import 'package:dartz/dartz.dart';

typedef UserNotificationCallback = void Function(UserNotification, Either<Uint8List, UserError>);

class UserNotificationParser extends NotificationParser<UserNotification, UserError> {
  UserNotificationParser({required String id, required UserNotificationCallback callback})
      : super(
          id: id,
          callback: callback,
          tyParser: (ty) => UserNotification.valueOf(ty),
          errorParser: (bytes) => UserError.fromBuffer(bytes),
        );
}

typedef NotificationCallback = void Function(WorkspaceNotification, Either<Uint8List, WorkspaceError>);

class WorkspaceNotificationParser extends NotificationParser<WorkspaceNotification, WorkspaceError> {
  WorkspaceNotificationParser({String? id, required NotificationCallback callback})
      : super(
          id: id,
          callback: callback,
          tyParser: (ty) => WorkspaceNotification.valueOf(ty),
          errorParser: (bytes) => WorkspaceError.fromBuffer(bytes),
        );
}

class NotificationParser<T, E> {
  String? id;
  void Function(T, Either<Uint8List, E>) callback;

  T? Function(int) tyParser;
  E Function(Uint8List) errorParser;

  NotificationParser({this.id, required this.callback, required this.errorParser, required this.tyParser});
  void parse(SubscribeObject subject) {
    if (id != null) {
      if (subject.id != id) {
        return;
      }
    }

    final ty = tyParser(subject.ty);
    if (ty == null) {
      return;
    }

    if (subject.hasError()) {
      final bytes = Uint8List.fromList(subject.error);
      final error = errorParser(bytes);
      callback(ty, right(error));
    } else {
      final bytes = Uint8List.fromList(subject.payload);
      callback(ty, left(bytes));
    }
  }
}
