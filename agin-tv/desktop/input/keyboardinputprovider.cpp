#include "keyboardinputprovider.h"

KeyboardInputProvider::KeyboardInputProvider(QObject* parent) :
    InputProvider(DeviceType::Keyboard, parent) {
    setupKeyMap();
}

void KeyboardInputProvider::setupKeyMap() {
    m_keyMap = {
        {Qt::Key_Up, InputAction::NavigateUp},
        {Qt::Key_Down, InputAction::NavigateDown},
        {Qt::Key_Left, InputAction::NavigateLeft},
        {Qt::Key_Right, InputAction::NavigateRight},
        {Qt::Key_Enter, InputAction::Select},
        {Qt::Key_Escape, InputAction::Back},
        {Qt::Key_Menu, InputAction::Menu},
    };
}

bool KeyboardInputProvider::handleKeyEvent(QKeyEvent* event) {
    auto it = m_keyMap.constFind(event->key());
    if (it == m_keyMap.constEnd())
        return false;

    InputAction::State actionState;
    if (event->type() == QEvent::KeyPress) {
        actionState =
            event->isAutoRepeat() ? InputAction::Repeat : InputAction::Pressed;
    } else if (event->type() == QEvent::KeyRelease) {
        actionState = InputAction::Released;
    } else {
        return false;
    }

    qDebug() << "Emitting action from keyboard:" << *it
             << (actionState == InputAction::Pressed        ? "Pressed"
                     : actionState == InputAction::Released ? "Released"
                                                            : "Repeat");

    emitAction(*it, actionState);
    return true;
}