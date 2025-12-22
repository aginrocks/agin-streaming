#include "navigable.h"

Navigable::Navigable(QObject* parent) :
    QObject(parent),
    m_canNavigate(false),
    m_hasFocus(false) {}
