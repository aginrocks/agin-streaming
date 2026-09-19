#include "navigationmanager.h"

#include <QQuickItem>
#include <QTimer>

NavigationManager::NavigationManager(QObject* parent) : QObject(parent) {}

NavigationManager::~NavigationManager() {
    delete m_engine;
    delete m_rootScope;
}

void NavigationManager::setRootItem(QQuickItem* item) {
    if (m_rootItem == item)
        return;

    m_rootItem = item;
    rebuildTree();

    emit rootItemChanged();
}

void NavigationManager::navigate(int direction) {
    if (!m_engine)
        return;

    bool moved = m_engine->navigate(static_cast<Direction>(direction));

    if (moved) {
        NavigationNode* focused = m_engine->currentFocus();

        // Update hasFocus on all attached properties
        for (auto it = m_attachedProperties.cbegin();
             it != m_attachedProperties.cend();
             ++it) {
            QQuickItem* item = it.key();
            NavigationNode* node = m_itemNodes.value(item, nullptr);
            bool isFocused = (node == focused);
            it.value()->setHasFocus(isFocused);
        }
    }
}

void NavigationManager::rebuildTree() {
    if (!m_rootItem)
        return;

    // Clean up old tree
    delete m_engine;
    delete m_rootScope;
    m_attachedProperties.clear();
    m_itemNodes.clear();

    // Build new tree
    m_rootScope = buildScopeTree(m_rootItem);
    m_engine = new NavigationEngine(m_rootScope);

    // Set initial focus to first component and sync hasFocus state
    navigate(static_cast<int>(Direction::Down));
}

NavigationScope* NavigationManager::buildScopeTree(QQuickItem* item) {
    QRectF bounds(item->x(), item->y(), item->width(), item->height());
    auto scope = new NavigationScope(bounds);

    // Check if item has Navigation attached property
    auto attached = qobject_cast<Navigable*>(
        qmlAttachedPropertiesObject<Navigable>(item, false)
    );

    if (attached) {
        m_attachedProperties[item] = attached;
        m_itemNodes[item] = scope;

        // Connect to geometry changes
        connect(
            item,
            &QQuickItem::xChanged,
            this,
            &NavigationManager::onItemGeometryChanged
        );
        connect(
            item,
            &QQuickItem::yChanged,
            this,
            &NavigationManager::onItemGeometryChanged
        );
        connect(
            item,
            &QQuickItem::widthChanged,
            this,
            &NavigationManager::onItemGeometryChanged
        );
        connect(
            item,
            &QQuickItem::heightChanged,
            this,
            &NavigationManager::onItemGeometryChanged
        );
    }

    traverseItems(item, scope);

    return scope;
}

void NavigationManager::traverseItems(
    QQuickItem* item,
    NavigationScope* parentScope
) {
    for (QQuickItem* child : item->childItems()) {
        if (!child->isVisible())
            continue;

        Navigable* attached = qobject_cast<Navigable*>(
            qmlAttachedPropertiesObject<Navigable>(child, false)
        );

        if (!attached) {
            // No attached property, recurse into children
            traverseItems(child, parentScope);
            continue;
        }

        if (attached->isScope()) {
            // Create nested scope
            NavigationScope* childScope = buildScopeTree(child);
            parentScope->addChild(childScope);
        } else if (attached->canNavigate()) {
            // Create component
            QRectF bounds = QRectF(
                child->mapToItem(m_rootItem, QPointF(0, 0)),
                QSizeF(child->width(), child->height())
            );

            auto component = new NavigationComponent(bounds);
            parentScope->addChild(component);

            m_attachedProperties[child] = attached;
            m_itemNodes[child] = component;

            // Connect to geometry changes
            connect(
                child,
                &QQuickItem::xChanged,
                this,
                &NavigationManager::onItemGeometryChanged
            );
            connect(
                child,
                &QQuickItem::yChanged,
                this,
                &NavigationManager::onItemGeometryChanged
            );
            connect(
                child,
                &QQuickItem::widthChanged,
                this,
                &NavigationManager::onItemGeometryChanged
            );
            connect(
                child,
                &QQuickItem::heightChanged,
                this,
                &NavigationManager::onItemGeometryChanged
            );
        }

        // Don't recurse into items with attached properties - they define boundaries
    }
}

void NavigationManager::onItemGeometryChanged() {
    // Debounce rebuilds
    if (m_rebuildScheduled)
        return;

    m_rebuildScheduled = true;
    QTimer::singleShot(0, this, [this]() {
        m_rebuildScheduled = false;

        // Update bounds instead of full rebuild for performance
        for (auto it = m_itemNodes.begin(); it != m_itemNodes.end(); ++it) {
            QQuickItem* item = it.key();
            NavigationNode* node = it.value();

            if (!node) {
                continue;
            }

            QRectF bounds;
            if (node->type() == NavigationNode::Type::Item) {
                bounds = QRectF(
                    item->mapToItem(m_rootItem, QPointF(0, 0)),
                    QSizeF(item->width(), item->height())
                );
            } else {
                bounds =
                    QRectF(item->x(), item->y(), item->width(), item->height());
            }
            node->setBounds(bounds);
        }
    });
}
