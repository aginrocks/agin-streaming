import QtQuick
import AginTV

Text {
    id: root
    property var typeStyle: Theme.typography.body[0]

    lineHeightMode: Text.FixedHeight
    lineHeight: typeStyle.lineHeight
    font.pixelSize: typeStyle.pixelSize
    font.weight: typeStyle.weight
    font.letterSpacing: typeStyle.letterSpacing
    font.variableAxes: ({ "ROND": 0, "wght": typeStyle.weight })

    color: Theme.colors.foreground
    font.family: Theme.typography.primaryFont
}
