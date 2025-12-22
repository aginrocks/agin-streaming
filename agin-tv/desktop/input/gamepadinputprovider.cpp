#include "gamepadinputprovider.h"

#include <QJoysticks.h>
#include <qdebug.h>

#include "inputaction.h"

GamepadInputProvider::GamepadInputProvider(QObject* parent) :
    InputProvider(DeviceType::Gamepad, parent) {
    connectSlots();
}

void GamepadInputProvider::connectSlots() {
    auto* joysticks = QJoysticks::getInstance();

    connect(
        joysticks,
        &QJoysticks::countChanged,
        this,
        &GamepadInputProvider::onJoystickCountChnaged
    );

    connect(
        joysticks,
        &QJoysticks::axisChanged,
        this,
        &GamepadInputProvider::onAxisChanged
    );

    connect(
        joysticks,
        &QJoysticks::buttonChanged,
        this,
        &GamepadInputProvider::onButtonChanged
    );

    connect(
        joysticks,
        &QJoysticks::povChanged,
        this,
        &GamepadInputProvider::onPovChanged
    );
}

void GamepadInputProvider::onJoystickCountChnaged() {
    auto activeDevices = QJoysticks::getInstance()->count();
    bool isActive = activeDevices > 0;
    setActive(isActive);

    qDebug() << "Active devices:" << activeDevices;
}

void GamepadInputProvider::onAxisChanged(
    const int js,
    const int axis,
    const qreal value
) {
    qDebug() << "Axis changed from joystick" << js << "Axis:" << axis
             << "Value:" << value;
}

void GamepadInputProvider::onButtonChanged(
    const int js,
    const int button,
    const bool pressed
) {
    qDebug() << "Button event from joystick" << js << "Button:" << button
             << "Pressed:" << pressed;
}

void GamepadInputProvider::onPovChanged(
    const int js,
    const int pov,
    const int angle
) {
    // TODO: Add repeating

    qDebug() << "POV event from joystick" << js << "POV:" << pov
             << "Angle:" << angle;

    InputAction::Type action;
    switch (angle) {
        case -1:
            action = m_lastDpadAction;
            break;
        case 0:
            action = InputAction::Type::NavigateUp;
            break;
        case 90:
            action = InputAction::Type::NavigateRight;
            break;
        case 180:
            action = InputAction::Type::NavigateDown;
            break;
        case 270:
            action = InputAction::Type::NavigateLeft;
            break;
        // TODO: Consider adding support for intermediate angles
        default:
            return;
    }

    InputAction::State state;

    // TODO: More robust handling
    if (angle == -1) {
        m_lastDpadAction = InputAction::Type::None;
        state = InputAction::State::Released;
    } else {
        m_lastDpadAction = action;
        state = InputAction::State::Pressed;
    }

    emit actionTriggered(InputAction(action, state));
}
