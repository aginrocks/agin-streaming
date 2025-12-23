#include "navigationengine.h"

#include <QtMath>
#include <limits>
#include <QDebug>

namespace {
// Convert a Direction enum to a readable string for logging.
const char* dirToString(Direction dir) {
    switch (dir) {
    case Direction::Up:
        return "Up";
    case Direction::Down:
        return "Down";
    case Direction::Left:
        return "Left";
    case Direction::Right:
        return "Right";
    }
    return "Unknown";
}
}

bool NavigationEngine::navigate(Direction dir) {
    qDebug() << "[Navigation] navigate()" << dirToString(dir);

    if (!m_currentFocus) {
        // No focus yet, focus first component in root
        auto components = collectComponents(m_root);
        if (!components.isEmpty()) {
            qDebug() << "[Navigation] initial focus ->" << components.first();
            return setFocus(components.first());
        }
        qDebug() << "[Navigation] No components available for initial focus";
        return false;
    }

    NavigationNode* next = findNext(m_currentFocus, dir);
    if (next && next != m_currentFocus) {
        qDebug() << "[Navigation] moving focus from" << m_currentFocus << "to" << next;
        return setFocus(next);
    }

    qDebug() << "[Navigation] staying on" << m_currentFocus;
    return false;
}

bool NavigationEngine::setFocus(NavigationNode* node) {
    if (!node || node->type() != NavigationNode::Type::Item) {
        qDebug() << "[Navigation] setFocus() rejected node" << node;
        return false;
    }

    m_currentFocus = node;
    qDebug() << "[Navigation] focus set to" << node << "bounds" << node->bounds();

    // Update lastFocused in containing scope
    NavigationScope* scope = findContainingScope(node);
    if (scope) {
        scope->setLastFocused(node);
    }

    return true;
}

NavigationNode*
NavigationEngine::findNext(NavigationNode* from, Direction dir) {
    // 1. Check manual override
    NavigationNode* manual = from->manualTarget(dir);
    if (manual) {
        qDebug() << "[Navigation] manual target" << manual << "for" << dirToString(dir);
        return manual;
    }

    // 2. Spatial search within containing scope
    NavigationScope* scope = findContainingScope(from);
    if (!scope)
        return nullptr;

    auto candidates = collectComponents(scope);
    candidates.removeOne(static_cast<NavigationComponent*>(from));
    candidates = filterByDirection(from->bounds(), candidates, dir);

    NavigationComponent* closest = findClosest(from->bounds(), candidates, dir);
    if (closest) {
        qDebug() << "[Navigation] closest candidate" << closest;
        return closest;
    }

    // 3. Check wrapping
    if (shouldWrap(scope, dir)) {
        qDebug() << "[Navigation] wrapping inside scope" << scope;
        return getWrapTarget(scope, dir);
    }

    // 4. Try escaping scope
    if (!scope->trapsFocus() && scope->parent()) {
        qDebug() << "[Navigation] escaping to parent scope";
        return findNext(scope, dir);
    }

    // 5. Nowhere to go
    qDebug() << "[Navigation] no valid target, staying put";
    return from;
}

NavigationScope* NavigationEngine::findContainingScope(NavigationNode* node) {
    NavigationNode* parent = node->parent();
    while (parent) {
        if (parent->type() == NavigationNode::Type::Scope) {
            return static_cast<NavigationScope*>(parent);
        }
        parent = parent->parent();
    }
    return m_root;
}

QList<NavigationComponent*>
NavigationEngine::collectComponents(NavigationScope* scope) {
    QList<NavigationComponent*> result;

    for (NavigationNode* child : scope->children()) {
        if (child->type() == NavigationNode::Type::Item) {
            result.append(static_cast<NavigationComponent*>(child));
        } else if (child->type() == NavigationNode::Type::Scope) {
            // Recursively collect from nested scopes
            result.append(
                collectComponents(static_cast<NavigationScope*>(child))
            );
        }
    }

    return result;
}

QList<NavigationComponent*> NavigationEngine::filterByDirection(
    const QRectF& fromBounds,
    const QList<NavigationComponent*>& candidates,
    Direction dir
) {
    QList<NavigationComponent*> result;
    QPointF fromCenter = fromBounds.center();

    for (NavigationComponent* candidate : candidates) {
        QPointF toCenter = candidate->bounds().center();

        switch (dir) {
            case Direction::Up:
                if (toCenter.y() < fromCenter.y())
                    result.append(candidate);
                break;
            case Direction::Down:
                if (toCenter.y() > fromCenter.y())
                    result.append(candidate);
                break;
            case Direction::Left:
                if (toCenter.x() < fromCenter.x())
                    result.append(candidate);
                break;
            case Direction::Right:
                if (toCenter.x() > fromCenter.x())
                    result.append(candidate);
                break;
        }
    }

    return result;
}

NavigationComponent* NavigationEngine::findClosest(
    const QRectF& fromBounds,
    const QList<NavigationComponent*>& candidates,
    Direction dir
) {
    if (candidates.isEmpty())
        return nullptr;

    NavigationComponent* closest = nullptr;
    qreal minDistance = std::numeric_limits<qreal>::max();
    QPointF fromCenter = fromBounds.center();

    for (NavigationComponent* candidate : candidates) {
        QPointF toCenter = candidate->bounds().center();
        qreal dx = toCenter.x() - fromCenter.x();
        qreal dy = toCenter.y() - fromCenter.y();
        qreal distance = qSqrt(dx * dx + dy * dy);

        if (distance < minDistance) {
            minDistance = distance;
            closest = candidate;
        }
    }

    return closest;
}

NavigationNode*
NavigationEngine::getWrapTarget(NavigationScope* scope, Direction dir) {
    auto components = collectComponents(scope);
    if (components.isEmpty())
        return nullptr;

    // For wrapping, find first/last component based on direction
    NavigationComponent* target = components.first();

    switch (dir) {
        case Direction::Up:
        case Direction::Down:
            // Find bottom-most or top-most
            for (auto* comp : components) {
                if ((dir == Direction::Down
                     && comp->bounds().top() < target->bounds().top())
                    || (dir == Direction::Up
                        && comp->bounds().bottom()
                            > target->bounds().bottom())) {
                    target = comp;
                }
            }
            break;
        case Direction::Left:
        case Direction::Right:
            // Find right-most or left-most
            for (auto* comp : components) {
                if ((dir == Direction::Right
                     && comp->bounds().left() < target->bounds().left())
                    || (dir == Direction::Left
                        && comp->bounds().right() > target->bounds().right())) {
                    target = comp;
                }
            }
            break;
    }

    return target;
}

bool NavigationEngine::shouldWrap(NavigationScope* scope, Direction dir) {
    switch (dir) {
        case Direction::Up:
        case Direction::Down:
            return scope->wrapVertical();
        case Direction::Left:
        case Direction::Right:
            return scope->wrapHorizontal();
    }
    return false;
}
