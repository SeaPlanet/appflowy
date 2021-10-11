import 'package:app_flowy/workspace/domain/i_user.dart';
import 'package:flowy_log/flowy_log.dart';
import 'package:flowy_sdk/protobuf/flowy-user/user_profile.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/workspace_create.pb.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:dartz/dartz.dart';

part 'menu_user_bloc.freezed.dart';

class MenuUserBloc extends Bloc<MenuUserEvent, MenuUserState> {
  final IUser iUserImpl;
  final IUserListener listener;

  MenuUserBloc(this.iUserImpl, this.listener) : super(MenuUserState.initial(iUserImpl.user));

  @override
  Stream<MenuUserState> mapEventToState(MenuUserEvent event) async* {
    yield* event.map(
      initial: (_) async* {
        listener.setProfileCallback(_profileUpdated);
        listener.setWorkspacesCallback(_workspacesUpdated);
        listener.start();

        await _initUser();
      },
      fetchWorkspaces: (_FetchWorkspaces value) async* {},
    );
  }

  @override
  Future<void> close() async {
    await listener.stop();
    super.close();
  }

  Future<void> _initUser() async {
    final result = await iUserImpl.initUser();
    result.fold((l) => null, (error) => Log.error(error));
  }

  void _profileUpdated(Either<UserProfile, UserError> userOrFailed) {}
  void _workspacesUpdated(Either<List<Workspace>, WorkspaceError> workspacesOrFailed) {
    // fetch workspaces
    // iUserImpl.fetchWorkspaces().then((result) {
    //   result.fold(
    //     (workspaces) async* {
    //       yield state.copyWith(workspaces: some(workspaces));
    //     },
    //     (error) async* {
    //       yield state.copyWith(successOrFailure: right(error.msg));
    //     },
    //   );
    // });
  }
}

@freezed
class MenuUserEvent with _$MenuUserEvent {
  const factory MenuUserEvent.initial() = _Initial;
  const factory MenuUserEvent.fetchWorkspaces() = _FetchWorkspaces;
}

@freezed
class MenuUserState with _$MenuUserState {
  const factory MenuUserState({
    required UserProfile user,
    required Option<List<Workspace>> workspaces,
    required Either<Unit, String> successOrFailure,
  }) = _MenuUserState;

  factory MenuUserState.initial(UserProfile user) => MenuUserState(
        user: user,
        workspaces: none(),
        successOrFailure: left(unit),
      );
}
