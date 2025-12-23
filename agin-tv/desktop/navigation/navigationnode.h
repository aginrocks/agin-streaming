#pragma once
#include <qvectornd.h>

#include <QHash>
#include <QList>
#include <QRectF>

enum class Direction { Up, Down, Left, Right };

class NavigationNode {
public:
    enum class Type {
        Item,
        Scope,
    };

    NavigationNode(
        Type type,
        const QRectF& bounds,
        NavigationNode* parent = nullptr
    ) :
        m_type(type),
        m_bounds(bounds),
        m_parent(parent) {}

    virtual ~NavigationNode() = default;

    Type type() const {
        return m_type;
    }

    QRectF bounds() const {
        return m_bounds;
    }

    void setBounds(const QRectF& bounds) {
        m_bounds = bounds;
    }

    NavigationNode* parent() const {
        return m_parent;
    }

    void setParent(NavigationNode* parent) {
        m_parent = parent;
    }

    // Manual navigation overrides
    void setManualTarget(Direction dir, NavigationNode* target) {
        m_manualTargets[dir] = target;
    }

    NavigationNode* manualTarget(Direction dir) const {
        return m_manualTargets.value(dir, nullptr);
    }

protected:
    Type m_type;
    QRectF m_bounds;
    NavigationNode* m_parent;
    QHash<Direction, NavigationNode*> m_manualTargets;
};

class NavigationComponent: public NavigationNode {
public:
    NavigationComponent(const QRectF& bounds) :
        NavigationNode(Type::Item, bounds) {};
};

class NavigationScope: public NavigationNode {
public:
    NavigationScope(const QRectF& bounds) :
        NavigationNode(Type::Scope, bounds),
        m_trapsFocus(false),
        m_wrapHorizontal(false),
        m_wrapVertical(false),
        m_lastFocused(nullptr) {}

    ~NavigationScope() {
        qDeleteAll(m_children);
    }

    const QList<NavigationNode*>& children() const {
        return m_children;
    }

    void addChild(NavigationNode* child) {
        m_children.append(child);
        child->setParent(this);
    }

    void removeChild(NavigationNode* child) {
        m_children.removeOne(child);
        if (m_lastFocused == child) {
            m_lastFocused = nullptr;
        }
    }

    bool trapsFocus() const {
        return m_trapsFocus;
    }

    void setTrapsFocus(bool trap) {
        m_trapsFocus = trap;
    }

    bool wrapHorizontal() const {
        return m_wrapHorizontal;
    }

    void setWrapHorizontal(bool wrap) {
        m_wrapHorizontal = wrap;
    }

    bool wrapVertical() const {
        return m_wrapVertical;
    }

    void setWrapVertical(bool wrap) {
        m_wrapVertical = wrap;
    }

    NavigationNode* lastFocused() const {
        return m_lastFocused;
    }

    void setLastFocused(NavigationNode* node) {
        m_lastFocused = node;
    }

private:
    QList<NavigationNode*> m_children;
    bool m_trapsFocus;
    bool m_wrapHorizontal;
    bool m_wrapVertical;
    NavigationNode* m_lastFocused;
};