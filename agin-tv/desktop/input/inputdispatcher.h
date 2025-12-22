#pragma once
#include <QObject>

#include "inputaction.h"
#include "inputprovider.h"

class InputDispatcher: public QObject {
    Q_OBJECT

public:
    static InputDispatcher* instance() {
        static InputDispatcher s_instance;
        return &s_instance;
    }

    void registerProvider(InputProvider* provider);
    void unregisterProvider(InputProvider* provider);

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

private slots:
    void onProviderAction(const InputAction& action);

private:
    InputDispatcher() = default;
    ~InputDispatcher() = default;

    InputDispatcher(const InputDispatcher&) = delete;
    InputDispatcher& operator=(const InputDispatcher&) = delete;

    QList<InputProvider*> m_providers;
    InputProvider::DeviceType m_activeDevice =
        InputProvider::DeviceType::Keyboard;
};
