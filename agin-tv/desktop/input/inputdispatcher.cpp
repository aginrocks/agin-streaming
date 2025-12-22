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
    connect(
        provider,
        &InputProvider::activeChanged,
        this,
        &InputDispatcher::onProviderActiveChanged
    );
    emit providerRegistered(provider);
}

void InputDispatcher::unregisterProvider(InputProvider* provider) {
    if (!m_providers.contains(provider))
        return;

    m_providers.removeAll(provider);
    m_providersHistory.removeProvider(provider);
    disconnect(
        provider,
        &InputProvider::actionTriggered,
        this,
        &InputDispatcher::onProviderAction
    );
    disconnect(
        provider,
        &InputProvider::activeChanged,
        this,
        &InputDispatcher::onProviderActiveChanged
    );
    emit providerUnregistered(provider);
}

void InputDispatcher::onProviderAction(const InputAction& action) {
    emit actionReceived(action);
    auto* provider = qobject_cast<InputProvider*>(sender());
    if (provider) {
        setActiveDevice(provider->deviceType());
    }
}

void InputDispatcher::onProviderActiveChanged(bool active) {
    qDebug() << "Provider active changed:" << active;
    auto* provider = qobject_cast<InputProvider*>(sender());
    if (provider) {
        if (active) {
            setActiveDevice(provider->deviceType());
        } else {
            auto previousDevice = m_providersHistory.previousDevice();
            setActiveDevice(previousDevice);
        }
    }
}