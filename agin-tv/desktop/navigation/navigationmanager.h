#pragma once
#include <QHash>
#include <QQuickItem>

#include "navigable.h"
#include "navigationengine.h"

class NavigationManager: public QObject {
    Q_OBJECT
    Q_PROPERTY(
        QQuickItem* rootItem READ rootItem WRITE setRootItem NOTIFY
            rootItemChanged
    )

public:
    explicit NavigationManager(QObject* parent = nullptr);
    ~NavigationManager();

    QQuickItem* rootItem() const {
        return m_rootItem;
    }

    void setRootItem(QQuickItem* item);

    Q_INVOKABLE void navigate(int direction);

signals:
    void rootItemChanged();

private slots:
    void onItemGeometryChanged();
    void rebuildTree();

private:
    void registerItem(QQuickItem* item);
    void unregisterItem(QQuickItem* item);
    void updateItemBounds(QQuickItem* item);

    NavigationScope* buildScopeTree(QQuickItem* item);
    void traverseItems(QQuickItem* item, NavigationScope* parentScope);

    QQuickItem* m_rootItem = nullptr;
    NavigationEngine* m_engine = nullptr;
    NavigationScope* m_rootScope = nullptr;

    QHash<QQuickItem*, Navigable*> m_attachedProperties;
    QHash<QQuickItem*, NavigationNode*> m_itemNodes;
    bool m_rebuildScheduled = false;
};
