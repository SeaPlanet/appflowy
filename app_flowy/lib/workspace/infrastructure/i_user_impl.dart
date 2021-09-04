import 'package:dartz/dartz.dart';
import 'package:app_flowy/workspace/domain/i_user.dart';
import 'package:app_flowy/workspace/infrastructure/repos/user_repo.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
export 'package:app_flowy/workspace/domain/i_user.dart';
export 'package:app_flowy/workspace/infrastructure/repos/user_repo.dart';

class IUserImpl extends IUser {
  UserRepo repo;
  IUserImpl({
    required this.repo,
  });

  @override
  Future<Either<Unit, WorkspaceError>> deleteWorkspace(String workspaceId) {
    return repo.deleteWorkspace(workspaceId: workspaceId);
  }

  @override
  Future<Either<UserProfile, UserError>> fetchUserProfile(String userId) {
    return repo.fetchUserProfile(userId: userId);
  }

  @override
  Future<Either<Unit, UserError>> signOut() {
    return repo.signOut();
  }

  @override
  UserProfile get user => repo.user;

  @override
  Future<Either<List<Workspace>, WorkspaceError>> fetchWorkspaces() {
    return repo.fetchWorkspaces();
  }
}

class IUserWatchImpl extends IUserWatch {
  UserWatchRepo repo;
  IUserWatchImpl({
    required this.repo,
  });
  @override
  void startWatching(
      {UserCreateWorkspaceCallback? createWorkspaceCallback,
      UserDeleteWorkspaceCallback? deleteWorkspaceCallback}) {
    repo.startWatching(
        createWorkspace: createWorkspaceCallback,
        deleteWorkspace: deleteWorkspaceCallback);
  }

  @override
  Future<void> stopWatching() async {
    await repo.close();
  }
}
