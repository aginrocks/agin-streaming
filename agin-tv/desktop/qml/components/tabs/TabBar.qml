import QtQuick
import QtQuick.Controls
import AginTV

Item {
    id: root

    implicitWidth: surface.implicitWidth
    implicitHeight: surface.implicitHeight

    property int activeTab: 0

    HeaderSurface {
        id: surface

        TabItem {
            active: root.activeTab == 0
            label: "Home"
        }

        TabItem {
            active: root.activeTab == 1
            label: "Live"
        }

        Button {
            text: "toggle"
            onClicked: {
                if (root.activeTab == 0) {
                    root.activeTab = 1;
                } else {
                    root.activeTab = 0;
                }
            }
        }
    }
}
