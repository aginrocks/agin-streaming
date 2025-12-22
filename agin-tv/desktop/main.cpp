#include <QDebug>
#include <QEvent>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QQmlApplicationEngine>

#include "input/inputdispatcher.h"
#include "input/keyboardinputprovider.h"

class MyApp: public QGuiApplication {
    Q_OBJECT
public:
    MyApp(int& argc, char** argv, KeyboardInputProvider* keyboardProvider) :
        QGuiApplication(argc, argv),
        m_keyboardProvider(keyboardProvider) {}

    virtual ~MyApp() {}

    bool notify(QObject* obj, QEvent* event) override {
        if (
            m_keyboardProvider &&
            (event->type() == QEvent::KeyPress
             || event->type() == QEvent::KeyRelease)
        ) {
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

    dispatcher->registerProvider(keyboardProvider);

    return keyboardProvider;
}

int main(int argc, char* argv[]) {
    qputenv("QT_IM_MODULE", QByteArray("qtvirtualkeyboard"));

    auto keyboardProvider = setupInput();

    MyApp app(argc, argv, keyboardProvider);

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
