#pragma once
#include "inputprovider.h"

#include <QKeyEvent>
#include <QMap>

class KeyboardInputProvider : public InputProvider {
    Q_OBJECT

public:
    explicit KeyboardInputProvider(QObject* parent = nullptr);

private:
    QMap<int, InputAction::Type> m_keyMap;

    void setupKeyMap();
};
