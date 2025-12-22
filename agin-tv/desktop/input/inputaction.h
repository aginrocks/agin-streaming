#pragma once
#include <QObject>
#include <QVariant>

class InputAction {
    Q_GADGET
public:
    enum Type {
        // Navigation
        NavigateUp,
        NavigateDown,
        NavigateLeft,
        NavigateRight,

        // TabView Navigation
        NextTab,
        PrevTab,

        // Item Actions
        Select,  // Enter
        Back,    // Escaoe
        Options, // A context menu with more options
        Action1, // Contextual action 1 (X button)
        Action2, // Contextual action 2 (Y button)

        Menu, // Main Menu
    };
    Q_ENUM(Type);

    enum State { Pressed, Released, Repeat };
    Q_ENUM(State);

    InputAction() = default;
    InputAction(Type type, State state, const QVariant& data = QVariant())
        : m_type(type), m_state(state), m_data(data) {}

    Type type() const { return m_type; }
    State state() const { return m_state; }
    QVariant data() const { return m_data; }

    bool isPressed() const { return m_state == Pressed; }
    bool isReleased() const { return m_state == Released; }
    bool isRepeat() const { return m_state == Repeat; }

private:
    Type m_type = Select;
    State m_state = Pressed;
    QVariant m_data;
};

Q_DECLARE_METATYPE(InputAction)
Q_DECLARE_METATYPE(InputAction::Type)
