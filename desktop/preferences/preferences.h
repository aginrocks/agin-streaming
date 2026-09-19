#pragma once

#include <qtmetamacros.h>

#include <QObject>
#include <QSettings>
#include <QString>

namespace User {
class Preferences: public QObject {
public:
    explicit Preferences(QObject* parent = nullptr) :
        QObject(parent),
        settings("User", "Preferences") {}
    Q_OBJECT
    Q_PROPERTY(
        QString tmdbTOKEN READ tmdbTOKEN WRITE setTmdbTOKEN NOTIFY
            tmdbTOKENchanged
    )

    QString tmdbTOKEN() const {
        return settings.value("TOKEN").toString();
    }

    void setTmdbTOKEN(const QString& token) {
        if (settings.value("TOKEN").value<QString>() == token)
            return;
        settings.setValue("TOKEN", token);
        emit tmdbTOKENchanged();
    }

signals:
    void tmdbTOKENchanged();

private:
    QSettings settings;
};

} // namespace User
