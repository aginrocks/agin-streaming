pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Effects
import AginTV

Item {
    id: root
    width: 320
    height: width * 9 / 16

    scale: root.isSelected ? 1.05 : 1.0

    Behavior on scale {
        NumberAnimation {
            duration: Theme.animations.duration
            easing.type: Theme.animations.easing
        }
    }

    readonly property int radius: Theme.radius.xl
    readonly property int borderGap: Theme.spacing.s(1)
    readonly property int borderWidth: 2
    readonly property real shadowBlur: Theme.spacing.s(4)
    readonly property real shadowOffset: Theme.spacing.s(0.5)

    property string source
    property bool isSelected: false

    Rectangle {
        anchors.fill: parent
        radius: root.radius + root.borderGap
        color: "transparent"
        border.color: Theme.colors.selection
        border.width: root.borderWidth
        opacity: root.isSelected ? 1.0 : 0.0

        Behavior on opacity {
            NumberAnimation {
                duration: Theme.animations.duration
                easing.type: Theme.animations.easing
            }
        }
    }

    MultiEffect {
        anchors.fill: coverImage
        source: coverImage
        z: coverImage.z - 1

        blurEnabled: true
        blur: 1.0
        blurMax: 80

        scale: 1.1

        opacity: root.isSelected ? 0.7 : 0.0

        brightness: 0.1

        Behavior on opacity {
            NumberAnimation {
                duration: Theme.animations.duration
                easing.type: Theme.animations.easing
            }
        }
    }

    MultiEffect {
        anchors.fill: coverImage
        source: coverImage
        shadowEnabled: true
        shadowColor: root.isSelected ? Theme.colors.shadow : Theme.colors.shadowLight
        shadowBlur: 1.0
        shadowVerticalOffset: 2
        shadowHorizontalOffset: 0
        z: coverImage.z - 2

        Behavior on shadowColor {
            ColorAnimation {
                duration: Theme.animations.duration
                easing.type: Theme.animations.easing
            }
        }
    }

    RoundedImage {
        id: coverImage
        anchors.fill: parent
        anchors.margins: root.borderGap + root.borderWidth
        radius: root.radius
        source: root.source
    }
}
