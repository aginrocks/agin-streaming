import QtQuick
import AginTV

Rectangle {
    id: root

    property alias label: text.text

    property bool active: false

    implicitWidth: text.implicitWidth + Theme.spacing.s(8)
    implicitHeight: text.implicitHeight + Theme.spacing.s(4)

    color: active ? Theme.colors.primary : "transparent"
    radius: height / 2

    Title {
        id: text
        anchors.centerIn: parent

        property real textWeight: root.active ? 550 : 400
        property color textColor: root.active ? Theme.colors.primaryForeground : Theme.colors.primary

        Behavior on textWeight {
            NumberAnimation {
                duration: Theme.animations.duration
            }
        }

        Behavior on textColor {
            ColorAnimation {
                duration: Theme.animations.duration
            }
        }

        color: textColor
        font.variableAxes: ({
                "wght": textWeight
            })
    }
}
