part of 'home_bloc.dart';

@freezed
abstract class HomeEvent with _$HomeEvent {
  const factory HomeEvent.showLoading(bool isLoading) = _ShowLoading;
  const factory HomeEvent.forceCollapse(bool forceCollapse) = _ForceCollapse;

  //page
  const factory HomeEvent.setPage(PageContext context) = SetCurrentPage;

  //edit pannel
  const factory HomeEvent.setEditPannel(EditPannelContext editContext) =
      _ShowEditPannel;
  const factory HomeEvent.dismissEditPannel() = _DismissEditPannel;
}
