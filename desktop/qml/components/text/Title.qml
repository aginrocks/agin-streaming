import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Title.Size.Large

    typeStyle: Theme.typography.title[size] || Theme.typography.title[Title.Size.Large]
}
