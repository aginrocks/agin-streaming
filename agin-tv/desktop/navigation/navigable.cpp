#include "navigable.h"

Navigable::Navigable(QObject *parent)
    : QObject(parent)
    , m_enabled(false)
    , m_selected(false)
{}
