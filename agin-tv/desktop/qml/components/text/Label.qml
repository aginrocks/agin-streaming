import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Label.Size.Large

    typeStyle: Theme.typography.label[size] || Theme.typography.label[Label.Size.Large]
}
