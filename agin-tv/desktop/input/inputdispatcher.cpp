#include "inputdispatcher.h"

#include <qobject.h>

#include "inputprovider.h"

void InputDispatcher::registerProvider(InputProvider* provider) {
    if (m_providers.contains(provider))
        return;

    m_providers.append(provider);
    connect(
        provider,
        &InputProvider::actionTriggered,
        this,
        &InputDispatcher::onProviderAction
    );
    emit providerRegistered(provider);
}

void InputDispatcher::unregisterProvider(InputProvider* provider) {
    if (!m_providers.contains(provider))
        return;

    m_providers.removeAll(provider);
    disconnect(
        provider,
        &InputProvider::actionTriggered,
        this,
        &InputDispatcher::onProviderAction
    );
    emit providerUnregistered(provider);
}

void InputDispatcher::onProviderAction(const InputAction& action) {
    emit actionReceived(action);
}