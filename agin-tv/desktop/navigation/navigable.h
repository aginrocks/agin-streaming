#pragma once
#include <qtmetamacros.h>

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
    Q_PROPERTY(bool isScope READ isScope WRITE setIsScope NOTIFY isScopeChanged)

public:
    explicit Navigable(QObject* parent = nullptr);

    ~Navigable() {}

    bool canNavigate() const {
        return m_canNavigate;
    }

    bool hasFocus() const {
        return m_hasFocus;
    }

    bool isScope() const {
        return m_isScope;
    }

    static Navigable* qmlAttachedProperties(QObject* object) {
        return new Navigable(object);
    }

public slots:

    void setCanNavigate(bool canNavigate) {
        if (m_canNavigate == canNavigate)
            return;

        m_canNavigate = canNavigate;
        emit canNavigateChanged(m_canNavigate);
    }

    void setHasFocus(bool hasFocus) {
        if (m_hasFocus == hasFocus)
            return;

        m_hasFocus = hasFocus;
        emit hasFocusChanged(m_hasFocus);
    }

    void setIsScope(bool isScope) {
        if (m_isScope == isScope)
            return;

        m_isScope = isScope;
        emit isScopeChanged(m_isScope);
    }

signals:
    void canNavigateChanged(bool canNavigate);
    void hasFocusChanged(bool hasFocus);
    void isScopeChanged(bool isScope);

private:
    bool m_canNavigate = false;
    bool m_hasFocus = false;
    bool m_isScope = false;
};
