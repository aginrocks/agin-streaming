pragma ComponentBehavior: Bound
import QtQuick
import AginTV

Item {
    id: root

    implicitWidth: surface.implicitWidth
    implicitHeight: surface.implicitHeight

    readonly property var tabs: ["Home", "Live", "Library"]

    property int activeTab: 0

    Rectangle {
        property var target: repeater.itemAt(root.activeTab)
        property point mapped: target ? target.mapToItem(parent, 0, 0) : Qt.point(0, 0)

        x: mapped.x
        y: mapped.y
        width: target ? target.width : 0
        height: target ? target.height : 0

        Behavior on x {
            NumberAnimation {
                duration: Theme.animations.duration
                easing: Theme.animations.easing
            }
        }
        Behavior on width {
            NumberAnimation {
                duration: Theme.animations.duration
            }
        }

        color: Theme.colors.primary
        radius: height / 2
        anchors.fill: repeater.itemAt(root.activeTab)
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
