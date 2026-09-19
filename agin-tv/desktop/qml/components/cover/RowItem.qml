pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Layouts
import AginTV

ColumnLayout {
    id: root

    property string label

    Cover {
        source: "https://image.tmdb.org/t/p/w1280/rAiYTfKGqDCRIIqo664sY9XZIvQ.jpg"
    }

    Label {
        text: root.label
    }
}
