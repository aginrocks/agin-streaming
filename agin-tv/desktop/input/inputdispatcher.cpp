#include "inputdispatcher.h"

#include <qdebug.h>
#include <qobject.h>

#include "inputprovider.h"
#include "navigation/navigationmanager.h"

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

void InputDispatcher::setNavigationManager(NavigationManager* manager) {
    m_navigationManager = manager;
}

void InputDispatcher::onProviderAction(const InputAction& action) {
    qDebug() << "Action received:" << action.type() << action.state();

    emit actionReceived(action);
    auto* provider = qobject_cast<InputProvider*>(sender());
    if (provider) {
        setActiveDevice(provider->deviceType());
    }

    handleNavigationAction(action);
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

void InputDispatcher::handleNavigationAction(const InputAction& action) {
    if (!m_navigationManager)
        return;

    if (action.isReleased())
        return;

    Direction direction;
    switch (action.type()) {
        case InputAction::NavigateUp:
            direction = Direction::Up;
            break;
        case InputAction::NavigateDown:
            direction = Direction::Down;
            break;
        case InputAction::NavigateLeft:
            direction = Direction::Left;
            break;
        case InputAction::NavigateRight:
            direction = Direction::Right;
            break;
        default:
            return;
    }

    m_navigationManager->navigate(static_cast<int>(direction));
}