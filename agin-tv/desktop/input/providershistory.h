#pragma once

#include <QObject>

#include "inputprovider.h"

class ProvidersHistory {
public:
    ProvidersHistory() = default;
    ~ProvidersHistory() = default;

    void addProvider(InputProvider::DeviceType device) {
        if (m_providers.contains(device))
            m_providers.removeAll(device);
        m_providers.prepend(device);
    }

    void removeProvider(InputProvider* provider) {
        if (!provider)
            return;

        m_providers.removeAll(provider->deviceType());
    }

    // Removes current device from history and returns a previously active device or Keyboard if no history exists
    InputProvider::DeviceType previousDevice() {
        if (m_providers.isEmpty())
            return InputProvider::DeviceType::
                Keyboard; // Keyboard is the default input device

        m_providers.removeFirst();

        if (m_providers.isEmpty())
            return InputProvider::DeviceType::
                Keyboard; // Keyboard is the default input device

        return m_providers.first();
    }

private:
    QList<InputProvider::DeviceType> m_providers;
};