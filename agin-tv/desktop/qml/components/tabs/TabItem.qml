import QtQuick
import AginTV

Item {
    id: root

    property alias label: text.text

    property bool active: false
    property bool barFocused: false

    implicitWidth: textStub.width + Theme.spacing.s(12)
    implicitHeight: textStub.height + Theme.spacing.s(6)

    // Stub needed in order to have fixed width when changing font weight
    TextMetrics {
        id: textStub

        text: root.label
        font.family: text.font.family
        font.pixelSize: text.font.pixelSize
        font.letterSpacing: text.font.letterSpacing
        font.weight: Font.DemiBold
        font.variableAxes: ({
                "wght": 600
            })
    }

    Title {
        id: text
        anchors.centerIn: parent

        property real textWeight: root.active ? 600 : 500
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

        font.weight: root.active ? Font.DemiBold : Font.Medium
        color: textColor
        font.variableAxes: ({
                "wght": textWeight
            })
    }
}
