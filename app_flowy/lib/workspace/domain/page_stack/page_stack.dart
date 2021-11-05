import 'package:flowy_infra/notifier.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:app_flowy/startup/startup.dart';
import 'package:app_flowy/workspace/presentation/stack_page/blank/blank_page.dart';
import 'package:app_flowy/workspace/presentation/stack_page/home_stack.dart';
import 'package:app_flowy/workspace/presentation/widgets/prelude.dart';

typedef NavigationCallback = void Function(String id);

abstract class NavigationItem {
  Widget get naviTitle;
  String get identifier;

  NavigationCallback get action => (id) {
        getIt<HomeStackManager>().setStackWithId(id);
      };
}

enum HomeStackType {
  blank,
  doc,
  trash,
}

List<HomeStackType> pages = HomeStackType.values.toList();

abstract class HomeStackContext<T> with NavigationItem {
  List<NavigationItem> get navigationItems;

  @override
  Widget get naviTitle;

  @override
  String get identifier;

  ValueNotifier<T> get isUpdated;

  HomeStackType get type;

  Widget buildWidget();

  void dispose();
}

class HomeStackNotifier extends ChangeNotifier {
  HomeStackContext stackContext;
  PublishNotifier<bool> collapsedNotifier = PublishNotifier();

  Widget get titleWidget => stackContext.naviTitle;

  HomeStackNotifier({HomeStackContext? context}) : stackContext = context ?? BlankStackContext();

  set context(HomeStackContext context) {
    stackContext.isUpdated.removeListener(notifyListeners);
    stackContext.dispose();

    stackContext = context;
    stackContext.isUpdated.addListener(notifyListeners);
    notifyListeners();
  }

  HomeStackContext get context => stackContext;
}

// HomeStack is initialized as singleton to controll the page stack.
class HomeStackManager {
  final HomeStackNotifier _notifier = HomeStackNotifier();
  HomeStackManager();

  Widget title() {
    return _notifier.context.naviTitle;
  }

  PublishNotifier<bool> get collapsedNotifier => _notifier.collapsedNotifier;

  void switchStack(HomeStackContext context) {
    _notifier.context = context;
  }

  void setStackWithId(String id) {}

  Widget stackTopBar() {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider.value(value: _notifier),
      ],
      child: Selector<HomeStackNotifier, Widget>(
        selector: (context, notifier) => notifier.titleWidget,
        builder: (context, widget, child) {
          return const HomeTopBar();
        },
      ),
    );
  }

  Widget stackWidget() {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider.value(value: _notifier),
      ],
      child: Consumer(builder: (ctx, HomeStackNotifier notifier, child) {
        return FadingIndexedStack(
          index: pages.indexOf(notifier.context.type),
          children: HomeStackType.values.map((viewType) {
            if (viewType == notifier.context.type) {
              return notifier.context.buildWidget();
            } else {
              return const BlankStackPage();
            }
          }).toList(),
        );
      }),
    );
  }
}
