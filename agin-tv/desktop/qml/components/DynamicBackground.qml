import QtQuick

Item {
    id: root
    width: parent.width
    height: parent.height

    property alias activeSource: backgroundImage.source
    property color baseColor

    Image {
        id: backgroundImage
        anchors.fill: parent
        anchors {
            bottomMargin: parent.height * 0.2
        }
        source: root.activeSource
        fillMode: Image.PreserveAspectCrop
        smooth: true
    }

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            orientation: Gradient.Vertical
            GradientStop {
                position: 0.0
                color: Qt.alpha(root.baseColor, 0.45)
            }
            GradientStop {
                position: 0.5
                color: Qt.alpha(root.baseColor, 0.6)
            }
            GradientStop {
                position: 0.8
                color: root.baseColor
            }
            GradientStop {
                position: 1
                color: root.baseColor
            }
        }
    }
}
