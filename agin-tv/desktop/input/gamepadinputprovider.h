
#pragma once
#include <QJoysticks.h>

#include <QKeyEvent>
#include <QMap>

#include "inputaction.h"
#include "inputprovider.h"

class GamepadInputProvider: public InputProvider {
    Q_OBJECT

public:
    explicit GamepadInputProvider(QObject* parent = nullptr);

    ~GamepadInputProvider() {}

private slots:
    void onJoystickCountChnaged();

    // Axis events
    void onAxisChanged(const int js, const int axis, const qreal value);

    // Button events
    void onButtonChanged(const int js, const int button, const bool pressed);

    // D-Pad events
    void onPovChanged(const int js, const int pov, const int angle);

private:
    InputAction::Type m_lastDpadAction = InputAction::Type::None;

    void connectSlots();
};
