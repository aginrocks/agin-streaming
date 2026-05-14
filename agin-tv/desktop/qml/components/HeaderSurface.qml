import QtQuick
import QtQuick.Layouts
import AginTV

Rectangle {
    id: root

    default property alias content: row.data
    property real padding: Theme.spacing.s(1.5)

    color: Theme.colors.transparentSurface
    radius: height / 2

    implicitWidth: row.implicitWidth + padding * 2
    implicitHeight: row.implicitHeight + padding * 2

    RowLayout {
        id: row
        anchors.fill: parent
        anchors.margins: root.padding
    }
}
