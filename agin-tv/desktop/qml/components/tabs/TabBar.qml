pragma ComponentBehavior: Bound
import QtQuick
import AginTV

Item {
    id: root

    implicitWidth: surface.implicitWidth
    implicitHeight: surface.implicitHeight

    property var tabs: ["Home", "Live", "Library"]

    property int activeTab: 0

    // Indicator that floats below the tabs
    Rectangle {
        id: indicator

        // Used to prevent animations during initial render
        property bool animate: false
        property var target: null
        property point mapped: target ? target.mapToItem(parent, 0, 0) : Qt.point(0, 0)

        function updateTarget() {
            target = repeater.itemAt(root.activeTab);
        }

        Component.onCompleted: {
            updateTarget();
            Qt.callLater(() => {
                animate = true;
            });
        }

        // Watch tab bar items updates
        Connections {
            target: repeater
            function onItemAdded(index, item) {
                indicator.updateTarget();
            }
        }

        // Watch active tab updates
        Connections {
            target: root
            function onActiveTabChanged() {
                indicator.updateTarget();
            }
        }

        x: mapped.x
        y: mapped.y
        width: target ? target.width : 0
        height: target ? target.height : 0

        Behavior on x {
            enabled: indicator.animate
            NumberAnimation {
                duration: Theme.animations.duration
                easing: Theme.animations.easing
            }
        }
        Behavior on width {
            enabled: indicator.animate
            NumberAnimation {
                duration: Theme.animations.duration
            }
        }

        color: Theme.colors.primary
        radius: height / 2
    }

    HeaderSurface {
        id: surface

        Repeater {
            id: repeater
            model: root.tabs

            TabItem {
                required property string modelData
                required property int index
                label: modelData
                active: root.activeTab === index

                MouseArea {
                    anchors.fill: parent
                    onClicked: root.activeTab = parent.index
                }
            }
        }

        // TabItem {
        //     active: root.activeTab == 0
        //     label: "Home"
        // }

        // TabItem {
        //     active: root.activeTab == 1
        //     label: "Live"
        // }

        // TabItem {
        //     active: root.activeTab == 2
        //     label: "Library"
        // }
    }
}
