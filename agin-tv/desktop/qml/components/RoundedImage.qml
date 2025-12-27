pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Effects

Item {
    id: root

    property alias source: image.source
    property real radius: 20
    property alias fillMode: image.fillMode

    Rectangle {
        anchors.fill: parent
        radius: root.radius
        color: "transparent"
        clip: true

        Image {
            id: image
            anchors.fill: parent
            source: root.source
            smooth: true
            fillMode: Image.PreserveAspectCrop

            layer.enabled: true
            layer.effect: MultiEffect {
                maskEnabled: true
                maskThresholdMin: 0.5
                maskSpreadAtMin: 1.0
                maskSource: ShaderEffectSource {
                    sourceItem: Rectangle {
                        width: image.width
                        height: image.height
                        radius: root.radius
                    }
                }
            }
        }
    }
}
