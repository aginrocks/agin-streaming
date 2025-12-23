#pragma once
#include <qcontainerfwd.h>

#include <QObject>

#include "inputaction.h"
#include "inputprovider.h"
#include "providershistory.h"

class NavigationManager;

class InputDispatcher: public QObject {
    Q_OBJECT

public:
    static InputDispatcher* instance() {
        static InputDispatcher s_instance;
        return &s_instance;
    }

    void registerProvider(InputProvider* provider);
    void unregisterProvider(InputProvider* provider);

    void setNavigationManager(NavigationManager* manager);

    QList<InputProvider*> providers() const {
        return m_providers;
    }

    InputProvider::DeviceType activeDevice() const {
        return m_activeDevice;
    }

signals:
    void providerRegistered(InputProvider* provider);
    void providerUnregistered(InputProvider* provider);
    void actionReceived(const InputAction& action);
    void activeDeviceChanged(InputProvider::DeviceType device);

private slots:
    void onProviderAction(const InputAction& action);
    void onProviderActiveChanged(bool active);

private:
    InputDispatcher() = default;
    ~InputDispatcher() = default;

    InputDispatcher(const InputDispatcher&) = delete;
    InputDispatcher& operator=(const InputDispatcher&) = delete;

    QList<InputProvider*> m_providers;
    InputProvider::DeviceType m_activeDevice =
        InputProvider::DeviceType::Keyboard;
    ProvidersHistory m_providersHistory;
    NavigationManager* m_navigationManager = nullptr;

    void handleNavigationAction(const InputAction& action);

    void setActiveDevice(InputProvider::DeviceType device) {
        if (m_activeDevice == device)
            return;

        m_activeDevice = device;
        emit activeDeviceChanged(device);

        m_providersHistory.addProvider(device);

        qDebug() << "Active input device changed to" << device;
    }
};
