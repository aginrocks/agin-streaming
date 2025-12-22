#include "keyboardinputprovider.h"

KeyboardInputProvider::KeyboardInputProvider(QObject* parent)
    : InputProvider(DeviceType::Keyboard, parent) {
    setupKeyMap();
}

void KeyboardInputProvider::setupKeyMap() {
    m_keyMap = {
        {Qt::Key_Up, InputAction::NavigateUp},     {Qt::Key_Down, InputAction::NavigateDown},
        {Qt::Key_Left, InputAction::NavigateLeft}, {Qt::Key_Right, InputAction::NavigateRight},
        {Qt::Key_Enter, InputAction::Select},      {Qt::Key_Escape, InputAction::Back}};
}
