#pragma once
#include <QKeyEvent>
#include <QMap>

#include "inputprovider.h"

class KeyboardInputProvider: public InputProvider {
    Q_OBJECT

public:
    explicit KeyboardInputProvider(QObject* parent = nullptr);

    bool handleKeyEvent(QKeyEvent* event);

private:
    QMap<int, InputAction::Type> m_keyMap;

    void setupKeyMap();
};
