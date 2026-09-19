import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Headline.Size.Large

    typeStyle: Theme.typography.headline[size] || Theme.typography.headline[Headline.Size.Large]
}
