# Agin TV Desktop

This QT project includes Agin TV for desktops and smart TVs (linux-based only)

It's built using QT and QML.

## Architecture

### Navigation

Every element in the UI is navigable using directional keys (up, down, left, right). There are many input methods supported, each implemented as a subclass of [`InputProvider`](./input/inputprovider.h).

Inputs are collected in the [`InputDispatcher`](./input/inputdispatcher.h), which forwards them to the [`NavigationEngine`](./navigation/navigationengine.h).

Next, the navigation engine uses a tree structure to represent the UI elements. Each element is represented as a [`NavigationNode`](./navigation/navigationnode.h), which contains information about its position, size, and relationships to other nodes (parent, children, siblings).
