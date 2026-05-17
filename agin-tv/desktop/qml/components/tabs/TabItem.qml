import QtQuick
import AginTV

Rectangle {
    id: root

    property alias label: text.text

    property bool active: false

    implicitWidth: textStub.width + Theme.spacing.s(8)
    implicitHeight: textStub.height + Theme.spacing.s(4)

    color: active ? Theme.colors.primary : "transparent"
    radius: height / 2

    // Stub needed in order to have fixed width when changing font weight
    TextMetrics {
        id: textStub

        text: root.label
        font.family: text.font.family
        font.pixelSize: text.font.pixelSize
        font.letterSpacing: text.font.letterSpacing
        font.variableAxes: ({
                "wght": 400
            })
    }

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
