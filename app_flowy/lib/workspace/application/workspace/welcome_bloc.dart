import 'package:app_flowy/workspace/domain/i_user.dart';
import 'package:app_flowy/workspace/infrastructure/repos/user_repo.dart';
import 'package:flowy_infra/flowy_logger.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/errors.pb.dart';
import 'package:flowy_sdk/protobuf/flowy-workspace/workspace_create.pb.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:dartz/dartz.dart';

part 'welcome_bloc.freezed.dart';

class WelcomeBloc extends Bloc<WelcomeEvent, WelcomeState> {
  final UserRepo repo;
  final IUserWatch watch;
  WelcomeBloc({required this.repo, required this.watch})
      : super(WelcomeState.initial());

  @override
  Stream<WelcomeState> mapEventToState(
    WelcomeEvent event,
  ) async* {
    yield* event.map(initial: (e) async* {
      watch.setWorkspacesCallback(_workspacesUpdated);
      watch.startWatching();
      //
      yield* _fetchWorkspaces();
    }, openWorkspace: (e) async* {
      yield* _openWorkspace(e.workspace);
    }, createWorkspace: (e) async* {
      yield* _createWorkspace(e.name, e.desc);
    }, workspacesReveived: (e) async* {
      yield e.workspacesOrFail.fold(
        (workspaces) => state.copyWith(
            workspaces: workspaces, successOrFailure: left(unit)),
        (error) => state.copyWith(successOrFailure: right(error)),
      );
    });
  }

  @override
  Future<void> close() async {
    await watch.stopWatching();
    super.close();
  }

  Stream<WelcomeState> _fetchWorkspaces() async* {
    final workspacesOrFailed = await repo.getWorkspaces();
    yield workspacesOrFailed.fold(
      (workspaces) =>
          state.copyWith(workspaces: workspaces, successOrFailure: left(unit)),
      (error) {
        Log.error(error);
        return state.copyWith(successOrFailure: right(error));
      },
    );
  }

  Stream<WelcomeState> _openWorkspace(Workspace workspace) async* {
    final result = await repo.openWorkspace(workspace.id);
    yield result.fold(
      (workspaces) => state.copyWith(successOrFailure: left(unit)),
      (error) {
        Log.error(error);
        return state.copyWith(successOrFailure: right(error));
      },
    );
  }

  Stream<WelcomeState> _createWorkspace(String name, String desc) async* {
    final result = await repo.createWorkspace(name, desc);
    yield result.fold(
      (workspace) {
        // add(const WelcomeEvent.fetchWorkspaces());
        return state.copyWith(successOrFailure: left(unit));
      },
      (error) {
        Log.error(error);
        return state.copyWith(successOrFailure: right(error));
      },
    );
  }

  void _workspacesUpdated(
      Either<List<Workspace>, WorkspaceError> workspacesOrFail) {
    add(WelcomeEvent.workspacesReveived(workspacesOrFail));
  }
}

@freezed
abstract class WelcomeEvent with _$WelcomeEvent {
  const factory WelcomeEvent.initial() = Initial;
  // const factory WelcomeEvent.fetchWorkspaces() = FetchWorkspace;
  const factory WelcomeEvent.createWorkspace(String name, String desc) =
      CreateWorkspace;
  const factory WelcomeEvent.openWorkspace(Workspace workspace) = OpenWorkspace;

  const factory WelcomeEvent.workspacesReveived(
          Either<List<Workspace>, WorkspaceError> workspacesOrFail) =
      WorkspacesReceived;
}

@freezed
abstract class WelcomeState implements _$WelcomeState {
  const factory WelcomeState({
    required bool isLoading,
    required List<Workspace> workspaces,
    required Either<Unit, WorkspaceError> successOrFailure,
  }) = _WelcomeState;

  factory WelcomeState.initial() => WelcomeState(
        isLoading: false,
        workspaces: List.empty(),
        successOrFailure: left(unit),
      );
}
