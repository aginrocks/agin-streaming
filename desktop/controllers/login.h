
#pragma once

#include <qtypes.h>
#include <QObject>
#include <QString>

namespace UILogic {

class LoginController: public QObject {
public:
    explicit LoginController(QObject* parent = nullptr) : QObject(parent) {}
    Q_OBJECT
    Q_PROPERTY(bool login READ login NOTIFY loginChanged)

    bool login() const {
        return 1;
    }
signals:
    void loginChanged();
};

} // namespace UILogic
