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

    DeviceType deviceType() const {
        return m_deviceType;
    }

    bool isActive() const {
        return m_active;
    }

signals:
    void actionTriggered(const InputAction& action);
    void activeChanged(bool active);

protected:
    void emitAction(
        InputAction::Type type,
        InputAction::State state,
        const QVariant& data = QVariant()
    );

    void setActive(bool active) {
        m_active = active;
        emit activeChanged(active);
    }

private:
    DeviceType m_deviceType;
    bool m_active = false;
};
