import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Display.Size.Large

    typeStyle: Theme.typography.display[size] || Theme.typography.display[Display.Size.Large]
}
