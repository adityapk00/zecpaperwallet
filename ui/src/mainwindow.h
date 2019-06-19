#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include "precompiled.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    void populateWallets();

    // The current JSON of the wallets.
    QString currentWallets; 

    Ui::MainWindow *ui;
    QIntValidator  *intValidator; 
};

#endif // MAINWINDOW_H
