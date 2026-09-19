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
    qDebug() << "Keyboard event:" << event->key() << event->nativeVirtualKey()
             << event->type();

    auto it = m_keyMap.constFind(event->key());
    if (it == m_keyMap.constEnd())
        return false;

    InputAction::State actionState;
    if (event->type() == QEvent::KeyPress) {
        actionState =
            event->isAutoRepeat() ? InputAction::Repeat : InputAction::Pressed;
    } else if (event->type() == QEvent::KeyRelease) {
        if (event->isAutoRepeat())
            return true; // Ignore release that accompanies repeats
        actionState = InputAction::Released;
    } else {
        return false;
    }

    emitAction(*it, actionState);
    return true;
}
