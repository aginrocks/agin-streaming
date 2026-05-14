import QtQuick
import QtQuick.Layouts
import AginTV

Item {
    id: root
    default property alias content: row.data

    implicitWidth: row.implicitWidth + row.anchors.margins * 2
    implicitHeight: row.implicitHeight + row.anchors.margins * 2

    Rectangle {
        anchors.fill: parent
        radius: 999999
        color: Theme.colors.transparentSurface
    }

    RowLayout {
        id: row
        anchors.fill: parent
        anchors.margins: Theme.spacing.s(1.5)
    }
}
