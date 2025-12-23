#include "navigable.h"

#include "inputdispatcher.h"

Navigable::Navigable(QObject* parent) :
    QObject(parent),
    m_canNavigate(false),
    m_hasFocus(false) {
    auto* dispatcher = InputDispatcher::instance();
}
