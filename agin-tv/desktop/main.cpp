#include <QDebug>
#include <QEvent>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QQmlApplicationEngine>

class MyApp : public QGuiApplication
{
    Q_OBJECT
public:
    MyApp(int &argc, char **argv)
        : QGuiApplication(argc, argv)
    {}
    virtual ~MyApp() {}
    bool notify(QObject *obj, QEvent *event) override
    {
        // qDebug() << this->focusWindow();
        // qDebug() << event->type();
        if (event->type() == QEvent::KeyPress) {
            QKeyEvent *keyEvent = static_cast<QKeyEvent *>(event);
            qDebug() << "Ate key press" << keyEvent->key();
            return true;
        }
        return QGuiApplication::notify(obj, event);
    }
};

int main(int argc, char *argv[])
{
    qputenv("QT_IM_MODULE", QByteArray("qtvirtualkeyboard"));

    MyApp app(argc, argv);

    QQmlApplicationEngine engine;
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection);
    engine.loadFromModule("AginTV", "Main");
    qDebug() << "Starting App";
    return app.exec();
}

#include "main.moc"
