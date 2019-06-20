#include "mainwindow.h"
#include "version.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

    QCoreApplication::setOrganizationDomain("zecwallet.co");
    QCoreApplication::setOrganizationName("zecpaperwallet");

    #ifdef Q_OS_LINUX
        QFontDatabase::addApplicationFont(":/fonts/res/Ubuntu-R.ttf");
        qApp->setFont(QFont("Ubuntu", 11, QFont::Normal, false));
    #endif

    QApplication a(argc, argv);
    MainWindow w;

    w.setWindowTitle(QString("zecpaperwallet ") + APP_VERSION);

    w.show();

    return a.exec();
}
