#ifndef ELLIDEDLABEL_H
#define ELLIDEDLABEL_H

#include "precompiled.h"

class EllidedLabel : public QFrame
{
    Q_OBJECT
    Q_PROPERTY(QString text READ text WRITE setText)
    Q_PROPERTY(bool isElided READ isElided)

public:
    explicit EllidedLabel(QWidget *parent = nullptr, const QString &text = "");

    void setText(const QString &text);
    const QString & text() const { return content; }
    bool isElided() const { return elided; }

protected:
    void paintEvent(QPaintEvent *event) Q_DECL_OVERRIDE;

signals:
    void elisionChanged(bool elided);

private:
    bool elided;
    QString content;
};

#endif // ELLIDEDLABEL_H
