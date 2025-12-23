#pragma once
#include "navigationnode.h"

class NavigationEngine {
public:
    NavigationEngine(NavigationScope* root) :
        m_root(root),
        m_currentFocus(nullptr) {}

    NavigationNode* currentFocus() const {
        return m_currentFocus;
    }

    bool navigate(Direction dir);
    bool setFocus(NavigationNode* node);

private:
    NavigationScope* m_root;
    NavigationNode* m_currentFocus;

    NavigationNode* findNext(NavigationNode* from, Direction dir);
    NavigationScope* findContainingScope(NavigationNode* node);

    QList<NavigationComponent*> collectComponents(NavigationScope* scope);
    QList<NavigationComponent*> filterByDirection(
        const QRectF& fromBounds,
        const QList<NavigationComponent*>& candidates,
        Direction dir
    );
    NavigationComponent* findClosest(
        const QRectF& fromBounds,
        const QList<NavigationComponent*>& candidates,
        Direction dir
    );

    NavigationNode* getWrapTarget(NavigationScope* scope, Direction dir);
    bool shouldWrap(NavigationScope* scope, Direction dir);
};
