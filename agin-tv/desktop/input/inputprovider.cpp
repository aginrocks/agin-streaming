#include "inputprovider.h"

void InputProvider::emitAction(InputAction::Type type,
                               InputAction::State state,
                               const QVariant &data)
{
    InputAction action(type, state, data);
    emit actionTriggered(action);
}
