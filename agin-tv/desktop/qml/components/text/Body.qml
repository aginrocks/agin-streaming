import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Body.Size.Large

    typeStyle: Theme.typography.body[size] || Theme.typography.body[Body.Size.Large]
}
