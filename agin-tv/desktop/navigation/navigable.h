#pragma once
#include <QQuickItem>

class Navigable: public QObject {
    Q_OBJECT
    QML_ELEMENT
    QML_ATTACHED(Navigable)

    Q_PROPERTY(
        bool canNavigate READ canNavigate WRITE setCanNavigate NOTIFY
            canNavigateChanged
    )
    Q_PROPERTY(bool hasFocus READ hasFocus NOTIFY hasFocusChanged)

public:
    explicit Navigable(QObject* parent = nullptr);

    ~Navigable() {}

    bool canNavigate() const {
        return m_canNavigate;
    }

    bool hasFocus() const {
        return m_hasFocus;
    }

    static Navigable* qmlAttachedProperties(QObject* object) {
        return new Navigable(object);
    }

public slots:

    void setCanNavigate(bool canNavigate) {
        m_canNavigate = canNavigate;
    }

    void setHasFocus(bool hasFocus) {
        m_hasFocus = hasFocus;
    }

signals:
    void canNavigateChanged(bool canNavigate);
    void hasFocusChanged(bool hasFocus);

private:
    bool m_canNavigate = false;
    bool m_hasFocus = false;
};
