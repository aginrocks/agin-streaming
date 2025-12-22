#include <QJoysticks.h>

#include <QDebug>
#include <QEvent>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QQmlApplicationEngine>

#include "input/gamepadinputprovider.h"
#include "input/inputdispatcher.h"
#include "input/keyboardinputprovider.h"

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
    qputenv("QT_IM_MODULE", QByteArray("qtvirtualkeyboard"));

    MyApp app(argc, argv);

    auto keyboardProvider = setupInput();
    app.setKeyboardProvider(keyboardProvider);

    enableVirtualJoystick();

    QQmlApplicationEngine engine;
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection
    );
    engine.loadFromModule("AginTV", "Main");
    qDebug() << "Starting App";
    return app.exec();
}

#include "main.moc"
