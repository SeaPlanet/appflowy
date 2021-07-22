import 'package:app_flowy/home/domain/i_workspace.dart';
import 'package:app_flowy/home/infrastructure/repos/workspace_repo.dart';
import 'package:dartz/dartz.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/app_create.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';

export 'package:app_flowy/home/domain/i_workspace.dart';

class IWorkspaceImpl extends IWorkspace {
  WorkspaceRepo repo;
  IWorkspaceImpl({
    required this.repo,
  });

  @override
  Future<Either<App, WorkspaceError>> createApp(
      {required String name, String? desc}) {
    return repo.createApp(name, desc ?? "");
  }

  @override
  Future<Either<List<App>, WorkspaceError>> getApps() {
    return repo.getWorkspace(readApps: true).then((result) {
      return result.fold(
        (workspace) => left(workspace.apps.items),
        (error) => right(error),
      );
    });
  }
}

class IWorkspaceWatchImpl extends IWorkspaceWatch {
  WorkspaceWatchRepo repo;
  IWorkspaceWatchImpl({
    required this.repo,
  });

  @override
  void startWatching(
      {WorkspaceAddAppCallback? addAppCallback,
      WorkspaceUpdatedCallback? updatedCallback}) {
    repo.startWatching(
        addAppCallback: addAppCallback, updatedCallback: updatedCallback);
  }

  @override
  Future<void> stopWatching() async {
    await repo.close();
  }
}
