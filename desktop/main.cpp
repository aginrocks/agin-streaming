#include <QJoysticks.h>
#include <qguiapplication.h>
#include <qobject.h>
#include <qqml.h>
#include <qqmlcontext.h>
#include <qquickview.h>

#include <QDebug>
#include <QEvent>
#include <QFontDatabase>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickItem>
#include <QQuickView>
#include <QQuickWindow>

#include "controllers/login.h"
#include "input/gamepadinputprovider.h"
#include "input/inputdispatcher.h"
#include "input/keyboardinputprovider.h"
#include "navigation/navigationmanager.h"
#include "preferences/preferences.h"

class MyApp: public QGuiApplication {
    Q_OBJECT
public:
    MyApp(int& argc, char** argv) : QGuiApplication(argc, argv) {}

    void setKeyboardProvider(KeyboardInputProvider* keyboardProvider) {
        m_keyboardProvider = keyboardProvider;
    }

    bool notify(QObject* obj, QEvent* event) override {
        if (m_keyboardProvider
            && (event->type() == QEvent::KeyPress
                || event->type() == QEvent::KeyRelease)) {
            QKeyEvent* keyEvent = static_cast<QKeyEvent*>(event);
            if (m_keyboardProvider->handleKeyEvent(keyEvent))
                return true;
        }
        return QGuiApplication::notify(obj, event);
    }

private:
    KeyboardInputProvider* m_keyboardProvider = nullptr;
};

KeyboardInputProvider* setupInput() {
    auto dispatcher = InputDispatcher::instance();

    auto keyboardProvider = new KeyboardInputProvider(dispatcher);
    auto gamepadProvider = new GamepadInputProvider(dispatcher);

    dispatcher->registerProvider(keyboardProvider);
    dispatcher->registerProvider(gamepadProvider);

    return keyboardProvider;
}

void enableVirtualJoystick() {
    if (!qApp) {
        qWarning()
            << "Cannot enable virtual joystick before QApplication is constructed";
        return;
    }

    auto* joysticks = QJoysticks::getInstance();
    // joysticks->setVirtualJoystickRange(1.0);
    // joysticks->setVirtualJoystickAxisSensibility(0.7);
    joysticks->setVirtualJoystickEnabled(false);
    joysticks->updateInterfaces();
}

int main(int argc, char* argv[]) {
    //qputenv("QT_IM_MODULE", QByteArray("qtvirtualkeyboard"));

    MyApp app(argc, argv);
    QQmlApplicationEngine engine;
    qDebug() << engine.importPathList();
    /*
    auto keyboardProvider = setupInput();
    app.setKeyboardProvider(keyboardProvider);

    enableVirtualJoystick();


    auto navigationManager = new NavigationManager(&engine);
    InputDispatcher::instance()->setNavigationManager(navigationManager);
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection
    );

    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreated,
        navigationManager,
        [navigationManager](QObject* obj, const QUrl&) {
            if (!obj)
                return;

            if (auto window = qobject_cast<QQuickWindow*>(obj)) {
                navigationManager->setRootItem(window->contentItem());
            } else if (auto item = qobject_cast<QQuickItem*>(obj)) {
                navigationManager->setRootItem(item);
            }
        }
    );

    */

    // https://doc.qt.io/qt-6/qtjavascript.html#making-a-qobject-available-to-the-script-engine
    QObject* preferences = new User::Preferences;
    QJSValue objectValue = engine.newQObject(preferences);
    engine.globalObject().setProperty("preferences", objectValue);

    QObject* loginController = new UILogic::LoginController;
    //engine.rootContext()->setContextObject(loginController);
    //QJSValue objectValue1 = engine.newQObject(loginController);
    //engine.globalObject().setProperty("loginController", objectValue1);
    //engine.rootContext()->setContextObject(loginController);
    engine.rootContext()->setContextProperty("loginController",loginController);

    engine.loadFromModule("AginTV", "Main");
    qDebug() << "Starting App";
    return app.exec();
}

#include "main.moc"
