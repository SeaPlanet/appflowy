import 'package:app_flowy/workspace/domain/edit_action/app_edit.dart';
import 'package:app_flowy/workspace/presentation/widgets/pop_up_action.dart';
import 'package:dartz/dartz.dart' as dartz;
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';

class AppDisclosureActionSheet with ActionList<DisclosureActionWrapper> implements FlowyOverlayDelegate {
  final Function(dartz.Option<AppDisclosureAction>) onSelected;
  final _items = AppDisclosureAction.values.map((action) => DisclosureActionWrapper(action)).toList();

  AppDisclosureActionSheet({
    required this.onSelected,
  });

  @override
  List<DisclosureActionWrapper> get items => _items;

  @override
  void Function(dartz.Option<DisclosureActionWrapper> p1) get selectCallback => (result) {
        result.fold(
          () => onSelected(dartz.none()),
          (wrapper) => onSelected(
            dartz.some(wrapper.inner),
          ),
        );
      };

  @override
  FlowyOverlayDelegate? get delegate => this;

  @override
  void didRemove() {
    onSelected(dartz.none());
  }

  @override
  ListOverlayFooter? get footer => null;
}

class DisclosureActionWrapper extends ActionItem {
  final AppDisclosureAction inner;

  DisclosureActionWrapper(this.inner);
  @override
  Widget? get icon => inner.icon;

  @override
  String get name => inner.name;
}
