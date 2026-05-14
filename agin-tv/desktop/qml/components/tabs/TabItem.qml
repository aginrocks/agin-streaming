import QtQuick
import QtQuick.Controls
import AginTV

Rectangle {
    id: root

    property alias label: text.text

    implicitWidth: text.implicitWidth + Theme.spacing.s(8)
    implicitHeight: text.implicitHeight + Theme.spacing.s(4)

    color: Theme.colors.primary
    radius: height / 2

    Text {
        id: text
        anchors.centerIn: parent
        color: Theme.colors.primaryForeground
    }
}
