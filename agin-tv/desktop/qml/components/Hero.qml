import QtQuick

Item {
    id: root
    width: parent.width
    height: parent.height

    property alias activeSource: heroImage.source

    Image {
        id: heroImage
        anchors.fill: parent
        source: root.activeSource
        fillMode: Image.PreserveAspectCrop
        smooth: true
    }
}
