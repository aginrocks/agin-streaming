#pragma once
#include <QObject>

#include "inputaction.h"

class InputProvider: public QObject {
    Q_OBJECT

public:
    enum DeviceType { Keyboard, Gamepad, Remote };

    Q_ENUM(DeviceType);

    explicit InputProvider(DeviceType type, QObject* parent = nullptr) :
        QObject(parent),
        m_deviceType(type) {}

    virtual ~InputProvider() = default;

signals:
    void actionTriggered(const InputAction &action);

protected:
    void emitAction(
        InputAction::Type type,
        InputAction::State state,
        const QVariant &data = QVariant()
    );

private:
    DeviceType m_deviceType;
};
